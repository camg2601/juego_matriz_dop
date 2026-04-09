use bevy::prelude::*;

pub mod componentes;
pub mod movimiento_en_tierra;
pub mod movimiento_en_agua;

pub use componentes::*;
use crate::EstadoJuego;
use crate::estados::EstadoPausa;


pub struct FisicasPlugin;

impl Plugin for FisicasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            movimiento_en_tierra::sistema_salto_y_gravedad
                .run_if(in_state(EstadoJuego::JugandoEnTierra))
                .run_if(in_state(EstadoPausa::Activo)),
        )
        .add_systems(
            FixedUpdate,
            movimiento_en_agua::sistema_movimiento_en_agua
                .run_if(in_state(EstadoJuego::JugandoEnAgua))
                .run_if(in_state(EstadoPausa::Activo)),
        )
        ;
    }
}
