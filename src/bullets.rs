use bevy::asset::io::embedded::GetAssetServer;
use bevy::color::palettes::basic::WHITE;
use bevy::color::palettes::css::RED;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::physics::Velocity;
#[derive(Component, Default, Clone)]
#[require(BulletSpeed)]
pub struct Bullet;

#[derive(Component, Clone)]
pub struct BulletSpeed(pub f32);
impl Default for BulletSpeed {
    fn default() -> Self {
        BulletSpeed(20.0)
    }
}



#[derive(Resource)]
struct BulletAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_asset_server();
        let mesh = assets.add(Sphere::new(0.5).into());
        let material = assets.add(Color::srgb(1.0, 0.0, 0.0).into());
        BulletAssets { mesh, material }
    }
}

#[derive(Component, Clone)]
pub struct BulletOwner(pub Entity);
impl Default for BulletOwner {
    fn default() -> Self {
        BulletOwner(Entity::PLACEHOLDER)
    }
}

#[derive(Component, Clone)]
pub struct BulletTarget(pub Entity);
impl Default for BulletTarget {
    fn default() -> Self {
        BulletTarget(Entity::PLACEHOLDER)
    }
}

#[derive(Component, Clone)]
#[require(Transform)]
pub struct Hitbox(pub f32);
impl Default for Hitbox {
    fn default() -> Self {
        Hitbox(0.5)
    }
}

#[derive(EntityEvent)]
pub struct BulletHit {
    #[event_target]
    pub target: Entity,
    pub bullet_entity: Entity,
}

fn bullet_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &BulletOwner, &BulletTarget), With<Bullet>>,
    hitbox_query: Query<(&Hitbox, &Transform), Without<Bullet>>,
) {
    for (bullet, b_transform, owner, target) in bullet_query.iter() {
        // let e = targeting.0;
        let (hitbox, t_transform) = hitbox_query.get(target.0).unwrap();

        let hit = b_transform.translation.distance_squared(t_transform.translation) <= hitbox.0 * hitbox.0;
        if hit {
            commands.trigger(BulletHit { target: target.0, bullet_entity: bullet });
            //commands.entity(bullet).despawn();
        }
    }
}

pub fn aim_bullet(origin: Vec3, target: Vec3) -> Vec3 {
    (target - origin).normalize_or_zero()
}

fn bullet_sys(mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    bullet_assets: Res<BulletAssets>,) {
    let mesh = bullet_assets.mesh.clone();
    let material = bullet_assets.material.clone();

    let origin = Transform::from_xyz(10.0, 0.0, 0.0);
    let speed = 20.0;
    let velocity = aim_bullet(origin.translation, Vec3::ZERO) * speed;

    if keyboard.pressed(KeyCode::Space) {
        commands.spawn_scene(bsn! {
            Bullet
            Transform::from_xyz(10.0, 0.0, 0.0)
            Velocity(velocity)
            Mesh3d(mesh)
            MeshMaterial3d<StandardMaterial>(material)
        });
    }
}
// fn bullet_sys(mut commands: Commands,
//               mut bullet_query: Query<(&Bullet, &Trajectory, &Velocity)>) {
//
// }

fn despawn_bullets(mut commands: Commands, mut bullet_query: Query<(&Bullet, &Transform, Entity)>) {
    for (bullet, transform, entity) in bullet_query.iter() {
        if transform.translation.x.abs() > 100. || transform.translation.z.abs() > 100. {
            commands.get_entity(entity).unwrap().despawn()
        }
    }
}

fn debug_bullets(mut commands: Commands,
                 mut gizmos: Gizmos,
                 bullet_query: Query<(&Bullet, &Transform)>
    ) {
    for (bullet, transform) in bullet_query.iter() {
        gizmos.sphere(Isometry3d::new(transform.translation, transform.rotation), 0.5, RED);
    }
}

fn debug_hitboxes(mut gizmos: Gizmos, hitboxes: Query<(&Hitbox, &Transform)>) {
    for (hitbox, transform) in hitboxes.iter() {
        gizmos.sphere(Isometry3d::new(transform.translation, transform.rotation), hitbox.0, WHITE);
    }
}

pub struct BulletsPlugin;
impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App ) {
        let assets = app.get_asset_server();
        app.init_resource::<BulletAssets>();
        app.add_systems(FixedUpdate, bullet_sys);
        app.add_systems(FixedUpdate, despawn_bullets);
        app.add_systems(FixedUpdate, debug_bullets);
        app.add_systems(FixedUpdate, debug_hitboxes);
        app.add_systems(FixedUpdate, bullet_collisions);
    }
}
