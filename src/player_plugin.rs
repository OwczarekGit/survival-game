use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    aggressive_ai_plugin::{AggressiveAi, AggressiveAiState},
    asset_loader_plugin::AssetLoader,
    bullet::fire_bullet,
    camera_plugin::MousePosition,
    components::{
        Damage, Enemy, Gathering, Health, IFrames, LifeTime, MainCamera, PickupRange, Player,
        UiLevelDisplayBar, UiLevelDisplayNumber,
    },
    events::SoundEvent,
    xp_plugin::xp_level::XpLevel,
};

#[derive(Debug, Resource)]
pub struct PlayerAttackTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.insert_resource(PlayerAttackTimer(Timer::from_seconds(0.5, TimerMode::Once)));
        app.add_systems(Update, move_player);
        app.add_systems(Update, shoot_bullets);
        app.add_systems(Update, kill_mode);
    }
}

fn spawn_player(mut cmd: Commands, asset_loader: Res<AssetLoader>) {
    let texture = asset_loader.player_sprite.clone();

    cmd.spawn((
        Player,
        RigidBody::Dynamic,
        Gathering {
            damage: 20.0,
            range: 64.0,
            delay_frames: 0.0,
        },
        XpLevel::with_level(1),
        PickupRange(32.),
        Health(1000., 1000.),
        IFrames::default(),
        Velocity::linear(Vec2 { x: 0.0, y: 0.0 }),
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0., 0., 10.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128., 128.)),
                ..default()
            },
            ..Default::default()
        },
        Name::new("Player"),
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

        velocity.linvel = new_vel.normalize_or_zero() * player_speed;
    }
}

fn shoot_bullets(
    mut cmd: Commands,
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut attack_timer: ResMut<PlayerAttackTimer>,
    time: Res<Time>,
    asset_loader: Res<AssetLoader>,
    keys: Res<ButtonInput<MouseButton>>,
    mouse: Res<MousePosition>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    attack_timer.0.tick(time.delta());
    if let Ok(player) = player_q.get_single() {
        if attack_timer.0.finished() && keys.pressed(MouseButton::Left) {
            const BULLET_SPEED: f32 = 1_000.0;

            fire_bullet(
                &mut cmd,
                20.0,
                player.translation,
                mouse.world_position.extend(player.translation.z),
                Damage(2.0),
                LifeTime(120),
                asset_loader.bullet_sprite.clone(),
                BULLET_SPEED,
            );

            sound_events.send(SoundEvent::PistolShoot);
            attack_timer.0.reset();
        }
    }
}

fn kill_mode(mut enemy_q: Query<&mut AggressiveAi, With<Enemy>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Backspace) {
        for mut e in enemy_q.iter_mut() {
            e.state = AggressiveAiState::KillMode;
        }
    }
}
