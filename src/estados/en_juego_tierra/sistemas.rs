
use bevy::prelude::*;
use crate::EstadoJuego;
use crate::estados::EstadoPausa;
use crate::entradas::{Accion, AccionEjecutada};

/// Setup inicial cuando entramos en Jugando
pub fn entrar_juego() {
    info!("Entrando en JUEGO");
    // habilitar HUD, reiniciar contadores, reproducir música de juego, etc.
}

/// Input de juego vía acciones (no teclas)
pub fn input_juego(
    mut reader: MessageReader<AccionEjecutada>,
    mut next_state_juego: ResMut<NextState<EstadoJuego>>,
    mut next_state_pausa: ResMut<NextState<EstadoPausa>>,
) {
    for evento in reader.read() {
        match evento.accion {
            Accion::Pausar => {
                next_state_pausa.set(EstadoPausa::Pausa);
                info!("Pausando desde JUEGO");
            }
            Accion::Reiniciar => {
                next_state_juego.set(EstadoJuego::Reiniciar);
                info!("Reiniciando desde juego en tierra");
            }
            Accion::MoverseEnTierra => {
                // podría reiniciar nivel si corresponde
            }
            Accion::MoverseEnAgua => {
                next_state_juego.set(EstadoJuego::JugandoEnAgua);
                info!("En tierra");
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
