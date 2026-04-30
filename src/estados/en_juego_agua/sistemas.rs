
use bevy::prelude::*;
use crate::EstadoJuego;
use crate::entradas::{Accion, AccionEjecutada};

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
            Accion::MoverseEnTierra => {
                next_state.set(EstadoJuego::JugandoEnTierra);
                info!("Reiniciando en tierra");
            }
            Accion::NavegarGrafo => {
                next_state.set(EstadoJuego::Transicion);
                info!("Transicionando grafo");
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
