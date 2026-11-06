
use bevy_jam_8::{bullets, enemy, inputs, physics, player, ui};
use bevy::prelude::*;

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use std::f32::consts::FRAC_PI_2;



fn spawn_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name("Stage".into()),
        Mesh3d(meshes.add(Circle::new(10.))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
    ));
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, -20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Point light
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(0.0, 10.0, 0.0),
        ShowLightGizmo::default(),
    ));
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
        // .add_plugins(ui::UiPlugin)
        .add_systems(Startup, (setup, spawn_stage).chain())
        .run();
}
