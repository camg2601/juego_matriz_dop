use bevy::prelude::*;

mod jugador;
mod fisicas;
mod audio;
mod entradas;
mod mapas;
mod estados;
mod triangulo;




use estados::en_menu::MenuPlugin;
use estados::en_pausa::PausaPlugin;
use estados::en_juego_tierra::JuegoTierraPlugin;
use estados::en_juego_agua::JuegoAguaPlugin;
use triangulo::TrianguloPlugin;



use estados::EstadoJuego;
use jugador::JugadorPlugin;
use fisicas::FisicasPlugin;
use audio::AudioPlugin;
use entradas::EntradasPlugin;
use mapas::MapasPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<EstadoJuego>()
        .add_plugins((
            MapasPlugin,
            MenuPlugin,
            PausaPlugin,
            JuegoTierraPlugin,
            JuegoAguaPlugin,
            EntradasPlugin,
            AudioPlugin,
            FisicasPlugin,
            JugadorPlugin,
            TrianguloPlugin
        ))
        .run();
}


// -----------------------------------------------------------------------------
// RECURSO GLOBAL: Mapear tecla → sonido + PISTA DE REPRODUCCIÓN ACTIVA
// -----------------------------------------------------------------------------


