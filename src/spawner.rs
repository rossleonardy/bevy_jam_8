use crate::enemy::{EnemyShields, SpinY, enemy_alpha, enemy_beta, enemy_gamma};
use crate::physics::ColorCharge;
use bevy::prelude::*;
use std::collections::VecDeque;

const STAGE_HALF_HEIGHT: f32 = 8.0;
const STAGE_HALF_WIDTH: f32 = 12.0;

fn ellipse_angle(half_width: f32, half_height: f32, angle: f32) -> Vec2 {
    let angle = angle.to_radians();
    Vec2::new(half_width * angle.cos(), half_height * angle.sin())
}

fn stage_edge(angle: f32) -> Vec2 {
    ellipse_angle(STAGE_HALF_WIDTH, STAGE_HALF_HEIGHT, -angle)
}

fn stage_edge_3(angle: f32) -> Vec3 {
    let xz = stage_edge(angle);
    Vec3::new(xz.x, 0.0, xz.y)
}

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnQueue {
    queue: VecDeque<Box<dyn Scene>>,
}

impl Default for SpawnQueue {
    fn default() -> Self {
        Self {
            queue: VecDeque::from([
                xxa(),
                axx(),
                axa(),
                xbx(),
                bxa(),
                axb(),
                aaa(),
                aba(),
                xgx(),
                gxa(),
                axg(),
                bab(),
                gxg(),
                xgx_final(),
            ]),
        }
    }
}

fn xxa() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Transform::default()
        Children [
            @enemy_alpha()
            ColorCharge::BLUE
            Transform::from_translation(stage_edge_3(30.))
        ]
    }
    .into()
}

fn axx() -> Box<dyn Scene> {
    bsn! {
                    @EnemyGroup
                    Transform::default()
                    Children [
                        @enemy_alpha()
                        ColorCharge::RED
                        Transform::from_translation(stage_edge_3(150.))
                    ]
    }
    .into()
}

fn axa() -> Box<dyn Scene> {
    bsn! {
                    @EnemyGroup
                    Children [
                        @enemy_alpha()
                        ColorCharge::GREEN
                        Transform::from_translation(stage_edge_3(45.))
                        --
                        @enemy_alpha()
                        ColorCharge::RED
                        Transform::from_translation(stage_edge_3(135.))
                    ]
    }
    .into()
}

fn xbx() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_beta()
            ColorCharge::RED
            Transform::from_translation(stage_edge_3(90.))
        ]
    }
    .into()
}

fn bxa() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_beta()
            ColorCharge::GREEN
            Transform::from_translation(stage_edge_3(180.))
            --
            @enemy_alpha()
            ColorCharge::BLUE
            Transform::from_translation(stage_edge_3(15.))
        ]
    }
    .into()
}

fn axb() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_alpha()
            ColorCharge::RED
            Transform::from_translation(stage_edge_3(150.))
            --
            @enemy_beta()
            ColorCharge::GREEN
            Transform::from_translation(stage_edge_3(45.))
        ]
    }
    .into()
}

fn aaa() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_alpha()
            ColorCharge::BLUE
            Transform::from_translation(stage_edge_3(195.))
            --
            @enemy_alpha()
            ColorCharge::RED
            Transform::from_translation(stage_edge_3(105.))
            --
            @enemy_alpha()
            ColorCharge::GREEN
            Transform::from_translation(stage_edge_3(15.))
        ]
    }
    .into()
}

fn aba() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_alpha()
            ColorCharge::BLUE
            Transform::from_translation(stage_edge_3(150.))
            --
            @enemy_beta()
            ColorCharge::RED
            Transform::from_translation(stage_edge_3(75.))
            --
            @enemy_alpha()
            ColorCharge::GREEN
            Transform::from_translation(stage_edge_3(-15.))
        ]
    }
    .into()
}

fn xgx() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_gamma()
            ColorCharge::RED
            Transform::from_translation(stage_edge_3(90.))
        ]
    }
    .into()
}

fn gxa() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_gamma()
            ColorCharge::GREEN
            Transform::from_translation(stage_edge_3(150.))
            --
            @enemy_alpha()
            SpinY(-4.)
            Transform::from_translation(stage_edge_3(30.))
        ]
    }
    .into()
}

fn axg() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_alpha()
            ColorCharge::RED
            SpinY(10.)
            Transform::from_translation(stage_edge_3(180.))
            --
            @enemy_gamma()
            ColorCharge::BLUE
            Transform::from_translation(stage_edge_3(15.))
        ]
    }
    .into()
}

fn bab() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_beta()
            ColorCharge::BLUE
            Transform::from_translation(stage_edge_3(120.))
            --
            @enemy_alpha()
            ColorCharge::GREEN
            Transform::from_translation(stage_edge_3(90.))
            --
            @enemy_beta()
            ColorCharge::RED
            Transform::from_translation(stage_edge_3(60.))
        ]
    }
    .into()
}

fn gxg() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
    Children [
        @enemy_gamma()
        Transform::from_translation(stage_edge_3(150.))
        --
        @enemy_gamma()
        ColorCharge::GREEN
        Transform::from_translation(stage_edge_3(30.))
    ]}
    .into()
}

fn xgx_final() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_gamma()
            Transform::from_translation(stage_edge_3(90.))
            EnemyShields(5)
        ]
    }
    .into()
}

#[derive(SceneComponent, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct EnemyGroup;

impl EnemyGroup {
    pub fn scene() -> impl Scene {
        bsn! {
            #EnemyGroup
            Transform::default()
            Children []
        }
    }
}

pub fn run_spawner(mut commands: Commands, mut spawn_queue: ResMut<SpawnQueue>) {
    if let Some(spawnable) = spawn_queue.queue.pop_front() {
        commands.spawn_scene(spawnable);
    }
}

pub fn check_for_empty_groups(
    mut commands: Commands,
    mut spawn_queue: ResMut<SpawnQueue>,
    enemy_groups: Query<(&EnemyGroup, Entity), Without<Children>>,
) {
    for (_group, entity) in enemy_groups.iter() {
        //
        println!("Despawning group");
        commands.entity(entity).despawn();
        if let Some(spawnable) = spawn_queue.queue.pop_front() {
            commands.spawn_scene(spawnable);
        }
    }
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnQueue::default());
        app.add_systems(FixedUpdate, check_for_empty_groups);
        app.add_systems(Startup, run_spawner);
    }
}
