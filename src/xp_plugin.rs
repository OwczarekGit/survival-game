use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{PickupRange, Player, UiLevelDisplayBar, UiLevelDisplayNumber, Xp},
    events::{SoundEvent, XpDropEvent},
    xp_level::XpLevel,
};

pub struct XpPlugin;

impl Plugin for XpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<XpDropEvent>();
        app.add_systems(Update, (spawn_xp, pickup_xp, attract_xp, update_xp_display));
    }
}

fn spawn_xp(
    mut cmd: Commands,
    mut xp_drop_event: EventReader<XpDropEvent>,
    asset_loader: Res<AssetLoader>,
) {
    for ev in xp_drop_event.read() {
        cmd.spawn(Xp(10.))
            .insert(SpriteBundle {
                transform: Transform::from_translation(ev.0),
                texture: asset_loader.xp_sprite.clone(),
                ..default()
            })
            .insert(Velocity::zero())
            .insert(RigidBody::Dynamic)
            .insert(Name::new("XP"));
    }
    xp_drop_event.clear();
}

fn pickup_xp(
    mut cmd: Commands,
    mut player_q: Query<(&Transform, &PickupRange, &mut XpLevel), With<Player>>,
    xp_q: Query<(&Transform, &Xp, Entity), Without<Player>>,
    mut sound_event: EventWriter<SoundEvent>,
) {
    if let Ok((player, range, mut lvl)) = player_q.get_single_mut() {
        for (t, xp, e) in xp_q.iter() {
            if player.translation.distance(t.translation) <= range.0 {
                lvl.add_xp(*xp);
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
        let attract_speed = 500.0;
        let dt = time.delta_seconds();
        for (mut v, t) in xp_q.iter_mut() {
            let dist = player.translation.distance(t.translation);
            if dist <= (range.0 * 2.0) {
                let vector =
                    ((player.translation - t.translation).normalize_or_zero() * dt * attract_speed)
                        * dist;
                v.linvel = vector.xy();
            }
        }
    }
}

fn update_xp_display(
    player_q: Query<&XpLevel, With<Player>>,
    mut display_lvl_q: Query<&mut Text, With<UiLevelDisplayNumber>>,
    mut display_bar_q: Query<&mut Style, (With<UiLevelDisplayBar>, Without<UiLevelDisplayNumber>)>,
) {
    if let Ok(player) = player_q.get_single() {
        let mut lvl = display_lvl_q.single_mut();
        let mut bar = display_bar_q.single_mut();

        lvl.sections[0].value = format!("{}", player.level);
        bar.width = Val::Percent(player.xp / player.xp_to_next * 100.0);
    }
}
