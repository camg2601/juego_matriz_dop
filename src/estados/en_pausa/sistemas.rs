use bevy::prelude::*;
use crate::{EstadoJuego};
use crate::entradas::{Accion, AccionEjecutada};

pub fn limpiar_acciones_pendientes(mut mensajes: ResMut<Messages<AccionEjecutada>>) {
    mensajes.clear();
}

pub fn input_pausa(
    mut reader: MessageReader<AccionEjecutada>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
) {
    for evento in reader.read() {
        match evento.accion {
            Accion::Pausar => {
                next_state.set(EstadoJuego::JugandoEnTierra);
                info!("Reanudando juego desde PAUSA");
            }
            Accion::Reiniciar => {
                next_state.set(EstadoJuego::Reiniciar);
                info!("Reinicio solicitado desde PAUSA");
            }

            Accion::Salir => {
                info!("Salir desde menú de pausa");
            }

            _ => {}
        }
    }
}
