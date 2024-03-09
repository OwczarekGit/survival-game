use aggressive_ai_plugin::AggressiveAiPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use asset_loader_plugin::AssetLoaderPlugin;
use camera_plugin::CameraPlugin;
use generic_plugin::GenericPlugin;
use pickup_plugin::PickupPlugin;
use player_plugin::PlayerPlugin;
use spawner_plugin::SpawnerPlugin;
use tree_plugin::TreePlugin;
use xp_plugin::XpPlugin;

mod aggressive_ai_plugin;
mod asset_loader_plugin;
mod bullet;
mod camera_plugin;
mod components;
mod events;
mod generic_plugin;
mod pickup_plugin;
mod player_plugin;
mod spawner_plugin;
mod tree_plugin;
mod utils;
mod xp_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GenericPlugin)
        .add_plugins(XpPlugin)
        .add_plugins(TreePlugin)
        .add_plugins(PickupPlugin)
        .add_plugins(SpawnerPlugin)
        .add_plugins(AggressiveAiPlugin)
        .insert_resource(ClearColor(Color::rgb_u8(33, 70, 33)))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .run()
}
