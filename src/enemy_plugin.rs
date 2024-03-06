use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Enemy, Player};

#[derive(Debug, Resource)]
struct EnemySpawnTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack_player);
    }
}

fn attack_player(
    mut enemy_q: Query<(&mut Velocity, &Transform, &mut Sprite), With<Enemy>>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    const ENEMY_SPEED: f32 = 60.0;
    if let Ok(player) = player_q.get_single() {
        for (mut v, t, mut s) in enemy_q.iter_mut() {
            let vector = (player.translation - t.translation).normalize_or_zero();
            v.linvel.x = vector.x * ENEMY_SPEED;
            v.linvel.y = vector.y * ENEMY_SPEED;
            s.flip_x = v.linvel.x > 0.0;
        }
    }
}
