use std::time::Duration;
use bevy::prelude::*;
use crate::bullets::{aim_bullet, Bullet, BulletHit, BulletOwner, BulletTarget, Hitbox};
use crate::physics::*;
use crate::player::*;

fn spawn_enemy(mut commands: Commands,
               mut meshes: ResMut<Assets<Mesh>>,
               mut materials: ResMut<Assets<StandardMaterial>>,) {
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material = materials.add(StandardMaterial::default());

    // commands.spawn_scene(bsn! {
    //     #EnemyOne
    //     Mesh3d(mesh)
    //     MeshMaterial3d<StandardMaterial>(material)
    //     Transform::from_xyz(0.0, 0.0, 10.0)
    //     Cooldown::repeat(2000)
    //     Hitbox
    //     on(|ev: On<CooldownFinished>, mut commands: Commands,
    //         query: Query<(&Transform)>,
    //         player: Single<(&Transform, Entity), With<Player>>| {
    //             let e = ev.entity;
    //         let origin = query.get(ev.entity).unwrap().clone();
    //         let (p_transform, player) = player.into_inner();
    //         let aim = aim_bullet(origin.translation, p_transform.translation) * 20.;
    //         commands.spawn_scene(bsn! {
    //             Bullet
    //             ColorCharge::RED
    //             origin
    //             Velocity(aim)
    //             BulletTarget(player)
    //             BulletOwner(e)
    //         });
    //     })
    //
    // });

    commands.spawn_scene(bsn! {
        @enemy_alpha()
        Transform::from_xyz(0.0, 0.0, -10.0)
    });
}

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
            query: Query<(&Transform)>,
            player: Single<(Entity), With<Player>>|
        {
            let player = player.into_inner();
            let origin = query.get(e.entity).unwrap().clone();
            let aim = origin.forward() * 10.0;
            commands.spawn_scene(bsn! {
                Bullet
                ColorCharge::RED
                origin
                Velocity(aim)
                BulletTarget(player)
                BulletOwner({e.entity})
            });
        })
        on(|e: On<BulletHit>| {
            println!("Enemy hit by bullet");
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

fn enemy_ai(enemies: Query<(&Transform), With<Enemy>>) {

}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
        app.add_systems(FixedUpdate, enemy_ai);
        app.add_systems(FixedUpdate, spin_y_ai);
    }
}