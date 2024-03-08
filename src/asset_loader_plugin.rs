use bevy::prelude::*;

#[derive(Debug, Resource, Clone)]
pub struct AssetLoader {
    pub font: Handle<Font>,
    pub player_sprite: Handle<Image>,
    pub enemy_sprite: Handle<Image>,
    pub bullet_sprite: Handle<Image>,
    pub crystal_sprite: Handle<Image>,
    pub magnet_sprite: Handle<Image>,
    pub xp_sprite: Handle<Image>,
    pub spawner_sprite: Handle<Image>,
    pub death_sound: Handle<AudioSource>,
    pub damage_sound: Handle<AudioSource>,
    pub xp_pickup_sound: Handle<AudioSource>,
    pub pistol_shoot_sound: Handle<AudioSource>,

    // Tree
    pub tree_trunk_sprite: Handle<Image>,
    pub tree_main_sprite: Handle<Image>,
    pub attack_tree_sound: Handle<AudioSource>,
    pub tree_hit_ground_sound: Handle<AudioSource>,

    // Items
    pub item_wood_sprite: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_assets);
    }
}

fn init_assets(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font.ttf");
    let player_sprite = asset_server.load("player.png");
    let enemy_sprite = asset_server.load("enemy.png");
    let bullet_sprite = asset_server.load("bullet.png");
    let crystal_sprite = asset_server.load("crystal.png");
    let xp_sprite = asset_server.load("xp.png");
    let spawner_sprite = asset_server.load("spawner.png");
    let magnet_sprite = asset_server.load("magnet.png");
    let damage_sound = asset_server.load("damage.ogg");
    let death_sound = asset_server.load("death.ogg");
    let xp_pickup_sound = asset_server.load("xp_pickup.ogg");
    let pistol_shoot_sound = asset_server.load("pistol_fired.ogg");

    // Tree
    let tree_trunk_sprite = asset_server.load("tree-trunk.png");
    let tree_main_sprite = asset_server.load("tree-main.png");
    let attack_tree_sound = asset_server.load("attack-tree.ogg");
    let tree_hit_ground_sound = asset_server.load("tree-hit-ground.ogg");

    // Items
    let item_wood_sprite = asset_server.load("wood.png");

    cmd.insert_resource(AssetLoader {
        font,
        enemy_sprite,
        player_sprite,
        bullet_sprite,
        crystal_sprite,
        xp_sprite,
        spawner_sprite,
        magnet_sprite,
        damage_sound,
        death_sound,
        xp_pickup_sound,
        pistol_shoot_sound,
        tree_trunk_sprite,
        tree_main_sprite,
        attack_tree_sound,
        tree_hit_ground_sound,
        item_wood_sprite,
    });
}
