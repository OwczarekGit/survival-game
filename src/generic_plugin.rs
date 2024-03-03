use std::ops::Sub;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{Bullet, Damage, Enemy, Health, IFrames, LifeTime},
    events::{DamageEvent, DeathEvent},
};
use bevy::{audio::Volume, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct GenericPlugin;

impl Plugin for GenericPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>();
        app.add_event::<DeathEvent>();
        app.add_systems(
            Update,
            (
                update_postions,
                tick_iframes,
                tick_lifetimes,
                bullet_enemy_collision,
                play_damage_event_sound,
                play_death_event_sound,
            ),
        );
    }
}

fn update_postions(
    mut query: Query<(&mut Transform, &crate::components::Velocity)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (mut t, v) in query.iter_mut() {
        t.translation.x += v.0.x * dt;
        t.translation.y += v.0.y * dt;
    }
}

fn tick_iframes(mut query: Query<(&mut IFrames, &mut Sprite)>) {
    for (mut iframes, mut sprite) in query.iter_mut() {
        iframes.0 = (iframes.0 - 0.01).max(0.0);

        sprite.color.set_g(1.0 - iframes.0 * 5.0);
        sprite.color.set_b(1.0 - iframes.0 * 5.0);
    }
}

fn tick_lifetimes(mut cmd: Commands, mut query: Query<(&mut LifeTime, Entity)>) {
    for (mut l, e) in query.iter_mut() {
        l.0 = l.0.saturating_sub(1);

        if l.0 == 0 {
            cmd.entity(e).despawn();
        }
    }
}

fn play_damage_event_sound(
    mut cmd: Commands,
    mut ev_damage: EventReader<DamageEvent>,
    asset_loader: Res<AssetLoader>,
) {
    if !ev_damage.is_empty() {
        cmd.spawn(AudioBundle {
            source: asset_loader.damage_sound.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(0.5),
                ..Default::default()
            },
            ..Default::default()
        });
        ev_damage.clear();
    }
}

fn play_death_event_sound(
    mut cmd: Commands,
    mut ev_death: EventReader<DeathEvent>,
    asset_loader: Res<AssetLoader>,
) {
    if !ev_death.is_empty() {
        cmd.spawn(AudioBundle {
            source: asset_loader.death_sound.clone(),
            settings: PlaybackSettings {
                volume: Volume::new(0.1),
                mode: bevy::audio::PlaybackMode::Despawn,
                ..Default::default()
            },
            ..Default::default()
        });
        ev_death.clear();
    }
}

fn bullet_enemy_collision(
    mut cmd: Commands,
    mut enemy_q: Query<(&mut Health, &mut IFrames, Entity), With<Enemy>>,
    bullet_q: Query<(&Damage, Entity), (With<Bullet>, Without<Enemy>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
    mut ev_death: EventWriter<DeathEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(a, b, _f) = collision_event {
            for (mut hp, mut iframes, entity) in enemy_q.iter_mut() {
                if a == &entity {
                    for (dmg, bullet_e) in bullet_q.iter() {
                        if &bullet_e == b {
                            hp.0 = hp.0.sub(dmg.0);
                            iframes.0 = 0.2;
                            cmd.entity(bullet_e).despawn();

                            if hp.0 <= 0.0 {
                                cmd.entity(entity).despawn();
                                ev_death.send(DeathEvent);
                            } else {
                                ev_damage.send(DamageEvent);
                            }
                        }
                    }
                }
            }
        }
    }
}
