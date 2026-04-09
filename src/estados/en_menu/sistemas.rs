use bevy::prelude::*;
use crate::EstadoJuego;
use crate::entradas::{Accion, AccionEjecutada};
use crate::estados::reiniciar_juego::sistemas::EstadoInicialJuego;

/// Al entrar en Menu (setup UI, música, etc.)
pub fn entrar_menu() {
    info!("Entrando en MENU");
    // spawn UI, música, etc.
}

/// Input en el menú, usa mensajes (AccionEjecutada)
pub fn input_menu(
    mut reader: MessageReader<AccionEjecutada>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
    mut estado_inicial: ResMut<EstadoInicialJuego>,
) {
    for evento in reader.read() {
        match evento.accion {
            Accion::MoverseEnTierra => {
                estado_inicial.0 = EstadoJuego::JugandoEnTierra;
                next_state.set(EstadoJuego::JugandoEnTierra);
                info!("Seleccionado INICIAR desde MENU");
            }
            Accion::MoverseEnAgua => {
                estado_inicial.0 = EstadoJuego::JugandoEnAgua;
                next_state.set(EstadoJuego::JugandoEnAgua);
                info!("Seleccionado INICIAR EN AGUA desde MENU");
            }
            Accion::Reiniciar => {
                next_state.set(EstadoJuego::Reiniciar);
                info!("Seleccionado REINICIAR desde MENU");
            }
            Accion::Salir => {
                info!("Seleccionado SALIR desde MENU");
                // dispatch exit logic (o set a GameOver / salir del app)
            }
            _ => {}
        }
    }
}

/// Limpieza al salir del menú
pub fn salir_menu() {
    info!("Saliendo del MENU");
    // despawn UI, stop music, etc.
}
