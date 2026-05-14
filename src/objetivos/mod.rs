use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::*;

pub mod systems;
pub mod components;
pub mod resources;
pub mod events;
pub mod objective;

pub use resources::*;
pub use systems::*;

use crate::estados::EstadoJuego;
use crate::objetivos::components::EnemyKilledEvent;

pub struct ObjectivesPlugin;

impl Plugin for ObjectivesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            ObjectiveState {
                completed: false,
                objective: ObjectiveType::Hallway,
                progress: 0,
                requires_collect: false,
                requires_kills: false,
                target: 0,
                time_penalty: None,
                time_remaining: None,
                is_timed: false,
                has_waves: false,
                wave_size: None,
                requires_visuals: false,
                wave_spawned: false,
            }
        ).add_message::<EnemyKilledEvent>()
        .add_systems(OnEnter(EstadoJuego::Transicion), setup_objective)
        .add_systems(Update, (handle_objectives, visual_artifacts).run_if(in_state(EstadoJuego::InGame)))
        .add_systems(OnExit(EstadoJuego::Transicion), visual_defense)
        .add_systems(OnExit(EstadoJuego::Transicion), visual_generators);
    }
}