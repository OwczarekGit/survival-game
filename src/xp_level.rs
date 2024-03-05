use bevy::prelude::*;

use crate::components::Xp;

#[derive(Debug, Clone, Default, Component)]
pub struct XpLevel {
    pub xp: f32,
    pub xp_to_next: f32,
    pub level: u32,
}

impl XpLevel {
    pub fn with_level(level: u32) -> Self {
        let xp_to_next = level as f32 * 100.;
        Self {
            xp: 0.,
            xp_to_next,
            level,
        }
    }

    pub fn add_xp(&mut self, Xp(xp): Xp) {
        let xp_now = xp + self.xp;

        self.level = self.level + (xp_now / self.xp_to_next) as u32;
        self.xp = xp_now % self.xp_to_next;
        self.xp_to_next = self.level as f32 * 100.;
    }
}
