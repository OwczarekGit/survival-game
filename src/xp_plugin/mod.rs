pub mod xp_level;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{PickupRange, Player, UiLevelDisplayBar, UiLevelDisplayNumber},
    events::{SoundEvent, XpDropEvent},
};
use xp_level::XpLevel;

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Xp(pub f32);

pub struct XpPlugin;

impl Plugin for XpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<XpDropEvent>();
        app.add_systems(Update, (spawn_xp, pickup_xp, attract_xp, update_xp_display));
        app.register_type::<XpLevel>();
        app.register_type::<Xp>();
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

pub fn drop_xp(cmd: &mut Commands, xp: Xp, position: Vec2, velocity: Vec2, texture: Handle<Image>) {
    cmd.spawn((
        xp,
        SpriteBundle {
            transform: Transform::from_translation(position.extend(10.0)),
            texture,
            ..default()
        },
        Velocity::linear(velocity),
        Restitution::coefficient(2.0),
        RigidBody::Dynamic,
        Name::new("Xp"),
    ));
}
