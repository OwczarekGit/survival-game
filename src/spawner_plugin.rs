use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{Enemy, Health, IFrames, Player, Velocity},
    utils::random_vector,
};

#[derive(Debug, Component)]
pub struct Spawner;

#[derive(Debug, Component)]
pub struct SpawnerId(pub Entity);

#[derive(Debug, Event)]
pub struct SpawnedEntiyDeathEvent(pub Entity);

#[derive(Debug, Component)]
pub struct SpawnerSpawnTimer {
    pub timer: Timer,
    pub spawn_limit: u32,
    pub alive_now: u32,
}

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnedEntiyDeathEvent>();
        app.add_systems(
            Update,
            (spawn_spawners, spawner_tick, handle_spawned_entity_death),
        );
    }
}

fn spawner_tick(
    mut cmd: Commands,
    mut spawner_q: Query<(&Transform, &mut SpawnerSpawnTimer, Entity), With<Spawner>>,
    time: Res<Time>,
    assets: Res<AssetLoader>,
) {
    for (t, mut timer, e) in spawner_q.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.just_finished() && timer.alive_now < timer.spawn_limit {
            cmd.spawn(Enemy)
                .insert(RigidBody::Dynamic)
                .insert(Collider::capsule_y(44., 12.))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Sensor)
                .insert(IFrames::default())
                .insert(Velocity::default())
                .insert(Health(10., 10.))
                .insert(SpawnerId(e))
                .insert(SpriteBundle {
                    transform: Transform::from_translation(t.translation),
                    texture: assets.enemy_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(128., 128.)),
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Enemy"));

            timer.alive_now += 1;
        }
    }
}

fn spawn_spawners(
    mut cmd: Commands,
    player_q: Query<&Transform, With<Player>>,
    spawner_q: Query<Entity, (With<Spawner>, Without<Player>)>,
    assets: Res<AssetLoader>,
) {
    const MAX_SPAWNERS: usize = 10;

    let spawners_count = spawner_q.iter().len();

    if spawners_count < MAX_SPAWNERS {
        let mut rng = rand::thread_rng();

        let t = player_q.single().translation;

        let vector = random_vector() * rng.gen_range(2000.0..8000.0);

        cmd.spawn((
            Spawner,
            SpawnerSpawnTimer {
                timer: Timer::from_seconds(10.0, TimerMode::Repeating),
                spawn_limit: 16,
                alive_now: 0,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                texture: assets.spawner_sprite.clone(),
                transform: Transform::from_translation(t + vector),
                ..default()
            },
            Name::new("Spawner"),
        ));
    }
}

fn handle_spawned_entity_death(
    mut cmd: Commands,
    mut spawner_q: Query<(&mut SpawnerSpawnTimer, Entity), With<Spawner>>,
    mut events: EventReader<SpawnedEntiyDeathEvent>,
) {
    for SpawnedEntiyDeathEvent(e) in events.read() {
        if let Some(e) = cmd.get_entity(*e) {
            for (mut spawner, spawner_e) in spawner_q.iter_mut() {
                if spawner_e == e.id() {
                    spawner.alive_now = spawner.alive_now.saturating_sub(1);
                    dbg!(spawner.alive_now);
                }
            }
        }
    }
    events.clear();
}
