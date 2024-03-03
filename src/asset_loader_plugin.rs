use bevy::prelude::*;

#[derive(Debug, Resource, Clone)]
pub struct AssetLoader {
    pub player_sprite: Handle<Image>,
    pub enemy_sprite: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_assets);
    }
}

fn init_assets(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let player_sprite = asset_server.load("player.png");
    let enemy_sprite = asset_server.load("enemy.png");

    cmd.insert_resource(AssetLoader {
        enemy_sprite,
        player_sprite,
    });
}
