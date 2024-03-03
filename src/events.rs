use bevy::ecs::event::Event;

#[derive(Event)]
pub struct DamageEvent;

#[derive(Event)]
pub struct DeathEvent;
