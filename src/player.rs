use std::time::Duration;
use bevy::prelude::*;
use crate::inputs;
use crate::bullets::{BulletHit, BulletOwner, BulletTarget, Hitbox, RetargetBullet};
use crate::inputs::AccumulatedLR;
use crate::physics::*;

const RUN_SPEED: f32 = 6.0;
const SPRINT_SPEED: f32 = 6.0;
const ACCELERATION: f32 = 18.0;
const DECELERATION: f32 = 25.0;
const TURN_RATE: f32 = 450_f32.to_radians();
const GRAVITY: f32 = 20.0;

const P_HEIGHT: f32 = 1.8;
const P_RADIUS: f32 = 0.3;

#[derive(Resource)]
pub struct PlayerAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub ring_mesh: Handle<Mesh>,
    pub ring_material: Handle<StandardMaterial>,
}

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
enum SkillStatus {
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
struct SkillState {
    status: SkillStatus,
    timer: Timer,
}

fn player_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let _capsule = Capsule3d::new(P_RADIUS, P_HEIGHT - 2.0 * P_RADIUS);

    let ring = Plane3d::new(Vec3::Y, Vec2::splat(1.0));

    let texture: Handle<Image> = asset_server.load("direction_arrow.png");
    //let cuboid = Cuboid::new(0.5, 2., 0.5);
    let mesh =  asset_server.load(GltfAssetLabel::Primitive { mesh: 0, primitive: 0 }.from_asset("TEST_HERO.glb"));
    commands.insert_resource(PlayerAssets {
        mesh,
        material: materials.add(Color::srgb(0.2, 0.2, 1.0)),
        ring_mesh: meshes.add(ring),
        ring_material: materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            alpha_mode: AlphaMode::Mask(0.5),
            unlit: true,
            ..default()
        }),
    });
}

fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>
) {
    let mesh = player_assets.mesh.clone();
    let material = player_assets.material.clone();

    let ring = player_assets.ring_mesh.clone();
    let ring_material = player_assets.ring_material.clone();

    commands.spawn_scene(bsn! {
        #Player
        Player
        Velocity(Vec3::ZERO)
        Mesh3d(mesh)
        MeshMaterial3d<StandardMaterial>(material)
        Transform::from_xyz(0.0, 0.0, 0.0)
        SkillState
        SkillTiming {
            cooldown: 0,
            startup: 1,
            active: 260,
            recovery: 160,
        }
        Hitbox(0.5)
        ColorCharge::WHITE
        Children [
            Mesh3d(ring)
            MeshMaterial3d<StandardMaterial>(ring_material)
            Transform::from_xyz(0.0, -1.0, 0.0)
        ]
        on(on_player_bullet_hit)
    });
}

fn on_player_bullet_hit(e: On<BulletHit>,
                        mut commands: Commands,
                        mut bullets: Query<(&BulletOwner, &BulletTarget, &mut ColorCharge), Without<Player>>,
                        player: Single<(Entity, &SkillState, &ColorCharge), With<Player>>,
) {
    let bullet = e.bullet_entity;
    let (b_owner, _b_target, mut b_cc) = bullets.get_mut(bullet).unwrap();
    let (player, skill_state, cc) = player.into_inner();
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
            println!("Bullet hit")
        }
    }

}

const SPEED: f32 = 10.0;

fn apply_move(
    time: Res<Time>,
    input: Res<inputs::AccumulatedMove>,
    camera: Single<&Transform, (With<Camera3d>, Without<Player>)>,
    player: Single<(&mut Transform, &mut Velocity), With<Player>>) {
    let (mut transform, mut velocity) = player.into_inner();
    let dt = time.delta_secs();
    let input = input.0.clamp_length_max(1.0);

    let forward = camera.forward().as_vec3().with_y(0.0).normalize_or_zero();
    let right = camera.right().as_vec3().with_y(0.0).normalize_or_zero();
    let direction = right * input.x + forward * input.y;
        // Vec3::new(input.x, 0.0, -input.y);
    let rate = if input == Vec2::ZERO { DECELERATION } else { ACCELERATION };

    let vel = Vec3::new(velocity.0.x, 0.0, velocity.0.z)
        .move_towards(direction * RUN_SPEED, rate * dt);
    velocity.0.x = vel.x;
    velocity.0.z = vel.z;

    if input != Vec2::ZERO {
        let target = Quat::from_rotation_y(f32::atan2(-direction.x, -direction.z));
        transform.rotation = transform.rotation.rotate_towards(target, TURN_RATE * dt);
    }

    // transform.translation += velocity.0 * dt;
}

