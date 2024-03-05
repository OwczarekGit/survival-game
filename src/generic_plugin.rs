use std::ops::Sub;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{Bullet, Damage, Enemy, Gathering, Health, IFrames, LifeTime},
    events::{SoundEvent, XpDropEvent},
};
use bevy::{audio::Volume, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct GenericPlugin;

impl Plugin for GenericPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>();
        app.add_systems(
            Update,
            (
                update_postions,
                tick_iframes,
                tick_lifetimes,
                bullet_enemy_collision,
                play_sound_event,
                tick_gathering,
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

fn tick_gathering(mut query: Query<&mut Gathering>) {
    for mut g in query.iter_mut() {
        g.delay_frames = (g.delay_frames - 0.01).max(0.0);
    }
}
fn play_sound_event(
    mut cmd: Commands,
    mut sound_event: EventReader<SoundEvent>,
    asset_loader: Res<AssetLoader>,
) {
    for ev in sound_event.read() {
        let (sound, volume) = match ev {
            SoundEvent::Damage => (asset_loader.damage_sound.clone(), 0.3),
            SoundEvent::Death => (asset_loader.death_sound.clone(), 0.2),
            SoundEvent::XpPickup => (asset_loader.xp_pickup_sound.clone(), 0.5),
            SoundEvent::AttackTree => (asset_loader.attack_tree_sound.clone(), 1.0),
        };

        cmd.spawn(AudioBundle {
            source: sound,
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(volume),
                ..Default::default()
            },
        });
    }
    sound_event.clear();
}

// I don't even...
fn bullet_enemy_collision(
    mut cmd: Commands,
    mut enemy_q: Query<(&Transform, &mut Health, &mut IFrames, Entity), With<Enemy>>,
    bullet_q: Query<(&Damage, Entity), (With<Bullet>, Without<Enemy>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut sound_event: EventWriter<SoundEvent>,
    mut xp_event: EventWriter<XpDropEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(a, b, _f) = collision_event {
            for (transform, mut hp, mut iframes, entity) in enemy_q.iter_mut() {
                if a == &entity {
                    for (dmg, bullet_e) in bullet_q.iter() {
                        if &bullet_e == b {
                            hp.0 = hp.0.sub(dmg.0);
                            iframes.0 = 0.2;
                            cmd.entity(bullet_e).despawn();

                            if hp.0 <= 0.0 {
                                cmd.entity(entity).despawn();
                                sound_event.send(SoundEvent::Death);
                                xp_event.send(XpDropEvent(transform.translation, 10.));
                            } else {
                                sound_event.send(SoundEvent::Damage);
                            }
                        }
                    }
                } else if b == &entity {
                    for (dmg, bullet_e) in bullet_q.iter() {
                        if &bullet_e == a {
                            hp.0 = hp.0.sub(dmg.0);
                            iframes.0 = 0.2;
                            cmd.entity(bullet_e).despawn();

                            if hp.0 <= 0.0 {
                                cmd.entity(entity).despawn();
                                sound_event.send(SoundEvent::Death);
                                xp_event.send(XpDropEvent(transform.translation, 10.));
                            } else {
                                sound_event.send(SoundEvent::Damage);
                            }
                        }
                    }
                }
            }
        }
    }
}
