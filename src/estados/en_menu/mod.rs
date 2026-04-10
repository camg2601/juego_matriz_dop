use bevy::prelude::*;
use crate::EstadoJuego;

pub mod sistemas;
pub use sistemas::*;


pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(EstadoJuego::Menu), sistemas::entrar_menu)
            .add_systems(
                Update,
                sistemas::input_menu.run_if(in_state(EstadoJuego::Menu))
            )
            .add_systems(OnExit(EstadoJuego::Menu), sistemas::salir_menu);
    }
}
