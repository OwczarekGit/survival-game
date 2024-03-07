use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{MainCamera, Player};

#[derive(Debug, Clone, Default, Resource)]
pub struct MousePosition {
    pub screen_position: Vec2,
    pub world_position: Vec2,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct MouseHighlightedAction(pub Option<Entity>);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera);
        app.add_systems(Update, (follow_player, update_mouse_screen_pos));
        app.init_resource::<MousePosition>();
        app.init_resource::<MouseHighlightedAction>();
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

fn update_mouse_screen_pos(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<PrimaryWindow>)>,
    mut mouse: ResMut<MousePosition>,
) {
    if let Ok(win) = window_q.get_single() {
        let cam = camera_q.single();
        mouse.screen_position = win.cursor_position().unwrap_or(Vec2::ZERO);

        let cursor_world = cam
            .0
            .viewport_to_world_2d(cam.1, mouse.screen_position)
            .unwrap_or_default();

        mouse.world_position = cursor_world;
    }
}
