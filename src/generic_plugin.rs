use std::ops::Sub;

use crate::{
    aggressive_ai_plugin::{AggressiveAi, AggressiveAiState},
    asset_loader_plugin::AssetLoader,
    bullet_plugin::OriginPosition,
    components::{
        AttractedToPlayer, Bullet, Damage, Enemy, Gathering, Health, IFrames, LifeTime,
        PickupRange, PickupType, Player, PlayerPickup,
    },
    events::{ItemDropEvent, SoundEvent, XpDropEvent},
    spawner_plugin::{SpawnedEntiyDeathEvent, SpawnerId},
    tree_plugin::drop_wood,
};
use bevy::{audio::Volume, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct GenericPlugin;

impl Plugin for GenericPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>();
        app.add_event::<ItemDropEvent>();
        app.add_systems(
            Update,
            (
                tick_iframes,
                tick_lifetimes,
                bullet_enemy_collision,
                play_sound_event,
                tick_gathering,
                attract_all_items,
                handle_item_drop_event,
            ),
        );
        app.register_type::<PlayerPickup>();
        app.register_type::<Health>();
        app.register_type::<PickupRange>();
        app.register_type::<Damage>();
        app.register_type::<IFrames>();
        app.register_type::<LifeTime>();
        app.register_type::<Gathering>();
        app.register_type::<PickupType>();
    }
}

fn handle_item_drop_event(
    mut cmd: Commands,
    mut drop_events: EventReader<ItemDropEvent>,
    assets: Res<AssetLoader>,
) {
    for ev in drop_events.read() {
        match ev {
            ItemDropEvent::Wood(count, point) => {
                drop_wood(&mut cmd, *point, *count, assets.item_wood_sprite.clone());
            }
        }
    }
    drop_events.clear();
}

fn attract_all_items(
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
            SoundEvent::Damage => (asset_loader.damage_sound.clone(), 0.1),
            SoundEvent::Death => (asset_loader.death_sound.clone(), 0.005),
            SoundEvent::XpPickup => (asset_loader.xp_pickup_sound.clone(), 0.3),
            SoundEvent::AttackTree => (asset_loader.attack_tree_sound.clone(), 0.5),
            SoundEvent::TreeHitGround => (asset_loader.tree_hit_ground_sound.clone(), 0.7),
            SoundEvent::PistolShoot => (asset_loader.pistol_shoot_sound.clone(), 0.5),
            SoundEvent::MachineGunShoot => (asset_loader.machine_gun_shoot_sound.clone(), 0.2),
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
    mut enemy_q: Query<
        (
            &Transform,
            &mut Health,
            &mut IFrames,
            &SpawnerId,
            &mut AggressiveAi,
            Entity,
        ),
        With<Enemy>,
    >,
    bullet_q: Query<(&Damage, &OriginPosition, Entity), (With<Bullet>, Without<Enemy>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut sound_event: EventWriter<SoundEvent>,
    mut xp_event: EventWriter<XpDropEvent>,
    mut entity_death_event: EventWriter<SpawnedEntiyDeathEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(a, b, _f) = collision_event {
            for (transform, mut hp, mut iframes, sid, mut ai, entity) in enemy_q.iter_mut() {
                if a == &entity {
                    for (dmg, o, bullet_e) in bullet_q.iter() {
                        if &bullet_e == b {
                            hp.0 = hp.0.sub(dmg.0);
                            iframes.0 = 0.2;
                            cmd.entity(bullet_e).despawn();

                            if ai.state != AggressiveAiState::KillMode {
                                ai.state = AggressiveAiState::CheckLocation(o.0);
                            }

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
                    for (dmg, o, bullet_e) in bullet_q.iter() {
                        if &bullet_e == a {
                            hp.0 = hp.0.sub(dmg.0);
                            iframes.0 = 0.2;
                            cmd.entity(bullet_e).despawn();

                            if ai.state != AggressiveAiState::KillMode {
                                ai.state = AggressiveAiState::CheckLocation(o.0);
                            }

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
