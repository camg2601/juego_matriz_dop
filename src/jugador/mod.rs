use bevy::prelude::*;

pub mod sistemas;
pub mod componentes;

pub use sistemas::*;


pub struct JugadorPlugin;

impl Plugin for JugadorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configurar_visual);
    }
}
