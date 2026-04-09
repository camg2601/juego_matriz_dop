use bevy::prelude::*;

pub mod en_juego_agua;
pub mod en_juego_tierra;
pub mod en_menu;
pub mod en_pausa;
pub mod reiniciar_juego;

pub use en_juego_agua::*;
pub use en_juego_tierra::*;
pub use en_menu::*;
pub use en_pausa::*;
pub use reiniciar_juego::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum EstadoJuego {
    #[default]
    Menu,
    JugandoEnTierra,
    JugandoEnAgua,
    Reiniciar,
    Cinematica,
    GameOver,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum EstadoPausa {
    #[default]
    Activo,
    Pausa,
}
