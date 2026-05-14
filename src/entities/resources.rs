use std::collections::HashMap;

use bevy::{ecs::resource::Resource, time::Timer};

use crate::entities::entity_types::EnemyType;

#[derive(Resource)]
pub struct EnemyWeights {
    pub weights: HashMap<EnemyType, f32>,
}

#[derive(Resource)]
pub struct EnemyHealths {
    pub healths: HashMap<EnemyType, u32>
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);