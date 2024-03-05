use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{Bullet, Damage, Enemy, Health, IFrames, LifeTime, PickupRange, Player, Velocity},
};

#[derive(Debug, Resource)]
pub struct PlayerAttackTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.insert_resource(PlayerAttackTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, (move_player, shoot_bullets));
    }
}

fn spawn_player(mut cmd: Commands, asset_server: Res<AssetLoader>) {
    let texture = asset_server.player_sprite.clone();

    cmd.spawn((
        Player,
        PickupRange(32.),
        Health(1000.),
        IFrames::default(),
        Velocity::default(),
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0., 0., 10.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128., 128.)),
                ..default()
            },
            ..Default::default()
        },
    ));
}

fn move_player(
    mut query: Query<(&mut Velocity, &mut Sprite), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let player_speed = 50.;
    if let Ok((mut velocity, mut sprite)) = query.get_single_mut() {
        let mut new_vel = Vec2::default();

        if keys.pressed(KeyCode::KeyW) {
            new_vel.y = 1.;
        }
        if keys.pressed(KeyCode::KeyS) {
            new_vel.y = -1.;
        }

        if keys.pressed(KeyCode::KeyA) {
            new_vel.x = -1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            new_vel.x = 1.;
        }

        sprite.flip_x = new_vel.x < 0.0;

        velocity.0 = new_vel.normalize_or_zero() * player_speed;
    }
}

fn shoot_bullets(
    mut cmd: Commands,
    player_q: Query<&Transform, With<Player>>,
    enemy_q: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut attack_timer: ResMut<PlayerAttackTimer>,
    time: Res<Time>,
    asset_loader: Res<AssetLoader>,
) {
    if let Ok(player) = player_q.get_single() {
        if attack_timer.0.just_finished() {
            let mut closest = f32::MAX;
            let mut entity = None;
            enemy_q.iter().for_each(|e| {
                let dist = player.translation.distance(e.translation);
                if dist < closest {
                    closest = dist;
                    entity = Some(e);
                }
            });

            if let Some(entity) = entity {
                let bullet_speed = 2500.;
                let dt = time.delta_seconds();
                let vel = (entity.translation - player.translation).normalize_or_zero()
                    * dt
                    * bullet_speed;

                cmd.spawn(Bullet)
                    .insert(Damage(5.))
                    .insert(Velocity(vel.xy()))
                    .insert(LifeTime(420))
                    .insert(RigidBody::Dynamic)
                    .insert(Sensor)
                    .insert(Collider::ball(10.))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(SpriteBundle {
                        transform: Transform::from_translation(player.translation),
                        texture: asset_loader.bullet_sprite.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(32., 32.)),
                            ..default()
                        },
                        ..Default::default()
                    })
                    .insert(Name::new("Bullet"));
            }
        }
        attack_timer.0.tick(time.delta());
    }
}
