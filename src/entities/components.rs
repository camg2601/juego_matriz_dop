use bevy::ecs::component::Component;

use crate::entities::entity_types::EnemyType;

#[derive(Component)]
pub struct Enemy {
    pub e_type: EnemyType,
    pub health: u32,
}

#[derive(Component)]
pub struct Destroy; 


