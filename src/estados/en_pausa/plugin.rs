use bevy::prelude::*;

use crate::estados::EstadoPausa;

use super::{sistemas, ui};

pub struct PausaPlugin;

impl Plugin for PausaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(EstadoPausa::Pausa),
            (sistemas::limpiar_acciones_pendientes, ui::mostrar_ui_pausa),
        )
        .add_systems(
            Update,
            sistemas::input_pausa.run_if(in_state(EstadoPausa::Pausa)),
        )
        .add_systems(
            OnExit(EstadoPausa::Pausa),
            (sistemas::limpiar_acciones_pendientes, ui::ocultar_ui_pausa),
        );
    }
}
