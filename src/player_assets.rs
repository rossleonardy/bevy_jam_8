use crate::physics::ColorCharge;
use bevy::gltf::GltfMaterialName;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

#[derive(Resource, Default)]
pub struct PlayerAssets {
    pub gltf: Handle<WorldAsset>,
    pub material: Handle<PlayerMaterial>,
    pub active_color_scale: f32,
    pub active_color: Srgba,
    pub animation_graph: Handle<AnimationGraph>,
    pub forward_anim: Handle<AnimationClip>,
    pub swat_l_anim: Handle<AnimationClip>,
    pub swat_r_anim: Handle<AnimationClip>,
    pub forward_i: AnimationNodeIndex,
    pub swat_l_i: AnimationNodeIndex,
    pub swat_r_i: AnimationNodeIndex,
}

const DEFAULT_GOLD: Srgba = Srgba::rgb(0.4, 0.3, 0.0);

impl PlayerAssets {
    fn get_color(self: &Self) -> Srgba {
        // if active color scale is 0 return gold
        DEFAULT_GOLD.mix(&self.active_color, self.active_color_scale)
    }
}

const PLAYER_SHADER: &str = "player_shader.wesl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct PlayerExtension {
    #[uniform(100)]
    pub end_color: LinearRgba,
    #[texture(101)]
    #[sampler(102)]
    pub mask: Handle<Image>,
}

impl MaterialExtension for PlayerExtension {
    fn fragment_shader() -> ShaderRef {
        PLAYER_SHADER.into()
    }
    fn deferred_fragment_shader() -> ShaderRef {
        PLAYER_SHADER.into()
    }
}

pub type PlayerMaterial = ExtendedMaterial<StandardMaterial, PlayerExtension>;

const CLOAK_PATH: &str = "cloak.glb";
fn player_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlayerMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    let mask = asset_server.load("cloak_mask.png");
    // let _capsule = Capsule3d::new(crate::player::P_RADIUS, crate::player::P_HEIGHT - 2.0 * crate::player::P_RADIUS);
    let mat = materials.add(ExtendedMaterial {
        base: StandardMaterial::default(),
        extension: PlayerExtension {
            end_color: Srgba::rgb(0.9, 0.8, 0.0).into(),
            mask,
        },
    });

    let texture: Handle<Image> = asset_server.load("direction_arrow.png");
    //let cuboid = Cuboid::new(0.5, 2., 0.5);
    let gltf = asset_server.load(GltfAssetLabel::Scene(0).from_asset(CLOAK_PATH));

    let forward_anim: Handle<AnimationClip> =
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(CLOAK_PATH));
    let swat_l_anim: Handle<AnimationClip> =
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(CLOAK_PATH));
    let swat_r_anim: Handle<AnimationClip> =
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(CLOAK_PATH));

    let mut animation_graph = AnimationGraph::new();
    let forward_i = animation_graph.add_clip(forward_anim.clone(), 1.0, animation_graph.root);
    let swat_l_i = animation_graph.add_clip(swat_l_anim.clone(), 1.0, animation_graph.root);
    let swat_r_i = animation_graph.add_clip(swat_r_anim.clone(), 1.0, animation_graph.root);

    commands.insert_resource(PlayerAssets {
        gltf,
        material: mat,
        animation_graph: graphs.add(animation_graph),
        forward_i,
        swat_l_i,
        swat_r_i,
        ..default()
    });
}

fn swap_materials(
    mut commands: Commands,
    added: Query<(Entity, &GltfMaterialName), Added<MeshMaterial3d<StandardMaterial>>>,
    player_assets: Res<PlayerAssets>,
) {
    for (entity, name) in &added {
        println!("Name: {}", name.0);
        if name.0 == "PlayerMaterial" {
            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .insert(MeshMaterial3d(player_assets.material.clone()));
        }
    }
}

fn update_material(
    player: Query<&MeshMaterial3d<PlayerMaterial>>,
    mut materials: ResMut<Assets<PlayerMaterial>>,
    player_assets: Res<PlayerAssets>,
) {
    for handle in &player {
        if let Some(mut mat) = materials.get_mut(handle) {
            mat.extension.end_color = player_assets.get_color().into();
        }
    }
}

fn update_graph(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationPlayer)>,
    player_assets: Res<PlayerAssets>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    for (entity, mut player) in query.iter_mut() {
        commands
            .entity(entity)
            .insert((AnimationGraphHandle(player_assets.animation_graph.clone()),));
    }
}

pub struct PlayerAssetsPlugin;
impl Plugin for PlayerAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PlayerMaterial>::default());
        app.add_systems(PreStartup, player_assets);
        app.add_systems(Update, swap_materials);
        app.add_systems(Update, update_material);
        app.add_systems(Update, update_graph);
    }
}
