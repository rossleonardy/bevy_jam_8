use crate::bullets::{BulletHit, BulletOwner, BulletTarget, Hitbox, RetargetBullet};
use crate::inputs;
use crate::inputs::AccumulatedLR;
use crate::physics::*;
use crate::player_assets::{PlayerAssets, PlayerAssetsPlugin};
use bevy::prelude::*;
use std::time::Duration;

const RUN_SPEED: f32 = 7.0;
const ACCELERATION: f32 = 18.0;
const DECELERATION: f32 = 25.0;
const TURN_RATE: f32 = 450_f32.to_radians();

#[derive(Component, Default, Clone)]
#[require(Transform, Velocity, Hitbox, ColorCharge)]
pub struct Player;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
struct SkillTiming {
    cooldown: u64,
    startup: u64,
    active: u64,
    recovery: u64,
}

#[derive(Default, Clone, Reflect, PartialEq)]
pub enum SkillStatus {
    Cooldown,
    #[default]
    Idle,
    Startup,
    Active,
    Recovery,
}

#[derive(Component, Default, Clone, Reflect)]
#[require(SkillTiming)]
#[reflect(Component)]
pub struct SkillState {
    pub status: SkillStatus,
    pub timer: Timer,
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct PlayerHealth(pub(crate) f32);

// some = is invuln, none = not invuln
#[derive(Component, Default, Clone)]
struct Invulnerable(Option<f32>);

pub fn player() -> impl Scene {
    bsn! {
        #Player
        Player
        PlayerHealth(100.0)
        Velocity(Vec3::ZERO)
        Invulnerable(None)
        template(|context| {
            let gltf = context.resource::<PlayerAssets>().gltf.clone();
            Ok(WorldAssetRoot(gltf))
        })
        Transform::from_xyz(0.0, 0.0, 0.0)
        SkillState
        SkillTiming {
            cooldown: 0,
            startup: 1,
            active: 260,
            recovery: 160,
        }
        Hitbox(0.75)
        ColorCharge::WHITE
        on(on_player_bullet_hit)
    }
}

fn on_player_bullet_hit(
    e: On<BulletHit>,
    mut commands: Commands,
    mut bullets: Query<(&BulletOwner, &BulletTarget, &mut ColorCharge), Without<Player>>,
    player: Single<
        (
            Entity,
            &SkillState,
            &ColorCharge,
            &mut PlayerHealth,
            &mut Invulnerable,
        ),
        With<Player>,
    >,
) {
    let bullet = e.bullet_entity;
    let (b_owner, _b_target, mut b_cc) = bullets.get_mut(bullet).unwrap();
    let (player, skill_state, cc, mut health, mut invulnerable) = player.into_inner();
    match skill_state.status {
        SkillStatus::Active => {
            if *b_cc == *cc {
                println!("Bullet deflected");
                commands.trigger(RetargetBullet {
                    bullet,
                    new_target: b_owner.0,
                    new_owner: player,
                });
                *b_cc = ColorCharge::WHITE;
            }
        }
        _ => {
            if let Some(_invulnerable) = invulnerable.0 {
                // do nothing
                // println!("Bullet hit, but invulnerable");
            } else {
                health.0 -= 10.0;
                invulnerable.0 = Some(2.0);
                // println!("Health: {}", health.0);
            }
            // println!("Bullet hit");
        }
    }
}

fn tick_invulnerable(mut query: Query<&mut Invulnerable>, time: Res<Time>) {
    for mut inv in query.iter_mut() {
        match inv.0 {
            Some(t) => {
                if t <= 0.0 {
                    inv.0 = None;
                } else {
                    inv.0 = Some(t - time.delta_secs());
                }
            }
            None => {
                inv.0 = None;
            }
        }
    }
}

fn apply_move(
    time: Res<Time>,
    input: Res<inputs::AccumulatedMove>,
    camera: Single<&Transform, (With<Camera3d>, Without<Player>)>,
    player: Single<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut transform, mut velocity) = player.into_inner();
    let dt = time.delta_secs();
    let input = input.0.clamp_length_max(1.0);

