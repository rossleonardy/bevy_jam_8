
use bevy_jam_8::{bullets, enemy, inputs, physics, player, spawner, ui};
use bevy::prelude::*;
use bevy_transform_interpolation::prelude::*;

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use std::f32::consts::{FRAC_PI_2, PI};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::gltf::convert_coordinates::GltfConvertCoordinates;
use bevy::gltf::GltfPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::post_process::bloom::Bloom;
use bevy::window::WindowWrapper;

fn spawn_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((
        Name("Stage".into()),
        Mesh3d(meshes.add(Ellipse::new(12., 8.))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform {
            translation: Vec3::new(0.0, -1.0, 0.0),
            rotation: Quat::from_rotation_x(-FRAC_PI_2),
            scale: Vec3::splat(1.0),
        }
    ));
}

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 14.0, 15.0);

fn init_scene() -> impl SceneList {
    bsn_list!{
        Camera3d
        Tonemapping::TonyMcMapface
        Bloom::NATURAL
        Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y)
        --
        PointLight
        Transform::from_xyz(0.0, 10.0, 0.0)
    }
}

fn camera_pan(
    time: Res<Time>,
    player: Single<&Transform, With<player::Player>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<player::Player>)>
) {
    let target = player.translation * 0.5 + CAMERA_OFFSET;
    camera.translation.smooth_nudge(&target, 3.0, time.delta_secs());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set({
            GltfPlugin {
                convert_coordinates: GltfConvertCoordinates {
                    rotate_meshes: true,
                    ..default()
                },
                ..default()
            }
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Tab)))
        .add_plugins(TransformInterpolationPlugin::interpolate_all())
        .add_plugins(inputs::InputsPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(bullets::BulletsPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(spawner::SpawnerPlugin)
        .add_systems(Startup, (init_scene.spawn(), spawn_stage).chain())
        .add_systems(Update, camera_pan)
        .run();
}
