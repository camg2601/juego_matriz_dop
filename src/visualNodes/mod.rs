use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::Plugin;
use bevy::prelude::App;
use bevy::prelude::Startup;
use bevy::prelude::Update;

pub mod components;
pub mod resources;
pub mod systems;

use bevy::state::condition::in_state;
pub use resources::{CoreChannels, GameState};
pub use systems::{
    recibir_grafo,
    spawn_opciones,
    setup_ui,
    actualizar_ui,
    iniciar_partida,
    minimapa,
};

use crate::estados::EstadoJuego;
use crate::visualNodes::components::Completed;

pub struct VisualNodesPlugin;

impl Plugin for VisualNodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_ui,
            iniciar_partida, 
        ))
        .add_systems(Update, (
            recibir_grafo,
            actualizar_ui,
            minimapa,
        ))
        .add_message::<Completed>()
        .add_systems(Update, spawn_opciones.run_if(in_state(EstadoJuego::InGame)))
        ;
    }
}