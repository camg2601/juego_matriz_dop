use bevy::prelude::*;
use crossbeam_channel::unbounded;

mod jugador;
mod fisicas;
mod audio;
mod entradas;
mod mapas;
mod estados;
mod core;
mod visualNodes;




use estados::en_menu::MenuPlugin;
use estados::en_pausa::PausaPlugin;
use estados::en_juego_tierra::JuegoTierraPlugin;
use estados::en_juego_agua::JuegoAguaPlugin;
use core::{start_core_thread, CoreRequest, CoreResponse};
use visualNodes::*;



use estados::EstadoJuego;
use jugador::JugadorPlugin;
use fisicas::FisicasPlugin;
use audio::AudioPlugin;
use entradas::EntradasPlugin;
use mapas::MapasPlugin;

fn main() {

    let (tx_req, rx_req) = unbounded();
    let (tx_res, rx_res) = unbounded();

    start_core_thread(rx_req, tx_res);

    App::new()
        .insert_resource(CoreChannels {
            tx: tx_req,
            rx: rx_res,
        })
        .insert_resource(GameState::default())
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
            VisualNodesPlugin,
        ))
        .run();
}

// -----------------------------------------------------------------------------
// RECURSO GLOBAL: Mapear tecla → sonido + PISTA DE REPRODUCCIÓN ACTIVA
// -----------------------------------------------------------------------------


