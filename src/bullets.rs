use bevy::asset::io::embedded::GetAssetServer;
use bevy::color::palettes::basic::WHITE;
use bevy::color::palettes::css::RED;
use bevy::ecs::template::TemplateTuple;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::physics::*;

#[derive(Resource)]
struct BulletAssets {
    pub mesh: Handle<Mesh>,
    red: Handle<StandardMaterial>,
    green: Handle<StandardMaterial>,
    blue: Handle<StandardMaterial>,
    white: Handle<StandardMaterial>,
}

impl BulletAssets {
    pub fn material(&self, cc: ColorCharge) -> Handle<StandardMaterial> {
        match cc {
            ColorCharge::RED => self.red.clone(),
            ColorCharge::GREEN => self.green.clone(),
            ColorCharge::BLUE => self.blue.clone(),
            ColorCharge::WHITE => self.white.clone(),
            _ => unreachable!(),
        }
    }
}

fn bullet_base_color(cc: &ColorCharge) -> Color {
    match cc {
        ColorCharge::RED => Color::srgb(1.0, 0.1, 0.1),
        ColorCharge::GREEN => Color::srgb(0.1, 1.0, 0.1),
        ColorCharge::BLUE => Color::srgb(0.1, 0.1, 1.0),
        ColorCharge::WHITE => Color::srgb(1.0, 1.0, 1.0),
        _ => unreachable!(),
    }
}
const BULLET_GLOW: f32 = 8.0;

fn bullet_emissive(cc: &ColorCharge) -> LinearRgba {
    let color = bullet_base_color(cc).to_linear();
    let strength = if matches!(cc, ColorCharge::WHITE) {
        BULLET_GLOW * 0.6
    } else {
        BULLET_GLOW
    };

    LinearRgba::rgb(color.red * strength, color.green * strength, color.blue * strength)
}

fn bullet_material(cc: &ColorCharge) -> StandardMaterial {
   StandardMaterial {
       base_color: bullet_base_color(cc),
       emissive: bullet_emissive(cc),
       unlit: true,
       ..default()
   }
}

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        let mesh = world.resource_mut::<Assets<Mesh>>().add(Sphere::new(0.5));
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        Self {
            mesh,
            red: materials.add(bullet_material(&ColorCharge::RED)),
            green: materials.add(bullet_material(&ColorCharge::GREEN)),
            blue: materials.add(bullet_material(&ColorCharge::BLUE)),
            white: materials.add(bullet_material(&ColorCharge::WHITE)),
        }
    }
}

#[derive(SceneComponent, Default, Clone)]
#[scene(BulletProps)]
pub struct Bullet;

pub struct BulletProps {
    pub target: Entity,
    pub owner: Entity,
    pub cc: ColorCharge,
}
impl Default for BulletProps {
    fn default() -> Self {
        BulletProps {
            target: Entity::PLACEHOLDER,
            owner: Entity::PLACEHOLDER,
            cc: ColorCharge::RED,
        }
    }
}

impl Bullet {
    fn scene(props: BulletProps) -> impl Scene {
        let BulletProps { target, owner , cc} = props;
        let material_cc = cc.clone();
        bsn! {
            BulletSpeed
            BulletTarget(target)
            BulletOwner(owner)
            template(move |context| {
                let material = context.resource::<BulletAssets>().material(material_cc);
                Ok(
                    MeshMaterial3d(material)
                )
            })
            template(|context| {
                let mesh = context.resource::<BulletAssets>().mesh.clone();
                Ok(Mesh3d(mesh))
            })
            cc
        }
    }
}

#[derive(Component, Clone)]
pub struct BulletSpeed(pub f32);
impl Default for BulletSpeed {
    fn default() -> Self {
        BulletSpeed(20.0)
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
        let Ok((hitbox, t_transform)) = hitbox_query.get(target.0) else {
            continue;
        };

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
        app.add_systems(FixedUpdate, despawn_bullets);
        app.add_systems(FixedUpdate, debug_bullets);
        app.add_systems(FixedUpdate, debug_hitboxes);
        app.add_systems(FixedUpdate, bullet_collisions);
        app.add_observer(on_retarget_bullet);
    }
}
