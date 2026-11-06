use bevy::prelude::*;
#[derive(Component, Default, Clone)]
pub struct Velocity(pub Vec3);

fn apply_velocity(time: Res<Time>,
                  mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_velocity);
    }
}