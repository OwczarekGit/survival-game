use asset_loader_plugin::AssetLoaderPlugin;
use bevy::prelude::*;
use camera_plugin::CameraPlugin;
use enemy_plugin::EnemyPlugin;
use generic_plugin::GenericPlugin;
use player_plugin::PlayerPlugin;

mod asset_loader_plugin;
mod camera_plugin;
mod components;
mod enemy_plugin;
mod generic_plugin;
mod player_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(GenericPlugin)
        .run()
}
