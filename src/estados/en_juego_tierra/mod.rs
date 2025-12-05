use bevy::prelude::*;
use crate::EstadoJuego;
use crate::fisicas::movimiento_en_tierra;
use crate::fisicas::movimiento_en_tierra::sistema_salto_y_gravedad;


pub mod sistemas;
pub use sistemas::*;


pub struct JuegoTierraPlugin;

impl Plugin for JuegoTierraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(EstadoJuego::JugandoEnTierra), sistemas::entrar_juego)
            .add_systems(
                Update,
                sistemas::input_juego.run_if(in_state(EstadoJuego::JugandoEnTierra))
            )
            .add_systems(
                FixedUpdate,
                sistema_salto_y_gravedad
            .run_if(in_state(EstadoJuego::JugandoEnTierra))
            )
            .add_systems(OnExit(EstadoJuego::JugandoEnTierra), sistemas::salir_juego);
    }
}