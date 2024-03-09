use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{Bullet, Damage, LifeTime},
    utils::random_vector,
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>();
        app.add_systems(Update, handle_bullet_fired_event);
    }
}

#[derive(Debug, Clone, Event)]
pub struct BulletFiredEvent {
    pub from: Vec2,
    pub at: Vec2,
    pub acc: f32,
    pub dmg: Damage,
    pub lifetime: LifeTime,
    pub bullet_speed: f32,
}

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

fn handle_bullet_fired_event(
    mut cmd: Commands,
    mut ev: EventReader<BulletFiredEvent>,
    asset_loader: Res<AssetLoader>,
) {
    for e in ev.read() {
        fire_bullet(
            &mut cmd,
            e.acc,
            e.from.extend(10.0),
            e.at.extend(10.0),
            e.dmg,
            e.lifetime,
            asset_loader.bullet_sprite.clone(),
            e.bullet_speed,
        );
    }

    ev.clear();
}
