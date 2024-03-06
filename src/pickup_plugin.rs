use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader_plugin::AssetLoader,
    components::{AttractedToPlayer, Magnet, PickupRange, PickupType, Player, PlayerPickup, Xp},
    events::PickupTakenEvent,
};

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickupTakenEvent>();
        app.add_systems(Update, (spawn_magnets, take_player_pickups));
        app.add_systems(Update, handle_pickup_taken);
    }
}

fn spawn_magnets(
    mut cmd: Commands,
    magnet_q: Query<Entity, With<Magnet>>,
    assets: Res<AssetLoader>,
) {
    const SPAWN_RANGE: f32 = 10_000.0;
    const MAX_MAGNETS: usize = 10;

    if magnet_q.iter().len() < MAX_MAGNETS {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(-SPAWN_RANGE..SPAWN_RANGE);
        let y = rng.gen_range(-SPAWN_RANGE..SPAWN_RANGE);

        cmd.spawn((
            Magnet,
            PlayerPickup(PickupType::Magnet),
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: assets.magnet_sprite.clone(),
                ..default()
            },
            Name::new("Magnet"),
        ));
    }
}

fn take_player_pickups(
    player_q: Query<(&Transform, &PickupRange), With<Player>>,
    pickup_q: Query<(&Transform, &PlayerPickup, Entity), Without<Player>>,
    mut pickup_event: EventWriter<PickupTakenEvent>,
) {
    let (p_transform, p_pickup_range) = player_q.single();

    for (t, pt, e) in pickup_q.iter() {
        let dist = t.translation.distance(p_transform.translation);
        if dist <= p_pickup_range.0 {
            pickup_event.send(PickupTakenEvent(e, pt.0));
        }
    }
}

fn handle_pickup_taken(
    mut cmd: Commands,
    xp_q: Query<Entity, With<Xp>>,
    mut events: EventReader<PickupTakenEvent>,
) {
    for PickupTakenEvent(e, typ) in events.read() {
        match typ {
            PickupType::Magnet => {
                for xp in xp_q.iter() {
                    cmd.entity(xp).insert(AttractedToPlayer);
                }
            }
        }
        if let Some(mut e) = cmd.get_entity(*e) {
            e.despawn();
        }
    }
    events.clear();
}
