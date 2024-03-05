use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Player;

#[derive(Debug, Clone, Component)]
pub struct Bullet;

#[derive(Debug, Clone, Component)]
pub struct Enemy;

#[derive(Debug, Clone, Component)]
pub struct MainCamera;

#[derive(Debug, Clone, Component)]
pub struct UiLevelDisplayNumber;

#[derive(Debug, Clone, Component)]
pub struct UiLevelDisplayBar;

#[derive(Debug, Clone, Copy, Component)]
pub struct Health(pub f32, pub f32);

#[derive(Debug, Clone, Copy, Component)]
pub struct Damage(pub f32);

#[derive(Debug, Clone, Copy, Component)]
pub struct Xp(pub f32);

#[derive(Debug, Clone, Copy, Component)]
pub struct PickupRange(pub f32);

#[derive(Debug, Clone, Default, Component)]
pub struct Acceleration(pub Vec2);

#[derive(Debug, Clone, Default, Component)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct IFrames(pub f32);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct LifeTime(pub u32);
