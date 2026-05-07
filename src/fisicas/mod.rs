use bevy::{prelude::*, ui::update};

pub mod componentes;
pub mod movimiento_en_tierra;
pub mod movimiento_en_agua;
pub mod movimiento_grafo;
pub mod combat;

pub use componentes::*;
use crate::EstadoJuego;

fn estados_validos(estados: Vec<EstadoJuego>) -> impl FnMut(Res<State<EstadoJuego>>) -> bool {
    move |state: Res<State<EstadoJuego>>| {
        estados.contains(state.get())
    }
}

pub struct FisicasPlugin;

impl Plugin for FisicasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            movimiento_en_tierra::sistema_salto_y_gravedad
                .run_if(in_state(EstadoJuego::JugandoEnTierra)),
        )
        .add_systems(
            FixedUpdate,
            movimiento_en_agua::sistema_movimiento_en_agua
                .run_if(in_state(EstadoJuego::InGame)),
        )
        .add_systems(Update, movimiento_grafo::input_movimiento_grafo)
        .add_systems(Update, combat::attack_enemy)
        ;
    }
}