    let forward = camera.forward().as_vec3().with_y(0.0).normalize_or_zero();
    let right = camera.right().as_vec3().with_y(0.0).normalize_or_zero();
    let direction = right * input.x + forward * input.y;
    // Vec3::new(input.x, 0.0, -input.y);
    let rate = if input == Vec2::ZERO {
        DECELERATION
    } else {
        ACCELERATION
    };

    let vel =
        Vec3::new(velocity.0.x, 0.0, velocity.0.z).move_towards(direction * RUN_SPEED, rate * dt);
    velocity.0.x = vel.x;
    velocity.0.z = vel.z;

    if input != Vec2::ZERO {
        let target = Quat::from_rotation_y(f32::atan2(-direction.x, -direction.z));
        transform.rotation = transform.rotation.rotate_towards(target, TURN_RATE * dt);
    }

    // transform.translation += velocity.0 * dt;
}

fn anim_move(
    player_assets: Res<PlayerAssets>,
    mut anim_player: Populated<&mut AnimationPlayer>,
    player: Single<&Velocity, With<Player>>,
) {
    for mut anim_player in anim_player.iter_mut() {
        if player.0.length().abs() > 0.01 {
            anim_player.play(player_assets.forward_i).repeat();
        } else {
            anim_player.stop(player_assets.forward_i);
        }
    }
}

