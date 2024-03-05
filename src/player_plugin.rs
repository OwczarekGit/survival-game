use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{
        Bullet, Damage, Enemy, Gathering, Health, IFrames, LifeTime, PickupRange, Player,
        UiLevelDisplayBar, UiLevelDisplayNumber, Velocity,
    },
    xp_level::XpLevel,
};

#[derive(Debug, Resource)]
pub struct PlayerAttackTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.insert_resource(PlayerAttackTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, (move_player, shoot_bullets));
    }
}

fn spawn_player(mut cmd: Commands, asset_loader: Res<AssetLoader>) {
    let texture = asset_loader.player_sprite.clone();

    cmd.spawn((
        Player,
        Gathering {
            damage: 20.0,
            range: 64.0,
            delay_frames: 0.0,
        },
        XpLevel::with_level(1),
        PickupRange(32.),
        Health(1000., 1000.),
        IFrames::default(),
        Velocity::default(),
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0., 0., 10.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128., 128.)),
                ..default()
            },
            ..Default::default()
        },
    ));

    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Grid,
            justify_content: JustifyContent::SpaceBetween,
            grid_template_columns: vec![GridTrack::px(64.0), GridTrack::px(128.0)],
            column_gap: Val::Px(8.),
            padding: UiRect::all(Val::Px(16.)),
            ..default()
        },
        ..default()
    })
    .insert(Name::new("Xp UI"))
    .with_children(|parent| {
        parent.spawn((
            UiLevelDisplayNumber,
            TextBundle::from_section(
                "1",
                TextStyle {
                    font: asset_loader.font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
        ));

        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                Outline {
                    color: Color::WHITE,
                    offset: Val::Px(2.0),
                    width: Val::Px(4.0),
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    UiLevelDisplayBar,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(0.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::hex("69BD30").unwrap()),
                        ..default()
                    },
                ));
            });
    });
}

fn move_player(
    mut query: Query<(&mut Velocity, &mut Sprite), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let player_speed = 100.;
    if let Ok((mut velocity, mut sprite)) = query.get_single_mut() {
        let mut new_vel = Vec2::default();

        if keys.pressed(KeyCode::KeyW) {
            new_vel.y = 1.;
        }
        if keys.pressed(KeyCode::KeyS) {
            new_vel.y = -1.;
        }

        if keys.pressed(KeyCode::KeyA) {
            new_vel.x = -1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            new_vel.x = 1.;
        }

        sprite.flip_x = new_vel.x < 0.0;

        velocity.0 = new_vel.normalize_or_zero() * player_speed;
    }
}

fn shoot_bullets(
    mut cmd: Commands,
    player_q: Query<&Transform, With<Player>>,
    enemy_q: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut attack_timer: ResMut<PlayerAttackTimer>,
    time: Res<Time>,
    asset_loader: Res<AssetLoader>,
) {
    if let Ok(player) = player_q.get_single() {
        if attack_timer.0.just_finished() {
            let mut closest = f32::MAX;
            let mut entity = None;
            enemy_q.iter().for_each(|e| {
                let dist = player.translation.distance(e.translation);
                if dist < closest {
                    closest = dist;
                    entity = Some(e);
                }
            });

            if let Some(entity) = entity {
                let bullet_speed = 6000.;
                let dt = time.delta_seconds();
                let vel = (entity.translation - player.translation).normalize_or_zero()
                    * dt
                    * bullet_speed;

                cmd.spawn(Bullet)
                    .insert(Damage(5.))
                    .insert(Velocity(vel.xy()))
                    .insert(LifeTime(420))
                    .insert(RigidBody::Dynamic)
                    .insert(Sensor)
                    .insert(Collider::ball(2.))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(SpriteBundle {
                        transform: Transform::from_translation(player.translation),
                        texture: asset_loader.bullet_sprite.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(8., 8.)),
                            ..default()
                        },
                        ..Default::default()
                    })
                    .insert(Name::new("Bullet"));
            }
        }
        attack_timer.0.tick(time.delta());
    }
}
