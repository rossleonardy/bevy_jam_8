use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_jam_8::player::Player;
use bevy_jam_8::{bullets, enemy, inputs, physics, player, ui};

fn setup(mut commands: Commands) {
    commands.spawn_scene(enemy::enemy_alpha());
}

fn init_scene() -> impl SceneList {
    bsn_list! {
        Camera3d
        Transform::from_xyz(0.0, 10.0, -20.0).looking_at(Vec3::ZERO, Vec3::Y)
        --
        PointLight
        Transform::from_xyz(0.0, 10.0, 0.0)
        --
        Player
        Transform::from_xyz(0.0, 0.0, 0.0)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(bullets::BulletsPlugin)
        .add_systems(Startup, init_scene.spawn())
        .add_systems(Startup, setup)
        .run();
}
