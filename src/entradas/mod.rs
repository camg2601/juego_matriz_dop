pub mod acciones;
pub mod teclado;

pub use acciones::*;
pub use teclado::*;

use bevy::prelude::*;

pub struct EntradasPlugin;

use crate::EstadoJuego;
use bevy::prelude::in_state;
use bevy::app::Update;

impl Plugin for EntradasPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<crate::mapas::MapaEntradas>()
            .add_message::<AccionEjecutada>()
            .add_systems(
                Update,
                traducir_entradas_a_acciones
            );
    }
}
