use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_jam_8::bullets::BulletsPlugin;
use bevy_jam_8::physics::*;
use bevy_jam_8::ui::*;
use bevy_jam_8::player::*;

fn setup(mut commands: Commands) {
    commands.spawn_scene(
        bsn! {
            Camera3d
        }
    );
}

fn rotate_sys(
    keys: Res<ButtonInput<KeyCode>>,
    mut i: Single<(& mut ColorSkills)>,
    mut ui_parent: Single<&mut UiTransform, With<CircleParent>>) {
    if keys.just_pressed(KeyCode::ArrowLeft) {
        i.0 -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        i.0 += 1;
    }
}

fn cooldowns(keys: Res<ButtonInput<KeyCode>>,
             mut red_skill: Single<(&mut Cooldown), (With<RedSkill>, Without<GreenSkill>, Without<BlueSkill>)>,
            mut green_skill: Single<(&mut Cooldown), (With<GreenSkill>, Without<RedSkill>, Without<BlueSkill>)>,
            mut blue_skill: Single<(&mut Cooldown), (With<BlueSkill>, Without<RedSkill>, Without<GreenSkill>)>) {
    if keys.just_pressed(KeyCode::KeyR) {
        red_skill.0.reset();
    }
    if keys.just_pressed(KeyCode::KeyG) {
        green_skill.0.reset();
    }
    if keys.just_pressed(KeyCode::KeyB) {
        blue_skill.0.reset();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(UiPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(BulletsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, player_skills.spawn())
        .add_systems(Update, rotate_sys)
        .add_systems(Update, cooldowns)
        .run();
}
