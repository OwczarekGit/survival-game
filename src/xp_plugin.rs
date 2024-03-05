use bevy::prelude::*;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{PickupRange, Player, Velocity, Xp},
    events::{SoundEvent, XpDropEvent},
};

pub struct XpPlugin;

impl Plugin for XpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<XpDropEvent>();
        app.add_systems(Update, (spawn_xp, pickup_xp, attract_xp));
    }
}

fn spawn_xp(
    mut cmd: Commands,
    mut xp_drop_event: EventReader<XpDropEvent>,
    asset_loader: Res<AssetLoader>,
) {
    for ev in xp_drop_event.read() {
        cmd.spawn(Xp(ev.1))
            .insert(SpriteBundle {
                transform: Transform::from_translation(ev.0),
                texture: asset_loader.xp_sprite.clone(),
                ..default()
            })
            .insert(Velocity(Vec2::ZERO))
            .insert(Name::new("XP"));
    }
    xp_drop_event.clear();
}

fn pickup_xp(
    mut cmd: Commands,
    player_q: Query<(&Transform, &PickupRange), With<Player>>,
    xp_q: Query<(&Transform, &Xp, Entity), (With<Xp>, Without<Player>)>,
    mut sound_event: EventWriter<SoundEvent>,
) {
    if let Ok((player, range)) = player_q.get_single() {
        for (t, _xp, e) in xp_q.iter() {
            if player.translation.distance(t.translation) <= range.0 {
                cmd.entity(e).despawn();
                sound_event.send(SoundEvent::XpPickup);
            }
        }
    }
}

fn attract_xp(
    player_q: Query<(&Transform, &PickupRange), With<Player>>,
    mut xp_q: Query<(&mut Velocity, &Transform), (With<Xp>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok((player, range)) = player_q.get_single() {
        let dt = time.delta_seconds();
        for (mut v, t) in xp_q.iter_mut() {
            let dist = player.translation.distance(t.translation);
            if dist <= (range.0 * 2.0) {
                let vector =
                    ((player.translation - t.translation).normalize_or_zero() * dt * 500.0) * dist;
                v.0 = vector.xy();
            }
        }
    }
}
