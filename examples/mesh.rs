use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::gltf::GltfPlugin;
use bevy::gltf::convert_coordinates::GltfConvertCoordinates;
use bevy::input::common_conditions::input_toggle_active;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_jam_8::inputs::InputsPlugin;
use bevy_jam_8::physics::ColorCharge;
use bevy_jam_8::player::PlayerPlugin;
use bevy_jam_8::player_assets::PlayerAssets;

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 10.0);

fn mesh_scene() -> impl SceneList {
    bsn_list! {
        Camera3d
        Tonemapping::TonyMcMapface
        Bloom::NATURAL
        Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y)
        --
        PointLight
        Transform::from_xyz(0.0, 10.0, 0.0)
        --
        // @player()
    }
}

fn test_colors(keys: Res<ButtonInput<KeyCode>>, mut player_assets: ResMut<PlayerAssets>) {
    if keys.just_pressed(KeyCode::KeyR) {
        player_assets.active_color_scale = 1.0;
        player_assets.active_color = ColorCharge::RED.into();
    } else if keys.just_pressed(KeyCode::KeyG) {
        player_assets.active_color_scale = 1.0;
        player_assets.active_color = ColorCharge::GREEN.into();
    } else if keys.just_pressed(KeyCode::KeyB) {
        player_assets.active_color_scale = 1.0;
        player_assets.active_color = ColorCharge::BLUE.into();
    }
}

fn test_anims(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_assets: ResMut<PlayerAssets>,
    anim_player: Query<(&mut AnimationPlayer)>,
) {
    if keys.just_pressed(KeyCode::Digit7) {
        for mut player in anim_player {
            player.play(player_assets.forward_i).replay();
        }
    } else if keys.just_pressed(KeyCode::Digit8) {
        for mut player in anim_player {
            player.play(player_assets.swat_l_i).replay();
        }
    } else if keys.just_pressed(KeyCode::Digit9) {
        for mut player in anim_player {
            player.play(player_assets.swat_r_i).replay();
        }
    }
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
        .add_plugins(InputsPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Update, test_colors)
        .add_systems(Update, test_anims)
        .add_systems(Startup, mesh_scene.spawn())
        .run();
}
