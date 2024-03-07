use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    components::Player,
    utils::{random_mag_from_range, random_vector},
};

pub struct AggressiveAiPlugin;

impl Plugin for AggressiveAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ai);
        app.register_type::<AggressiveAi>();
        app.register_type::<AggressiveAiState>();
    }
}

#[derive(Debug, Component, Reflect)]
pub struct AggressiveAi {
    pub view_range: f32,
    pub state: AggressiveAiState,
}

impl AggressiveAi {
    pub fn with_view_range(view_range: f32) -> Self {
        Self {
            view_range,
            state: AggressiveAiState::ImmediateWander,
        }
    }
}

#[derive(Debug, Component, Reflect)]
pub enum AggressiveAiState {
    ImmediateWander,
    CheckLocation(Vec2),
    Attack,
    Stand,
    Wander(Vec2),
}

fn update_ai(
    player_q: Query<&Transform, With<Player>>,
    mut ai_q: Query<(&Transform, &mut Velocity, &mut AggressiveAi), Without<Player>>,
) {
    let player = player_q.single();
    ai_q.iter_mut()
        .for_each(|(t, mut v, mut a)| ai_tick((t, &mut v, &mut a), player));
}

fn ai_tick((ai_t, ai_v, ai_a): (&Transform, &mut Velocity, &mut AggressiveAi), p_t: &Transform) {
    let mut rng = rand::thread_rng();
    let distance_to_player = p_t.translation.distance(ai_t.translation);

    const ATTACK_SPEED: f32 = 80.0;

    match ai_a.state {
        AggressiveAiState::Attack => {
            if distance_to_player > ai_a.view_range * 1.5 {
                ai_a.state = AggressiveAiState::Stand;
            } else {
                let vector = (p_t.translation - ai_t.translation)
                    .truncate()
                    .normalize_or_zero()
                    * ATTACK_SPEED;
                ai_v.linvel.x = vector.x;
                ai_v.linvel.y = vector.y;
            }
        }
        AggressiveAiState::Stand => {
            if distance_to_player < ai_a.view_range {
                ai_a.state = AggressiveAiState::Attack;
            } else if rng.gen_bool(1.0 / 1000.0) {
                let mut point = random_vector().truncate() * random_mag_from_range(50.0, 300.0);
                point.x += ai_t.translation.x;
                point.y += ai_t.translation.y;

                ai_a.state = AggressiveAiState::Wander(point);
            }
        }
        AggressiveAiState::Wander(point) => {
            if distance_to_player < ai_a.view_range {
                ai_a.state = AggressiveAiState::Attack;
            }

            const WANDER_SPEED: f32 = 20.0;
            let vector = (point - ai_t.translation.truncate()).normalize_or_zero() * WANDER_SPEED;
            ai_v.linvel.x = vector.x;
            ai_v.linvel.y = vector.y;

            if ai_t.translation.truncate().distance(point) < 4.0 {
                ai_v.linvel = Vec2::ZERO;
                ai_a.state = AggressiveAiState::Stand;
            }
        }
        AggressiveAiState::ImmediateWander => {
            let mut point = random_vector().truncate() * random_mag_from_range(50.0, 300.0);
            point.x += ai_t.translation.x;
            point.y += ai_t.translation.y;

            ai_a.state = AggressiveAiState::Wander(point);
        }
        AggressiveAiState::CheckLocation(point) => {
            if distance_to_player < ai_a.view_range {
                ai_a.state = AggressiveAiState::Attack;
            }
            let vector = (point - ai_t.translation.truncate()).normalize_or_zero() * ATTACK_SPEED;
            ai_v.linvel.x = vector.x;
            ai_v.linvel.y = vector.y;

            if ai_t.translation.truncate().distance(point) < 4.0 {
                ai_v.linvel = Vec2::ZERO;
                ai_a.state = AggressiveAiState::Stand;
            }
        }
    }
}
