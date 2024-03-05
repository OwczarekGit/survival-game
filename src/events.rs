use bevy::prelude::*;

#[derive(Debug, Event)]
pub enum SoundEvent {
    Damage,
    Death,
    XpPickup,
}

#[derive(Debug, Default, Event, Clone)]
pub struct XpDropEvent(pub Vec3, pub f32);
