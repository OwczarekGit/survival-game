use bevy::prelude::*;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{Health, Player, Velocity},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, move_player);
    }
}

fn spawn_player(mut cmd: Commands, asset_server: Res<AssetLoader>) {
    let texture = asset_server.player_sprite.clone();

    cmd.spawn((
        Player,
        Health(1000.),
        Velocity::default(),
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0., 0., 10.),
            ..Default::default()
        },
    ));
}

fn move_player(mut query: Query<&mut Velocity, With<Player>>, keys: Res<ButtonInput<KeyCode>>) {
    let player_speed = 50.;
    if let Ok(mut player) = query.get_single_mut() {
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

        player.0 = new_vel.normalize_or_zero() * player_speed;
    }
}
