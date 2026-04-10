use bevy::prelude::*;
use crate::{EstadoJuego};
use crate::entradas::{Accion, AccionEjecutada};

/// Al entrar en pausa
pub fn entrar_pausa() {
    info!("Entrando en PAUSA");
    // Aquí puedes spawnear UI, oscurecer pantalla, pausar audio, etc.
}

/// Lee SOLO acciones abstractas (no teclas)
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

            Accion::Salir => {
                info!("Salir desde menú de pausa");
            }

            _ => {}
        }
    }
}

/// Al salir de pausa
pub fn salir_pausa() {
    info!("Saliendo de PAUSA");
}
