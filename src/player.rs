use std::time::Duration;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use crate::inputs;
use bevy_enhanced_input::prelude::*;
use crate::bullets::{Bullet, BulletHit, BulletTarget, Hitbox};
use crate::inputs::AccumulatedLR;
use crate::physics::Velocity;

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
    let capsule = Capsule3d::new(P_RADIUS, P_HEIGHT - 2.0 * P_RADIUS);

    let ring = Plane3d::new(Vec3::Y, Vec2::splat(1.0));

    let texture: Handle<Image> = asset_server.load("direction_arrow.png");
    //let cuboid = Cuboid::new(0.5, 2., 0.5);
    commands.insert_resource(PlayerAssets {
        mesh: meshes.add(capsule),
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
        Transform::from_xyz(0.0, P_HEIGHT / 2.0, 0.0)
        SkillState
        SkillTiming {
            cooldown: 1000,
            startup: 1,
            active: 260,
            recovery: 160,
        }
        Hitbox(0.5)
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
                        player: Single<&SkillState ,With<Player>>) {
    let bullet = e.bullet_entity;
    let (skill) = player.into_inner();
    match skill.status {
        SkillStatus::Active => {
            println!("Bullet deflected");
            commands.entity(bullet).despawn();
        }
        _ => {
            println!("Bullet hit")
        }
    }

}

const SPEED: f32 = 10.0;

fn apply_move(
    time: Res<Time>,
    move_dir: Res<inputs::AccumulatedMove>,
    player: Single<(&mut Transform, &mut Velocity), With<Player>>) {
    let (mut transform, mut velocity) = player.into_inner();
    // RUN_SPEED
    let dt = time.delta_secs();
    let move_dir = move_dir.0.clamp_length_max(1.0);

    let direction = Vec3::new(-move_dir.x, 0.0, move_dir.y);
    let rate = if move_dir == Vec2::ZERO { DECELERATION } else { ACCELERATION };

    let vel = Vec3::new(velocity.0.x, 0.0, velocity.0.z)
        .move_towards(direction * RUN_SPEED, rate * dt);
    velocity.0.x = vel.x;
    velocity.0.z = vel.z;

    if move_dir != Vec2::ZERO {
        let target = Quat::from_rotation_y(f32::atan2(-direction.x, -direction.z));
        transform.rotation = transform.rotation.rotate_towards(target, TURN_RATE * dt);
    }

    // transform.translation += velocity.0 * dt;
}

fn skill_transition(
    time: Res<Time>,
    player: Single<(&mut SkillState, &SkillTiming, &mut Hitbox), With<Player>>,
    lr: Res<AccumulatedLR>) {
    let (mut skill_state, timing, mut hitbox) = player.into_inner();
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
                    println!("Left");
                    skill_state.status = SkillStatus::Startup;
                    skill_state.timer = Timer::new(Duration::from_millis(timing.startup), TimerMode::Once);
                },
                AccumulatedLR::Right => {
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

fn player_debug(mut gizmos: Gizmos, player: Single<(&SkillState, &Transform), With<Player>>) {
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

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, player_assets)
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, apply_move)
            .add_systems(Update, player_debug)
            .add_systems(FixedUpdate, skill_transition);
            // .add_systems(FixedUpdate, player_deflect_skill);
            // .add_observer(swat_left)
            // .add_observer(swat_right);
    }
}
