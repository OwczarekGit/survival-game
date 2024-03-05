use bevy::prelude::*;

use crate::components::PickupType;

#[derive(Debug, Event)]
pub enum SoundEvent {
    Damage,
    Death,
    XpPickup,
    AttackTree,
    TreeHitGround,
}

#[derive(Debug, Event)]
pub struct TreeDiedEvent(pub Entity, pub Vec3, pub f32);

#[derive(Debug, Default, Event, Clone)]
pub struct XpDropEvent(pub Vec3, pub f32);

#[derive(Debug, Event, Clone)]
pub struct PickupTakenEvent(pub Entity, pub PickupType);