fn skill_transition(
    time: Res<Time>,
    player: Single<(&mut SkillState, &SkillTiming, &mut Hitbox), With<Player>>,
    color_skills: Single<(&mut ColorSkills, &Children)>,
    mut skill_cooldowns: Query<&mut Cooldown >,
    lr: Res<AccumulatedLR>) {
    let (mut skill_state, timing, mut hitbox) = player.into_inner();
    let (mut color_skills, skill_children) = color_skills.into_inner();
    skill_state.timer.tick(time.delta());

    match skill_state.status {
        SkillStatus::Cooldown => {
            if skill_state.timer.is_finished() {
                skill_state.status = SkillStatus::Idle;
            }
        }
        SkillStatus::Idle => {
            match lr.into_inner() {
                AccumulatedLR::Left => {
                    let len = skill_children.len() as isize;
                    let prev = color_skills.0 - 1;
                    let prev_skill_child = skill_children[prev.rem_euclid(len) as usize];
                    let mut prev_skill_cooldown = skill_cooldowns.get_mut(prev_skill_child).unwrap();
                    if prev_skill_cooldown.0.is_finished()  {
                        println!("LEFT OK!");
                        color_skills.0 -= 1;
                        skill_state.status = SkillStatus::Startup;
                        skill_state.timer = Timer::new(Duration::from_millis(timing.startup), TimerMode::Once);
                        prev_skill_cooldown.0.reset();
                    }
                }
                AccumulatedLR::Right => {
                    let len = skill_children.len() as isize;
                    let next = color_skills.0 + 1;
                    let next_skill_child = skill_children[next.rem_euclid(len) as usize];
                    let mut next_skill_cooldown = skill_cooldowns.get_mut(next_skill_child).unwrap();
                    if next_skill_cooldown.0.is_finished()  {
                        println!("RIGHT OK!");
                        color_skills.0 += 1;
                        skill_state.status = SkillStatus::Startup;
                        skill_state.timer = Timer::new(Duration::from_millis(timing.startup), TimerMode::Once);
                        next_skill_cooldown.0.reset();
                    }
                    ;

                    println!("Right");
                },
                AccumulatedLR::None => {

                },
            }
        }
        SkillStatus::Startup => {
            if skill_state.timer.is_finished() {
                hitbox.0 = 1.0;
                skill_state.status = SkillStatus::Active;
                skill_state.timer = Timer::new(Duration::from_millis(timing.active), TimerMode::Once);
            }
        }
        SkillStatus::Active => {
            if skill_state.timer.is_finished() {
                hitbox.0 = 0.5;
                skill_state.status = SkillStatus::Recovery;
                skill_state.timer = Timer::new(Duration::from_millis(timing.recovery), TimerMode::Once);
            }
        }
        SkillStatus::Recovery => {
            if skill_state.timer.is_finished() {
                skill_state.status = SkillStatus::Cooldown;
                skill_state.timer = Timer::new(Duration::from_millis(timing.cooldown), TimerMode::Once);
            }
        }
    }
}

fn player_debug(_gizmos: Gizmos, _player: Single<(&SkillState, &Transform), With<Player>>) {
    // let (skill_state, transform) = player.into_inner();
    // // gizmos.sphere(Isometry3d::new(transform.translation, transform.rotation), 0.5, WHITE);
    // if let Some(color) = match skill_state.status {
    //     SkillStatus::Idle => None,
    //     SkillStatus::Startup => Some(GRAY),
    //     SkillStatus::Active => Some(BLUE),
    //     SkillStatus::Recovery => Some(BLUE_VIOLET),
    //     SkillStatus::Cooldown => None,
    // } {
    //     gizmos.sphere(Isometry3d::new(transform.translation, transform.rotation), 1., color);
    // }
}


// fn player_deflect_skill(
//     mut commands: Commands,
//     player_query: Single<(Entity, &Transform, &SkillState)>,
//     bullet_query: Query<(Entity, &Transform, &Targeting), With<Bullet>>,
// ) {
//     let (player, p_transform, p_skillstate) = player_query.into_inner();
//     if p_skillstate.status == SkillStatus::Active {
//         for (bullet, b_transform, targeting) in bullet_query.iter() {
//             let hit = p_transform.translation.distance_squared(b_transform.translation) <= 1.0;
//             if hit {
//                 // println!("Deflecting bullet {}", bullet);
//                 commands.entity(bullet).despawn();
//             }
//         }
//     }
// }


#[derive(Component, Default, Clone)]
pub struct RedSkill;
#[derive(Component, Default, Clone)]

pub struct BlueSkill;
#[derive(Component, Default, Clone)]

pub struct GreenSkill;

#[derive(Component, Default, Clone, Reflect, Deref)]
#[reflect(Component)]
pub struct ColorSkills(pub isize);


pub fn copy_player_color_from_skills(
    color_skills: Single<&ColorSkills >,
    player: Single<&mut ColorCharge , With<Player>>,
) {
    let mut cc  = player.into_inner();
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

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, player_assets)
            .add_systems(Startup, spawn_player)
            .add_systems(Startup, player_skills.spawn())
            .add_systems(FixedUpdate, apply_move)
            .add_systems(Update, player_debug)
            .add_systems(FixedUpdate, skill_transition);
        app.add_systems(FixedUpdate, copy_player_color_from_skills);
            // .add_systems(FixedUpdate, player_deflect_skill);
            // .add_observer(swat_left)
            // .add_observer(swat_right);
    }
}
