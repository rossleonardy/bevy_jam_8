use crate::bullets::{Bullet, BulletHit, BulletOwner, BulletTarget, Hitbox, RetargetBullet};
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Enemy;

#[derive(Component, Default, Clone)]
pub struct SpeciesAlpha;

// random spray
pub fn enemy_alpha() -> impl Scene {
    bsn! {
        SpeciesAlpha
        WorldAssetRoot("alpha.glb#Scene0")
        Transform::default()
        Cooldown::repeat(200)
        Hitbox
        SpinY
        Children [
            @fire_straight_bullet_child()
            --
        ]
        @on_hit()

    }
}

#[derive(Component, Default, Clone)]
pub struct SpeciesBeta;

// stalker
pub fn enemy_beta() -> impl Scene {
    bsn! {
        SpeciesBeta
        WorldAssetRoot("alpha.glb#Scene0")
        Transform::default()
        Cooldown::repeat(500)
        Hitbox
        LookAtPlayer
        @on_hit()
        Children [
            @fire_straight_bullet_child()
            Cooldown::repeat(400)
            --
            @cycle_color_child()
        ]
    }
}

#[derive(Component, Default, Clone)]
pub struct SpeciesGamma;

pub fn enemy_gamma() -> impl Scene {
    bsn! {
        SpeciesGamma
        WorldAssetRoot("alpha.glb#Scene0")
        Transform::default()
        Hitbox(3.0)
        LookAtPlayer
        ColorCharge::RED
        @on_hit_deflect()
        Children [
            @fire_singleton_bullet_child()
        ]
    }
}

pub fn on_hit() -> impl Scene {
    bsn! {
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

#[derive(Component, Default, Clone)]
pub struct EnemyShields(pub u8);

pub fn on_hit_deflect() -> impl Scene {
    bsn! {
        EnemyShields(3)
        on(|e: On<BulletHit>, mut commands: Commands,
            mut shields: Query<&mut EnemyShields>,
            bullets: Query<(
                Entity,
                &BulletOwner,
                &BulletTarget,
            )>| {
            if let Ok(mut shields) = shields.get_mut(e.target) {
                if shields.0 > 0 {
                    shields.0 -= 1;
                    let (entity, b_owner, b_target) = bullets.get(e.bullet_entity).unwrap();
                    commands.trigger(RetargetBullet {
                            bullet: e.bullet_entity,
                            new_target: b_owner.0,
                            new_owner: b_target.0,
                    });
                }   else {

                }
            }
        })
    }
}

pub fn fire_straight_bullet_child() -> impl Scene {
    bsn! {
        Cooldown::repeat(200)
        on(|e: On<CooldownFinished>,
            mut commands: Commands,
            query: Query<&Transform >,
            player: Single<Entity , With<Player>>,
            colors: Query<&ColorCharge >,
            child_of_query: Query<&ChildOf> |
        {
            let parent = match child_of_query.get(e.entity) {
                Ok(childof) => childof.parent(),
                _ => return ()
            };

            let cc = *colors.get(parent).unwrap();
            let player = player.into_inner();
            let origin = *query.get(parent).unwrap();
            let aim = origin.forward() * 10.0;
            commands.spawn_scene(bsn! {
                @Bullet {
                    @target: player,
                    @owner: parent,
                    @cc: cc
                }
                origin
                Velocity(aim)
            });
        })
    }
}

#[derive(Component, Default, Clone)]
struct SingletonBullet;

fn fire_singleton_bullet_child() -> impl Scene {
    bsn! {
        Cooldown::repeat(200)
        on(|e: On<CooldownFinished>,
            mut commands: Commands,
            query: Query<&Transform >,
            player: Single<Entity , With<Player>>,
            colors: Query<&ColorCharge >,
            child_of_query: Query<&ChildOf>,
            singleton_bullet: Query<&SingletonBullet>|
        {
            if singleton_bullet.count() > 0 {
                return;
            }
            let parent = match child_of_query.get(e.entity) {
                Ok(childof) => childof.parent(),
                _ => return ()
            };

            let cc = *colors.get(parent).unwrap();
            let player = player.into_inner();
            let origin = *query.get(parent).unwrap();
            let aim = origin.forward() * 10.0;
            commands.spawn_scene(bsn! {
                @Bullet {
                    @target: player,
                    @owner: parent,
                    @cc: cc
                }
                origin
                Velocity(aim)
                SingletonBullet
            });
        }
        )
    }
}

pub fn cycle_color_child() -> impl Scene {
    bsn! {
        Cooldown::repeat(3000)
        on(|e: On<CooldownFinished>, mut commands: Commands,
            query: Query<&ChildOf>,
            mut cc_query: Query<&mut ColorCharge>| {
            let child_of = query.get(e.entity).unwrap();
            let parent = child_of.parent();
            if let Ok(mut cc) = cc_query.get_mut(parent) {
                *cc = cc.next();
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
        Self(2.)
    }
}

pub fn spin_y_ai(mut enemies: Query<(&mut Transform, &SpinY)>) {
    for (mut transform, spin) in enemies.iter_mut() {
        transform.rotate_y(spin.0.to_radians());
        transform.rotation = transform.rotation.normalize();
    }
}

#[derive(Component, Default, Clone)]
pub struct LookAtPlayer;

pub fn look_at_player(
    mut enemies: Query<(&mut Transform, &LookAtPlayer), Without<Player>>,
    player: Single<&Transform, With<Player>>,
) {
    for (mut transform, _) in enemies.iter_mut() {
        transform.look_at(player.translation, Vec3::Y);
    }
}

fn enemy_ai(_enemies: Query<&Transform, With<Enemy>>) {}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_enemy);
        app.add_systems(FixedUpdate, enemy_ai);
        app.add_systems(FixedUpdate, spin_y_ai);
        app.add_systems(FixedUpdate, look_at_player);
    }
}
