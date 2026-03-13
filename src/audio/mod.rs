pub mod inicio;
pub mod sistemas;

pub use inicio::*;
pub use sistemas::*;

use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cargar_sonidos).add_systems(
            Update,
            reproducir_sonidos.after(crate::entradas::traducir_entradas_a_acciones),
        );
    }
}
