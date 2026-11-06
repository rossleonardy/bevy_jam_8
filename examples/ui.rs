use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_jam_8::ui::{CircleParent, UiCircleIndex, UiCooldownPercent, UiPlugin};

fn setup(mut commands: Commands) {
    commands.spawn_scene(
        bsn! {
            Camera3d
        }
    );
}

fn rotate_sys(
    keys: Res<ButtonInput<KeyCode>>,
    mut i: ResMut<UiCircleIndex>,
    mut ui_parent: Single<&mut UiTransform, With<CircleParent>>) {
    if keys.just_pressed(KeyCode::ArrowLeft) {
        i.0 -= 1.;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        i.0 += 1.;
    }
}

fn cooldown_sys(
    keys: Res<ButtonInput<KeyCode>>,
    mut cooldown: ResMut<UiCooldownPercent>,
) {
     if keys.just_pressed(KeyCode::Space) {
        cooldown.0 = 0.;
     }
    if cooldown.0 < 1. {
        cooldown.0 += 0.01;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_sys)
        .add_systems(Update, cooldown_sys)
        .run();
}
