use bevy::prelude::*;
use crate::EstadoJuego;
use crate::entradas::{Accion, AccionEjecutada};

/// Al entrar en Menu (setup UI, música, etc.)
pub fn entrar_menu() {
    info!("Entrando en MENU");
    // spawn UI, música, etc.
}

/// Input en el menú, usa mensajes (AccionEjecutada)
pub fn input_menu(
    mut reader: MessageReader<AccionEjecutada>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
) {
    for evento in reader.read() {
        match evento.accion {
            Accion::MoverseEnTierra => {
                next_state.set(EstadoJuego::Transicion);
                info!("Seleccionado INICIAR desde MENU");
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
