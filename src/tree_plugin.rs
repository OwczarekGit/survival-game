use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader_plugin::AssetLoader,
    camera_plugin::{MouseHighlightedAction, MouseScreenPostion},
    components::{Gathering, IFrames, MainCamera, Player, Tree, TreeTrunk},
    events::SoundEvent,
};

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_trees);
        app.add_systems(Update, (select_tree, cut_tree));
    }
}

fn spawn_trees(mut cmd: Commands, asset_loader: Res<AssetLoader>) {
    let mut rng = rand::thread_rng();

    for x in -1000..1000 {
        for y in (-1000..1000) {
            if rng.gen_bool(1.0 / 1_000.0) {
                let pos = Vec3::new((x * 10) as f32, (y * 10) as f32, 7.0);

                cmd.spawn((
                    TreeTrunk,
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(64., 64.)),
                            ..default()
                        },
                        transform: Transform::from_translation(pos),
                        texture: asset_loader.tree_trunk_sprite.clone(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Tree,
                        IFrames(0.0),
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(64., 128.)),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 52.0, 0.0),
                            texture: asset_loader.tree_main_sprite.clone(),
                            ..default()
                        },
                    ));
                });
            }
        }
    }
}

fn select_tree(
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    player_q: Query<(&Transform, &Gathering), (With<Player>, Without<MainCamera>)>,
    tree_q: Query<(&GlobalTransform, Entity), (With<Tree>, Without<Player>, Without<MainCamera>)>,
    mouse: Res<MouseScreenPostion>,
    mut mouse_action: ResMut<MouseHighlightedAction>,
) {
    let (cam, cam_transform) = camera_q.single();
    if let Ok((p_transform, p_range)) = player_q.get_single() {
        if let Some(cursor_world) = cam.viewport_to_world_2d(cam_transform, mouse.0) {
            let mut closest_dist = f32::MAX;
            let mut closest_tree = None;

            for (tree, e) in tree_q.iter() {
                let dist = tree.translation().distance(cursor_world.extend(0.0));
                if dist < closest_dist && dist < 32.0 {
                    closest_dist = dist;
                    closest_tree = Some((tree, e));
                }
            }

            if let Some((tree, e)) = closest_tree {
                let dist_form_player = p_transform.translation.distance(tree.translation());
                if dist_form_player <= p_range.range {
                    mouse_action.0 = Some(e);
                }
            } else {
                mouse_action.0 = None;
            }
        }
    }
}

fn cut_tree(
    mut player_q: Query<&mut Gathering, (With<Player>, Without<Tree>)>,
    mut tree_q: Query<(&mut Transform, &mut IFrames, Entity), With<Tree>>,
    mut mouse_action: ResMut<MouseHighlightedAction>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut sound_event: EventWriter<SoundEvent>,
) {
    if mouse_buttons.pressed(MouseButton::Left) && mouse_action.0.is_some() {
        for (_t, mut iframes, e) in tree_q.iter_mut() {
            let mut player = player_q.single_mut();
            if mouse_action.0.is_some()
                && mouse_action.0.unwrap() == e
                && iframes.0 <= 0.0
                && player.delay_frames <= 0.0
            {
                player.delay_frames = 0.30;
                iframes.0 = 0.30;
                sound_event.send(SoundEvent::AttackTree);
                mouse_action.0 = None;
            }
        }
    }
}
