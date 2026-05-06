use bevy::prelude::*;
use crate::EstadoJuego;

pub mod sistemas;

pub struct JuegoAguaPlugin;

impl Plugin for JuegoAguaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(EstadoJuego::InGame), sistemas::entrar_juego)
            .add_systems(
                FixedUpdate,
                sistemas::input_juego.run_if(in_state(EstadoJuego::InGame)),
            )
            .add_systems(OnExit(EstadoJuego::InGame), sistemas::salir_juego);
    }
}
