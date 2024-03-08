use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Player;

#[derive(Debug, Clone, Component)]
pub struct Bullet;

#[derive(Debug, Clone, Component)]
pub struct Enemy;

#[derive(Debug, Clone, Component, Reflect)]
pub struct PlayerPickup(pub PickupType);

#[derive(Debug, Clone, Component)]
pub struct MainCamera;

#[derive(Debug, Clone, Component)]
pub struct UiLevelDisplayNumber;

#[derive(Debug, Clone, Component)]
pub struct UiLevelDisplayBar;

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Health(pub f32, pub f32);

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Damage(pub f32);

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Xp(pub f32);

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct PickupRange(pub f32);

#[derive(Debug, Clone, Default, Component)]
pub struct Acceleration(pub Vec2);

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
pub struct IFrames(pub f32);

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
pub struct LifeTime(pub u32);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct Tree;

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct AttractedToPlayer;

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct TreeTrunk;

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct Magnet;

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub enum TreeState {
    Standing,
    Falling,
    Dead,
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
pub struct Gathering {
    pub damage: f32,
    pub range: f32,
    pub delay_frames: f32,
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub enum PickupType {
    Magnet,
    Item,
}
