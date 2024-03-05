use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{Enemy, Health, IFrames, Player, Velocity},
};

#[derive(Debug, Resource)]
struct EnemySpawnTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, spawn_enemy);
        app.add_systems(Update, attack_player);
    }
}

fn attack_player(
    mut enemy_q: Query<(&mut Velocity, &Transform, &mut Sprite), With<Enemy>>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let enemy_speed = 250.0;
    let dt = time.delta_seconds();
    if let Ok(player) = player_q.get_single() {
        for (mut v, t, mut s) in enemy_q.iter_mut() {
            let vector = (player.translation - t.translation).normalize_or_zero();
            v.0.x = vector.x * dt * enemy_speed;
            v.0.y = vector.y * dt * enemy_speed;
            s.flip_x = v.0.x > 0.0;
        }
    }
}

fn spawn_enemy(
    mut cmd: Commands,
    asset_server: Res<AssetLoader>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    timer: Res<Time>,
) {
    if spawn_timer.0.just_finished() {
        let texture = asset_server.enemy_sprite.clone();

        let mut rng = rand::thread_rng();

        let transform = Transform::from_xyz(
            rng.gen_range(-100..100) as f32,
            rng.gen_range(-100..100) as f32,
            10.,
        );

        cmd.spawn(Enemy)
            .insert(RigidBody::Dynamic)
            .insert(Collider::capsule_y(44., 12.))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Sensor)
            .insert(IFrames::default())
            .insert(Velocity::default())
            .insert(Health(10.))
            .insert(SpriteBundle {
                transform,
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128., 128.)),
                    ..default()
                },
                ..Default::default()
            })
            .insert(Name::new("Enemy"));
    }

    spawn_timer.0.tick(timer.delta());
}
