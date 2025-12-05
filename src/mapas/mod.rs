pub mod mapa_de_sonidos;
pub mod mapa_de_teclado;

pub use mapa_de_sonidos::*;
pub use mapa_de_teclado::*;

use bevy::prelude::*;

pub struct MapasPlugin;

impl Plugin for MapasPlugin {
    fn build(&self, app: &mut App) {
        app
            // Solo recursos puros, sin lógica
            .init_resource::<MapaEntradas>();
            // MapaDeSonidos se inserta en Startup desde audio
    }
}
