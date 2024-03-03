use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use asset_loader_plugin::AssetLoaderPlugin;
use camera_plugin::CameraPlugin;
use enemy_plugin::EnemyPlugin;
use generic_plugin::GenericPlugin;
use player_plugin::PlayerPlugin;

mod asset_loader_plugin;
mod camera_plugin;
mod components;
mod enemy_plugin;
mod events;
mod generic_plugin;
mod player_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(GenericPlugin)
        .run()
}
