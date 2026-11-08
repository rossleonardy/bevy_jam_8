use crate::bullets::{Bullet, BulletHit, Hitbox};
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Enemy;

#[derive(Component, Default, Clone)]
pub struct SpeciesAlpha;

pub fn enemy_alpha() -> impl Scene {
    bsn! {
        SpeciesAlpha
        WorldAssetRoot("enemy_alpha.glb#Scene0")
        Transform::default()
        Cooldown::repeat(200)
        Hitbox
        SpinY
        on(|e: On<CooldownFinished>,
            mut commands: Commands,
            query: Query<&Transform >,
            player: Single<Entity , With<Player>>,
            colors: Query<&ColorCharge >|
        {
            let cc = *colors.get(e.entity).unwrap();
            let player = player.into_inner();
            let origin = *query.get(e.entity).unwrap();
            let aim = origin.forward() * 10.0;
            commands.spawn_scene(bsn! {
                @Bullet {
                    @target: player,
                    @owner: {e.entity},
                    @cc: cc
                }
                origin
                Velocity(aim)
            });
        })
        on(|e: On<BulletHit>, mut commands: Commands, cc: Query<&ColorCharge >| {
            let color = cc.get(e.bullet_entity).unwrap();
            match color {
                ColorCharge::WHITE => {
                    println!("Despawning enemy");
                    commands.entity(e.target).despawn();
                }
                _ => { /* do nothing */ }
            }

        })
    }
}

/// in degrees per second
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct SpinY(pub f32);
impl Default for SpinY {
    fn default() -> Self {
        Self(1.)
    }
}

pub fn spin_y_ai(mut enemies: Query<(&mut Transform, &SpinY)>) {
    for (mut transform, spin) in enemies.iter_mut() {
        transform.rotate_y(spin.0.to_radians());
        transform.rotation = transform.rotation.normalize();
    }
}

fn enemy_ai(_enemies: Query<&Transform, With<Enemy>>) {}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_enemy);
        app.add_systems(FixedUpdate, enemy_ai);
        app.add_systems(FixedUpdate, spin_y_ai);
    }
}
