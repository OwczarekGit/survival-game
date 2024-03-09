use crate::{
    components::{PickupType, PlayerPickup},
    xp_plugin::{drop_xp, Xp},
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::Restitution,
};
use rand::Rng;

use crate::{
    asset_loader_plugin::AssetLoader,
    camera_plugin::{MouseHighlightedAction, MousePosition},
    components::{Gathering, Health, IFrames, MainCamera, Player},
    events::{ItemDropEvent, SoundEvent, TreeDiedEvent},
    utils::{chance_one_in, random_in_range, random_vector},
};

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct TreeTrunk;

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub enum TreeState {
    Standing,
    Falling,
    Dead,
}

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct Tree;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TreeDiedEvent>();
        app.register_type::<TreeState>();
        app.add_systems(Startup, spawn_trees);
        app.add_systems(
            Update,
            (select_tree, cut_tree, update_trees, handle_tree_death),
        );
    }
}

fn update_trees(
    mut tree_q: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &Health,
            &mut TreeState,
            Entity,
        ),
        With<Tree>,
    >,
    mut tree_died_event: EventWriter<TreeDiedEvent>,
) {
    for (mut t, gt, hp, mut state, e) in tree_q.iter_mut() {
        match *state {
            TreeState::Standing => {
                if hp.0 <= 0.0 {
                    *state = TreeState::Falling;
                }
            }
            TreeState::Falling => {
                t.rotate_z(0.04);
                if t.rotation.z.abs() > 0.7 {
                    *state = TreeState::Dead;
                }
            }
            TreeState::Dead => {
                tree_died_event.send(TreeDiedEvent(e, gt.translation(), 100.0));
            }
        }
    }
}

fn spawn_trees(mut cmd: Commands, asset_loader: Res<AssetLoader>) {
    let mut rng = rand::thread_rng();

    for x in -1000..1000 {
        for y in -1000..1000 {
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
                        Health(100.0, 100.0),
                        TreeState::Standing,
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
                })
                .insert(Name::new("Tree"));
            }
        }
    }
}

fn select_tree(
    player_q: Query<(&Transform, &Gathering), (With<Player>, Without<MainCamera>)>,
    tree_q: Query<(&GlobalTransform, Entity), (With<Tree>, Without<Player>, Without<MainCamera>)>,
    mouse: Res<MousePosition>,
    mut mouse_action: ResMut<MouseHighlightedAction>,
) {
    if let Ok((p_transform, p_range)) = player_q.get_single() {
        let cursor_world = mouse.world_position;
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

fn cut_tree(
    mut player_q: Query<&mut Gathering, (With<Player>, Without<Tree>)>,
    mut tree_q: Query<(&mut GlobalTransform, &mut IFrames, &mut Health, Entity), With<Tree>>,
    mut mouse_action: ResMut<MouseHighlightedAction>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut sound_event: EventWriter<SoundEvent>,
    mut drop_event: EventWriter<ItemDropEvent>,
) {
    if mouse_buttons.pressed(MouseButton::Right) && mouse_action.0.is_some() {
        for (t, mut iframes, mut hp, e) in tree_q.iter_mut() {
            let mut player = player_q.single_mut();
            if mouse_action.0.is_some()
                && mouse_action.0.unwrap() == e
                && iframes.0 <= 0.0
                && player.delay_frames <= 0.0
            {
                player.delay_frames = 0.30;
                iframes.0 = 0.30;
                hp.0 -= player.damage;
                sound_event.send(SoundEvent::AttackTree);
                mouse_action.0 = None;

                if chance_one_in(10.0) {
                    drop_event.send(ItemDropEvent::Wood(1, t.translation().truncate()));
                }
            }
        }
    }
}

fn handle_tree_death(
    mut cmd: Commands,
    mut tree_death_ev: EventReader<TreeDiedEvent>,
    mut sound_events: EventWriter<SoundEvent>,
    asset_loader: Res<AssetLoader>,
) {
    for TreeDiedEvent(e, pos, xp) in tree_death_ev.read() {
        let range = random_in_range(1.0, 5.0);
        for _i in 0..range as u32 {
            let mut vector = random_vector();
            vector *= random_in_range(-15.0, 15.0);

            drop_xp(
                &mut cmd,
                Xp(*xp),
                pos.truncate(),
                vector.truncate(),
                asset_loader.xp_sprite.clone(),
            );
        }

        drop_wood(
            &mut cmd,
            pos.truncate(),
            random_in_range(0.0, 10.0) as u32,
            asset_loader.item_wood_sprite.clone(),
        );

        if let Some(mut e) = cmd.get_entity(*e) {
            e.despawn();
        }
        sound_events.send(SoundEvent::TreeHitGround);
    }
    tree_death_ev.clear();
}

pub fn drop_wood(cmd: &mut Commands, position: Vec2, count: u32, texture: Handle<Image>) {
    for _ in 0..=count {
        let vector = random_vector() * random_in_range(-10.0, 10.0);

        cmd.spawn((
            PlayerPickup(PickupType::Item),
            RigidBody::Dynamic,
            Restitution::coefficient(5.0),
            Velocity::linear(vector.truncate()),
            SpriteBundle {
                transform: Transform::from_xyz(position.x, position.y - 50.0, 20.0),
                texture: texture.clone(),
                ..default()
            },
            Name::new("Wood"),
        ));
    }
}
