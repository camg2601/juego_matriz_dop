use bevy::{ecs::{component::Component, entity::Entity, message::Message}, math::Vec3};

#[derive(Message)]
pub struct EnemyKilledEvent {
    pub entity: Entity,
    pub pos: Vec3,
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Artifact;

#[derive(Component)]
pub struct Generator {
    pub repaired: u32,
}

#[derive(Component)]
pub struct Defense {
    pub health: u32,
}

#[derive(Component)]
pub struct GeneratorText;