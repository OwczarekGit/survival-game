use bevy::prelude::*;

use crate::components::{MainCamera, Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera);
        app.add_systems(Update, follow_player);
    }
}

fn create_camera(mut cmd: Commands) {
    cmd.spawn((
        MainCamera,
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 4. / 10.,
                near: 100.,
                far: -100.,
                ..default()
            },
            ..default()
        },
    ));
}

fn follow_player(
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    if let Ok(player) = player_q.get_single() {
        let mut camera = camera_q.single_mut();
        camera.translation = player.translation;
    }
}
