use bevy::ecs::{component::Component, entity::Entity, message::Message};

#[derive(Message)]
pub struct EnemyKilledEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Dead;