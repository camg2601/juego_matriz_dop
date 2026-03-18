use bevy::prelude::*;

use crate::EstadoJuego;

use super::{sistemas, ui};

pub struct PausaPlugin;

impl Plugin for PausaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(EstadoJuego::Pausa),
                (sistemas::limpiar_acciones_pendientes, ui::mostrar_ui_pausa),
            )
            .add_systems(
                Update,
                sistemas::input_pausa.run_if(in_state(EstadoJuego::Pausa)),
            )
            .add_systems(
                OnExit(EstadoJuego::Pausa),
                (ui::ocultar_ui_pausa, sistemas::limpiar_acciones_pendientes),
            );
    }
}
