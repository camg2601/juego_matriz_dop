use std::collections::HashMap;

use bevy::app::FixedUpdate;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::Plugin;
use bevy::prelude::App;
use bevy::state::condition::in_state;

use crate::entities::entity_types::EnemyType;
use crate::entities::resources::EnemyHealths;
use crate::entities::resources::EnemyWeights;
use crate::EstadoJuego;

pub mod systems;
pub mod components;
pub mod resources;
pub mod entity_types;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyWeights {
            weights: HashMap::from([
                (EnemyType::Normal, 3.0),
                (EnemyType::Rapido, 2.5),
                (EnemyType::Tanque, 1.5),
                (EnemyType::Rango, 2.0),
                (EnemyType::Soporte, 1.2),
                (EnemyType::Limitador, 0.8),
            ]),
        })
        .insert_resource(EnemyHealths {
            healths: HashMap::from([
                (EnemyType::Normal, 100),
                (EnemyType::Rapido, 50),
                (EnemyType::Tanque, 200),
                (EnemyType::Rango, 100),
                (EnemyType::Soporte, 150),
                (EnemyType::Limitador, 25),
            ])
        }).add_systems(FixedUpdate, systems::spawn_enemies.run_if(in_state(EstadoJuego::JugandoEnAgua)))
        .add_systems(FixedUpdate, systems::despawn_enemies.run_if(in_state(EstadoJuego::Transicion)));
    }
}