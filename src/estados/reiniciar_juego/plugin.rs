use bevy::prelude::*;

use crate::EstadoJuego;

use super::{sistemas};

pub struct ReiniciarPlugin;

impl Plugin for ReiniciarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<sistemas::EstadoInicialJuego>()
            .add_systems(
                OnEnter(EstadoJuego::Reiniciar),
                (sistemas::limpiar_acciones_pendientes, sistemas::reiniciar_juego),
            )
            .add_systems(
                OnExit(EstadoJuego::Reiniciar),
                sistemas::limpiar_acciones_pendientes,
            );
    }
}