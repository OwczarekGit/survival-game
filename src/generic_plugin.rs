use bevy::prelude::*;

use crate::components::Velocity;

pub struct GenericPlugin;

impl Plugin for GenericPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_postions);
    }
}

fn update_postions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (mut t, v) in query.iter_mut() {
        t.translation.x += v.0.x * dt;
        t.translation.y += v.0.y * dt;
    }
}
