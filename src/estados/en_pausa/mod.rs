use bevy::prelude::*;
use crate::EstadoJuego;

pub mod sistemas;
pub use sistemas::*;

pub struct PausaPlugin;

impl Plugin for PausaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(EstadoJuego::Pausa), sistemas::entrar_pausa)
            .add_systems(
                Update,
                sistemas::input_pausa.run_if(in_state(EstadoJuego::Pausa))
            )
            .add_systems(OnExit(EstadoJuego::Pausa), sistemas::salir_pausa);
    }
}
