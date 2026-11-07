
use bevy_jam_8::{bullets, enemy, inputs, physics, player, ui};
use bevy::prelude::*;

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use std::f32::consts::{FRAC_PI_2, PI};

fn spawn_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name("Stage".into()),
        Mesh3d(meshes.add(Circle::new(10.))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform {
            translation: Vec3::new(0.0, -1.0, 0.0),
            rotation: Quat::from_rotation_x(-FRAC_PI_2),
            scale: Vec3::splat(1.0),
        }
    ));
}


fn init_scene() -> impl SceneList {
    bsn_list!{
        Camera3d
        Transform::from_xyz(0.0, 10.0, 15.0)
        CameraBounds {
            base_yaw: 0.0,
            yaw_bound: {(-10f32.to_radians(), 10f32.to_radians())},
            pitch_bound: {(-45f32.to_radians(), 45f32.to_radians())},
        }
        --
        PointLight
        Transform::from_xyz(0.0, 10.0, 0.0)
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct CameraBounds {
    base_yaw: f32,
    yaw_bound: (f32, f32),
    pitch_bound: (f32, f32),
}

fn camera_looking_at(
    mut camera: Single<(&mut Transform, &CameraBounds), (With<Camera3d>, Without<player::Player>)>,
    player: Single<&Transform, With<player::Player>>,
) {
    let (mut camera_transform, camera_bounds) = camera.into_inner();

    let dir = (player.translation - camera_transform.translation).normalize_or_zero();
    if dir == Vec3::ZERO {
        return;
    }

    let yaw = (-dir.x).atan2(-dir.z);
    let pitch = dir.y.clamp(-1.0, 1.0).asin();

    let base_yaw = camera_bounds.base_yaw;
    let (yaw_min, yaw_max) = camera_bounds.yaw_bound;
    let (pitch_min, pitch_max) = camera_bounds.pitch_bound;
    let rel_yaw = (yaw - base_yaw + PI).rem_euclid(2.0 * PI) - PI;
    let yaw = base_yaw + rel_yaw.clamp(yaw_min, yaw_max);
    let pitch = pitch.clamp(pitch_min, pitch_max);
    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    //**camera = camera.looking_at(player.translation, Vec3::Y);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(inputs::InputsPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(bullets::BulletsPlugin)
        .add_plugins(ui::UiPlugin)
        .add_systems(Startup, (init_scene.spawn(), spawn_stage).chain())
        .add_systems(Update, camera_looking_at)
        .run();
}
