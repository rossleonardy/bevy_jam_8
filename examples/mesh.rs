use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::gltf::convert_coordinates::GltfConvertCoordinates;
use bevy::gltf::GltfPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 10.0);

fn mesh_scene() -> impl SceneList {
    bsn_list!{
        Camera3d
        Tonemapping::TonyMcMapface
        Bloom::NATURAL
        Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y)
        --
        PointLight
        Transform::from_xyz(0.0, 10.0, 0.0)
        --
        WorldAssetRoot("TEST_HERO.glb#Scene0")
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
        .add_systems(Startup, mesh_scene.spawn())
        .run();
}
