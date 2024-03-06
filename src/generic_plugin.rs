use std::ops::Sub;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{
        AttractedToPlayer, Bullet, Damage, Enemy, Gathering, Health, IFrames, LifeTime, Player, Xp,
    },
    events::{SoundEvent, TreeDiedEvent, XpDropEvent},
    spawner_plugin::{SpawnedEntiyDeathEvent, SpawnerId},
    utils::random_vector,
};
use bevy::{audio::Volume, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct GenericPlugin;

impl Plugin for GenericPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>();
        app.add_event::<TreeDiedEvent>();
        app.add_systems(
            Update,
            (
                tick_iframes,
                tick_lifetimes,
                bullet_enemy_collision,
                play_sound_event,
                tick_gathering,
                handle_tree_death,
                attract_all_xp,
            ),
        );
    }
}

fn attract_all_xp(
    player_p: Query<&Transform, With<Player>>,
    mut xp_q: Query<(&Transform, &mut Velocity), (With<AttractedToPlayer>, Without<Player>)>,
) {
    let player = player_p.single();
    for (t, mut v) in xp_q.iter_mut() {
        const ATTRACT_SPEED: f32 = 1000.0;

        let vector = (player.translation - t.translation).normalize_or_zero();

        v.linvel.x = vector.x * ATTRACT_SPEED;
        v.linvel.y = vector.y * ATTRACT_SPEED;
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
            SoundEvent::Damage => (asset_loader.damage_sound.clone(), 0.01),
            SoundEvent::Death => (asset_loader.death_sound.clone(), 0.005),
            SoundEvent::XpPickup => (asset_loader.xp_pickup_sound.clone(), 0.3),
            SoundEvent::AttackTree => (asset_loader.attack_tree_sound.clone(), 0.5),
            SoundEvent::TreeHitGround => (asset_loader.tree_hit_ground_sound.clone(), 0.7),
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

fn handle_tree_death(
    mut cmd: Commands,
    mut tree_death_ev: EventReader<TreeDiedEvent>,
    mut sound_events: EventWriter<SoundEvent>,
    asset_loader: Res<AssetLoader>,
) {
    let mut rng = rand::thread_rng();

    for TreeDiedEvent(e, pos, xp) in tree_death_ev.read() {
        let range = rng.gen_range(1..=5);
        for _i in 0..range {
            let mut vector = random_vector();
            vector *= 10.0;
            cmd.spawn(Xp(*xp))
                .insert(SpriteBundle {
                    transform: Transform::from_translation(*pos),
                    texture: asset_loader.xp_sprite.clone(),
                    ..default()
                })
                .insert(Velocity::linear(vector.truncate()))
                .insert(RigidBody::Dynamic)
                .insert(Restitution::coefficient(0.01))
                .insert(Name::new("XP"));
        }

        if let Some(mut e) = cmd.get_entity(*e) {
            e.despawn();
        }
        sound_events.send(SoundEvent::TreeHitGround);
    }
    tree_death_ev.clear();
}

// I don't even...
fn bullet_enemy_collision(
    mut cmd: Commands,
    mut enemy_q: Query<(&Transform, &mut Health, &mut IFrames, &SpawnerId, Entity), With<Enemy>>,
    bullet_q: Query<(&Damage, Entity), (With<Bullet>, Without<Enemy>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut sound_event: EventWriter<SoundEvent>,
    mut xp_event: EventWriter<XpDropEvent>,
    mut entity_death_event: EventWriter<SpawnedEntiyDeathEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(a, b, _f) = collision_event {
            for (transform, mut hp, mut iframes, sid, entity) in enemy_q.iter_mut() {
                if a == &entity {
                    for (dmg, bullet_e) in bullet_q.iter() {
                        if &bullet_e == b {
                            hp.0 = hp.0.sub(dmg.0);
                            iframes.0 = 0.2;
                            cmd.entity(bullet_e).despawn();

                            if hp.0 <= 0.0 {
                                cmd.entity(entity).despawn();
                                entity_death_event.send(SpawnedEntiyDeathEvent(sid.0));
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
                                entity_death_event.send(SpawnedEntiyDeathEvent(sid.0));
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
