use crate::enemy::{cycle_color_child, enemy_alpha, enemy_beta, enemy_gamma, SpeciesGamma};
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
                bsn! {
                    @EnemyGroup
                    Transform::default()
                    Children [
                        @enemy_alpha()
                        ColorCharge::BLUE
                        Transform::from_translation(stage_edge_3(30.))
                    ]
                    #TestGroup
                }
                .into(),
                bsn! {
                    @EnemyGroup
                    Transform::default()
                    Children [
                        @enemy_alpha()
                        ColorCharge::RED
                        Transform::from_translation(stage_edge_3(150.))
                    ]
                }
                .into(),
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
                .into(),
                bsn! {
                    @EnemyGroup
                    Children [
                        @enemy_beta()
                        ColorCharge::RED
                        Transform::from_translation(stage_edge_3(90.))
                    ]
                }.into(),
                GAMMA_TEST(),
            ]),
        }
    }
}

fn GAMMA_TEST() -> Box<dyn Scene> {
    bsn! {
        @EnemyGroup
        Children [
            @enemy_gamma()
            ColorCharge::RED
            Transform::from_translation(stage_edge_3(150.))
        ]
    }.into()
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
