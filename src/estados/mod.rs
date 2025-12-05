use bevy::prelude::*;

pub mod en_pausa;
pub mod en_menu;
pub mod en_juego_agua;
pub mod en_juego_tierra;

pub use en_pausa::*;
pub use en_menu::*;
pub use en_juego_agua::*;   
pub use en_juego_tierra::*; 

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum EstadoJuego {
    #[default]
    Menu,
    JugandoEnTierra,
    JugandoEnAgua,
    Pausa,
    Cinematica,
    GameOver,
}
