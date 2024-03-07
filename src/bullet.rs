use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{Bullet, Damage, LifeTime},
    utils::random_vector,
};

#[derive(Debug, Clone, Copy, Component)]
pub struct OriginPosition(pub Vec2);

pub fn fire_bullet(
    cmd: &mut Commands,
    accuracy: f32,
    originates_from: Vec3,
    shoot_at: Vec3,
    damage: Damage,
    lifetime: LifeTime,
    texture: Handle<Image>,
    bullet_speed: f32,
) {
    let acc_skew = random_vector() * accuracy;
    let vel = (shoot_at + acc_skew - originates_from)
        .truncate()
        .normalize()
        * bullet_speed;

    cmd.spawn((
        Bullet,
        OriginPosition(originates_from.truncate()),
        RigidBody::Dynamic,
        Sensor,
        Collider::ball(2.0),
        ActiveEvents::COLLISION_EVENTS,
        Velocity::linear(vel),
        damage,
        lifetime,
        SpriteBundle {
            transform: Transform::from_translation(originates_from),
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            ..default()
        },
        Name::new("Bullet"),
    ));
}
