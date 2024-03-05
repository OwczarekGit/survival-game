use bevy::prelude::*;

#[derive(Debug, Resource, Clone)]
pub struct AssetLoader {
    pub player_sprite: Handle<Image>,
    pub enemy_sprite: Handle<Image>,
    pub bullet_sprite: Handle<Image>,
    pub crystal_sprite: Handle<Image>,
    pub xp_sprite: Handle<Image>,
    pub death_sound: Handle<AudioSource>,
    pub damage_sound: Handle<AudioSource>,
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
    let bullet_sprite = asset_server.load("bullet.png");
    let crystal_sprite = asset_server.load("crystal.png");
    let xp_sprite = asset_server.load("xp.png");
    let damage_sound = asset_server.load("damage.ogg");
    let death_sound = asset_server.load("death.ogg");

    cmd.insert_resource(AssetLoader {
        enemy_sprite,
        player_sprite,
        bullet_sprite,
        crystal_sprite,
        xp_sprite,
        damage_sound,
        death_sound,
    });
}
