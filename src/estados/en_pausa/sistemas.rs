use bevy::prelude::*;

use crate::{EstadoJuego, estados::EstadoPausa};
use crate::entradas::{Accion, AccionEjecutada};

pub fn limpiar_acciones_pendientes(mut mensajes: ResMut<Messages<AccionEjecutada>>) {
    mensajes.clear();
}

pub fn input_pausa(
    mut reader: MessageReader<AccionEjecutada>,
    mut next_state_juego: ResMut<NextState<EstadoJuego>>,
    mut next_state_pausa: ResMut<NextState<EstadoPausa>>,
) {
    for evento in reader.read() {
        match evento.accion {
            Accion::Pausar => {
                next_state_pausa.set(EstadoPausa::Activo);
                info!("Reanudando juego desde PAUSA");
            }
            Accion::Reiniciar => {
                next_state_pausa.set(EstadoPausa::Activo);
                next_state_juego.set(EstadoJuego::Reiniciar);
                info!("Reinicio solicitado desde PAUSA");
            }

            Accion::Salir => {
                info!("Salir desde menú de pausa");
            }

            _ => {}
        }
    }
}
