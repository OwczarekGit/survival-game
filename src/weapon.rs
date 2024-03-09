use bevy::prelude::*;

use crate::components::Damage;

#[derive(Debug, Clone, Component)]
pub struct Weapon {
    pub delay: Timer,
    pub damage: Damage,
    pub bullet_velocity: f32,
    pub accuracy: f32,
}

impl Weapon {
    pub fn new(damage: Damage, bullet_velocity: f32, accuracy: f32, fire_delay_secs: f32) -> Self {
        Self {
            delay: Timer::from_seconds(fire_delay_secs, TimerMode::Once),
            bullet_velocity,
            accuracy,
            damage,
        }
    }

    fn update(&mut self, dt: &Time) {
        self.delay.tick(dt.delta());
    }

    pub fn fire(&mut self, dt: &Time) -> bool {
        self.update(dt);

        if self.delay.finished() {
            self.delay.reset();
            return true;
        }

        false
    }
}
