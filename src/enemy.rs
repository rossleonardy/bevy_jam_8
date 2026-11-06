use std::time::Duration;
use bevy::prelude::*;
use crate::bullets::{aim_bullet, Bullet, BulletOwner, BulletTarget};
use crate::physics::Velocity;
use crate::player::Player;

fn spawn_enemy(mut commands: Commands,
               mut meshes: ResMut<Assets<Mesh>>,
               mut materials: ResMut<Assets<StandardMaterial>>,) {
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material = materials.add(StandardMaterial::default());

    commands.spawn_scene(bsn! {
        #EnemyOne
        Mesh3d(mesh)
        MeshMaterial3d<StandardMaterial>(material)
        Transform::from_xyz(0.0, 0.0, 10.0)
        Cooldown::from_millis(2000)
        on(|ev: On<CooldownFinished>, mut commands: Commands,
            query: Query<(&Transform)>,
            player: Single<(&Transform, Entity), With<Player>>| {
                let e = ev.entity;
            let origin = query.get(ev.entity).unwrap().clone();
            let (p_transform, player) = player.into_inner();
            let aim = aim_bullet(origin.translation, p_transform.translation) * 20.;
            commands.spawn_scene(bsn! {
                Bullet
                origin
                Velocity(aim)
                BulletTarget(player)
                BulletOwner(e)
            });
        })
    });
}

#[derive(Component, Default, Clone)]
pub struct Enemy;

#[derive(Component, Default, Clone)]
pub struct Cooldown(pub Timer);

#[derive(EntityEvent)]
pub struct CooldownFinished {
    pub entity: Entity,
}

impl Cooldown {
    pub fn from_millis(millis:u64) -> Self {
        Self(Timer::new(Duration::from_millis(millis), TimerMode::Repeating))
    }
}

pub fn tick_cooldowns(mut commands: Commands,
                      time: Res<Time>,
                      mut cooldowns: Query<(Entity, &mut Cooldown)>,
) {
    for (entity, mut cooldown) in &mut cooldowns {
        cooldown.0.tick(time.delta());

        if cooldown.0.just_finished() {
            commands.trigger(CooldownFinished { entity });
        }
    }
}


fn enemy_ai(enemies: Query<(&Transform), With<Enemy>>) {

}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
        app.add_systems(FixedUpdate, tick_cooldowns);
        app.add_systems(FixedUpdate, enemy_ai);
    }
}