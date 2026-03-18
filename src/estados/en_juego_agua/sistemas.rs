use crate::entradas::{Accion, AccionEjecutada};
use crate::EstadoJuego;
use bevy::prelude::*;

/// Setup inicial cuando entramos en Jugando
pub fn entrar_juego() {
    info!("Entrando en JUEGO");
    // habilitar HUD, reiniciar contadores, reproducir música de juego, etc.
}

/// Input de juego vía acciones (no teclas)
pub fn input_juego(
    mut reader: MessageReader<AccionEjecutada>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
) {
    for evento in reader.read() {
        match evento.accion {
            Accion::Pausar => {
                next_state.set(EstadoJuego::Pausa);
                info!("Pausando desde JUEGO");
            }
            Accion::Saltar => {
                next_state.set(EstadoJuego::JugandoEnTierra);
                info!("Saliendo del agua a tierra");
            }
            _ => {}
        }
    }
}

/// Cleanup al salir del juego (p.ej pasar a menu o gameover)
pub fn salir_juego() {
    info!("Saliendo de JUEGO");
    // limpiar entidades temporales, HUD, etc.
}
