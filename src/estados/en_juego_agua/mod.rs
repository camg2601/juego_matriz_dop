use bevy::prelude::*;
use crate::EstadoJuego;

pub mod sistemas;

pub struct JuegoAguaPlugin;

impl Plugin for JuegoAguaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(EstadoJuego::JugandoEnAgua), sistemas::entrar_juego)
            .add_systems(
                FixedUpdate,
                sistemas::input_juego.run_if(in_state(EstadoJuego::JugandoEnAgua)),
            )
            .add_systems(OnExit(EstadoJuego::JugandoEnAgua), sistemas::salir_juego);
    }
}
