use bevy::asset::io::embedded::GetAssetServer;
use bevy::color::palettes::basic::WHITE;
use bevy::color::palettes::css::RED;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::physics::*;
#[derive(Component, Default, Clone)]
#[require(BulletSpeed, ColorCharge)]
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

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform, ColorCharge)]
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
fn despawn_bullets(mut commands: Commands, mut bullet_query: Query<(&Bullet, &Transform, Entity)>) {
    for (bullet, transform, entity) in bullet_query.iter() {
        if transform.translation.x.abs() > 100. || transform.translation.z.abs() > 100. {
            commands.get_entity(entity).unwrap().despawn()
        }
    }
}

fn debug_bullets(mut commands: Commands,
                 mut gizmos: Gizmos,
                 bullet_query: Query<(&Bullet, &ColorCharge, &Transform)>
    ) {
    for (bullet, cc, transform) in bullet_query.iter() {
        gizmos.sphere(Isometry3d::new(transform.translation, transform.rotation), 0.5, cc.debug_color());
    }
}

fn debug_hitboxes(mut gizmos: Gizmos, hitboxes: Query<(&Hitbox, &ColorCharge, &Transform)>) {
    for (hitbox, cc, transform) in hitboxes.iter() {
        gizmos.sphere(Isometry3d::new(transform.translation, transform.rotation), hitbox.0, cc.debug_color());
    }
}

#[derive(Event)]
pub struct RetargetBullet {
    pub bullet: Entity,
    pub new_target: Entity,
    pub new_owner: Entity,
}

fn on_retarget_bullet(e: On<RetargetBullet>,
                      mut bullets: Query<(Entity, &mut BulletOwner, &mut BulletTarget, &mut Velocity, &Transform, &BulletSpeed)>,
                      hittable: Query<(Entity, &Hitbox, &Transform)>,
                      mut commands: Commands,) {
    let (bullet, mut b_owner, mut b_target, mut b_velocity,
        b_transform, b_speed) = bullets.get_mut(e.bullet).unwrap();
    b_owner.0 = e.new_owner;
    b_target.0 = e.new_target;
    let (_, _, h_transform) = hittable.get(e.new_target).unwrap();
    b_velocity.0 = aim_bullet(b_transform.translation, h_transform.translation) * b_speed.0;
}

pub struct BulletsPlugin;
impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App ) {
        let assets = app.get_asset_server();
        app.init_resource::<BulletAssets>();
        // app.add_systems(FixedUpdate, bullet_sys);
        app.add_systems(FixedUpdate, despawn_bullets);
        app.add_systems(FixedUpdate, debug_bullets);
        app.add_systems(FixedUpdate, debug_hitboxes);
        app.add_systems(FixedUpdate, bullet_collisions);
        app.add_observer(on_retarget_bullet);
    }
}