fn skill_transition(
    time: Res<Time>,
    mut player_assets: ResMut<PlayerAssets>,
    player: Single<(&mut SkillState, &SkillTiming, &mut Hitbox), With<Player>>,
    color_skills: Single<(&mut ColorSkills, &Children)>,
    mut skill_cooldowns: Query<&mut Cooldown>,
    lr: Res<AccumulatedLR>,
    mut anim_players: Query<&mut AnimationPlayer>,
) {
    let (mut skill_state, timing, mut hitbox) = player.into_inner();
    let (mut color_skills, skill_children) = color_skills.into_inner();
    skill_state.timer.tick(time.delta());

    match skill_state.status {
        SkillStatus::Cooldown => {
            if skill_state.timer.is_finished() {
                skill_state.status = SkillStatus::Idle;
            }
        }
        SkillStatus::Idle => match lr.into_inner() {
            AccumulatedLR::Left => {
                let len = skill_children.len() as isize;
                let prev = color_skills.0 - 1;
                let skill_child = skill_children[color_skills.0.rem_euclid(len) as usize];
                let prev_skill_child = skill_children[prev.rem_euclid(len) as usize];
                let [mut skill_cooldown, mut prev_skill_cooldown] = skill_cooldowns
                    .get_many_mut([skill_child, prev_skill_child])
                    .unwrap();
                if prev_skill_cooldown.0.is_finished() {
                    println!("LEFT OK!");
                    color_skills.0 -= 1;
                    skill_state.status = SkillStatus::Startup;
                    skill_state.timer =
                        Timer::new(Duration::from_millis(timing.startup), TimerMode::Once);
                    skill_cooldown.0.reset();
                    skill_cooldown.0.unpause();
                    prev_skill_cooldown.0.reset();
                    prev_skill_cooldown.0.pause();
                    for mut anim_player in anim_players.iter_mut() {
                        anim_player.play(player_assets.swat_l_i).replay();
                    }
                }
            }
            AccumulatedLR::Right => {
                let len = skill_children.len() as isize;
                let next = color_skills.0 + 1;
                let skill_child = skill_children[color_skills.0.rem_euclid(len) as usize];
                let next_skill_child = skill_children[next.rem_euclid(len) as usize];
                let [mut skill_cooldown, mut next_skill_cooldown] = skill_cooldowns
                    .get_many_mut([skill_child, next_skill_child])
                    .unwrap();
                if next_skill_cooldown.0.is_finished() {
                    println!("RIGHT OK!");
                    color_skills.0 += 1;
                    skill_state.status = SkillStatus::Startup;
                    skill_state.timer =
                        Timer::new(Duration::from_millis(timing.startup), TimerMode::Once);
                    skill_cooldown.0.reset();
                    skill_cooldown.0.unpause();
                    next_skill_cooldown.0.reset();
                    next_skill_cooldown.0.pause();
                    for mut anim_player in anim_players.iter_mut() {
                        anim_player.play(player_assets.swat_r_i).replay();
                    }
                };

                println!("Right");
            }
            AccumulatedLR::None => {}
        },
        SkillStatus::Startup => {
            if skill_state.timer.is_finished() {
                hitbox.0 = 1.5;
                skill_state.status = SkillStatus::Active;
                skill_state.timer =
                    Timer::new(Duration::from_millis(timing.active), TimerMode::Once);
                let cs: ColorSkills = color_skills.into_inner().clone();
                let cc: ColorCharge = cs.into();
                player_assets.active_color = cc.into();
            }
        }
        SkillStatus::Active => {
            if skill_state.timer.is_finished() {
                hitbox.0 = 0.75;
                skill_state.status = SkillStatus::Recovery;
                skill_state.timer =
                    Timer::new(Duration::from_millis(timing.recovery), TimerMode::Once);
            }
        }
        SkillStatus::Recovery => {
            if skill_state.timer.is_finished() {
                skill_state.status = SkillStatus::Cooldown;
                skill_state.timer =
                    Timer::new(Duration::from_millis(timing.cooldown), TimerMode::Once);
            }
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct RedSkill;
#[derive(Component, Default, Clone)]

pub struct BlueSkill;
#[derive(Component, Default, Clone)]

pub struct GreenSkill;

#[derive(Component, Default, Clone, Reflect, Deref)]
#[reflect(Component)]
pub struct ColorSkills(pub isize);

impl From<ColorSkills> for ColorCharge {
    fn from(cs: ColorSkills) -> Self {
        let i = cs.0.rem_euclid(3);
        match i {
            0 => ColorCharge::RED,
            1 => ColorCharge::GREEN,
            2 => ColorCharge::BLUE,
            _ => unreachable!(),
        }
    }
}

pub fn copy_player_color_from_skills(
    color_skills: Single<&ColorSkills>,
    player: Single<&mut ColorCharge, With<Player>>,
) {
    let mut cc = player.into_inner();
    let i = color_skills.0.rem_euclid(3);
    *cc = match i {
        0 => ColorCharge::RED,
        1 => ColorCharge::GREEN,
        2 => ColorCharge::BLUE,
        _ => unreachable!(),
    };
}

pub fn player_skills() -> impl Scene {
    bsn! {
        #ColorSkills
        ColorSkills
        Children [
            #RedSkill
            RedSkill
            Cooldown::once(2000)
            on(|_e: On<CooldownFinished>| {
                println!("Red skill cooldown finished");
            })
            --
            #GreenSkill
            GreenSkill
            Cooldown::once(2000)
            on(|_e: On<CooldownFinished>| {
                println!("Green skill cooldown finished");
            })
            --
            #BlueSkill
            BlueSkill
            Cooldown::once(2000)
            on(|_e: On<CooldownFinished>| {
                println!("Blue skill cooldown finished");
            })
        ]
    }
}

pub fn color_decay(
    time: Res<Time>,
    mut player_assets: ResMut<PlayerAssets>,
    skill_state: Single<&SkillState>,
) {
    match skill_state.status {
        SkillStatus::Active | SkillStatus::Recovery => {
            player_assets
                .active_color_scale
                .smooth_nudge(&1.0_f32, 4.0, time.delta_secs());
        }
        _ => {
            player_assets
                .active_color_scale
                .smooth_nudge(&0.0_f32, 0.6, time.delta_secs());
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerAssetsPlugin)
            .add_systems(Startup, player.spawn())
            .add_systems(Startup, player_skills.spawn())
            .add_systems(FixedUpdate, apply_move)
            .add_systems(FixedUpdate, skill_transition);
        app.add_systems(FixedUpdate, copy_player_color_from_skills);
        app.add_systems(Update, color_decay);
        app.add_systems(Update, anim_move);
        app.add_systems(Update, tick_invulnerable);
        // .add_systems(FixedUpdate, player_deflect_skill);
        // .add_observer(swat_left)
        // .add_observer(swat_right);
    }
}
