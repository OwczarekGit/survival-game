use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;

use crate::{
    asset_loader_plugin::AssetLoader,
    bullet_plugin::BulletFiredEvent,
    components::{Damage, Enemy, LifeTime},
    events::SoundEvent,
    weapon::Weapon,
};

#[derive(Debug, Clone, Copy, Component)]
pub struct Turret;

#[derive(Debug, Clone, Copy, Component)]
pub struct TurretViewRange(pub f32);

#[derive(Debug, Clone, Event)]
pub struct SpawnTurretEvent(pub Vec2);

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTurretEvent>();
        app.add_systems(Update, handle_spawn_turret_event);
        app.add_systems(Update, turret_fire);
    }
}

fn handle_spawn_turret_event(
    mut cmd: Commands,
    mut events: EventReader<SpawnTurretEvent>,
    asset_loader: Res<AssetLoader>,
) {
    for ev in events.read() {
        spawn_turret(&mut cmd, ev.0, asset_loader.turret_sprite.clone());
    }

    events.clear();
}

pub fn spawn_turret(cmd: &mut Commands, pos: Vec2, texture: Handle<Image>) {
    cmd.spawn((
        Turret,
        TurretViewRange(350.0),
        Weapon::new(Damage(0.5), 700.0, 40.0, 0.1),
        RigidBody::Fixed,
        Velocity::zero(),
        Restitution::default(),
        Collider::cuboid(32.0, 32.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(64.0)),
                ..default()
            },
            transform: Transform::from_translation(pos.extend(25.0)),
            texture,
            ..default()
        },
    ));
}

fn turret_fire(
    enemy_q: Query<&Transform, (With<Enemy>, Without<Turret>)>,
    mut turret_q: Query<(&Transform, &TurretViewRange, &mut Weapon), With<Turret>>,
    mut bullet_ev: EventWriter<BulletFiredEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    dt: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (tt, vr, mut tw) in turret_q.iter_mut() {
        if let Some(target) = enemy_q
            .iter()
            .filter(|e| e.translation.truncate().distance(tt.translation.truncate()) <= vr.0)
            .collect::<Vec<_>>()
            .choose(&mut rng)
        {
            if tw.fire(&dt) {
                bullet_ev.send(BulletFiredEvent {
                    acc: tw.accuracy,
                    at: target.translation.truncate(),
                    from: tt.translation.truncate(),
                    dmg: tw.damage,
                    lifetime: LifeTime(30),
                    bullet_speed: 1_000.0,
                });

                sound_ev.send(SoundEvent::MachineGunShoot);
            }
        }
    }
}
