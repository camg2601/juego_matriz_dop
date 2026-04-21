use bevy::prelude::Plugin;
use bevy::prelude::App;
use bevy::prelude::Startup;
use bevy::prelude::Update;

pub mod components;
pub mod resources;
pub mod systems;

pub use components::{NodoSeleccion, TextoUI};
pub use resources::{CoreChannels, GameState};
pub use systems::{
    recibir_grafo,
    spawn_opciones,
    input_movimiento_grafo,
    setup_ui,
    actualizar_ui,
    iniciar_partida,
};

pub struct VisualNodesPlugin;

impl Plugin for VisualNodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_ui,
            iniciar_partida, 
        ))
        .add_systems(Update, (
            recibir_grafo,
            spawn_opciones,
            actualizar_ui,
            input_movimiento_grafo,
        ));
    }
}