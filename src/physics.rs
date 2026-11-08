use std::time::Duration;
use bevy::color::palettes::basic::{BLACK, BLUE, GREEN, WHITE};
use bevy::color::palettes::css::{GRAY, RED};
use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Velocity(pub Vec3);

fn apply_velocity(time: Res<Time>,
                  mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

#[derive(Component, Default, Copy, Clone, Reflect, PartialEq)]
#[reflect(Component)]
pub enum ColorCharge {
    #[default]
    NONE,
    RED,
    GREEN,
    BLUE,
    WHITE,
    BLACK,
}

impl ColorCharge {
    pub(crate) fn debug_color(&self) -> Color {
        match self {
            ColorCharge::NONE => GRAY.into(),
            ColorCharge::RED => RED.into(),
            ColorCharge::GREEN => GREEN.into(),
            ColorCharge::BLUE => BLUE.into(),
            ColorCharge::WHITE => WHITE.into(),
            ColorCharge::BLACK => BLACK.into(),
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct Cooldown(pub Timer);

#[derive(EntityEvent)]
pub struct CooldownFinished {
    pub entity: Entity,
}

impl Cooldown {
    pub fn once(millis:u64) -> Self {
        let mut t = Timer::new(Duration::from_millis(millis), TimerMode::Once);
        t.finish();
        Self(t)
    }
    pub fn repeat(millis:u64) -> Self {
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

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, tick_cooldowns);
        app.add_systems(FixedUpdate, apply_velocity);
    }
}
