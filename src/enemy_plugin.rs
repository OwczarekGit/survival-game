use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{Enemy, Health, Player, Velocity},
};

#[derive(Debug, Resource)]
struct EnemySpawnTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, spawn_enemy);
        app.add_systems(Update, attack_player);
    }
}

fn attack_player(
    mut enemy_q: Query<(&mut Velocity, &Transform), With<Enemy>>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let enemy_speed = 250.0;
    let dt = time.delta_seconds();
    if let Ok(player) = player_q.get_single() {
        for (mut v, t) in enemy_q.iter_mut() {
            let vector = (player.translation - t.translation).normalize_or_zero();
            v.0.x = vector.x * dt * enemy_speed;
            v.0.y = vector.y * dt * enemy_speed;
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
            rng.gen_range(-1000..1000) as f32,
            rng.gen_range(-1000..1000) as f32,
            20.,
        );

        cmd.spawn((
            Enemy,
            Velocity(Vec2::new(1., 0.)),
            Health(10.),
            SpriteBundle {
                transform,
                texture,
                ..Default::default()
            },
        ));
    }

    spawn_timer.0.tick(timer.delta());
}
