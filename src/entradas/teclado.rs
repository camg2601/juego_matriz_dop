use bevy::prelude::*;
use crate::entradas::{Accion, AccionEjecutada};
use crate::estados::EstadoPausa;
use crate::mapas::MapaEntradas;

pub fn traducir_entradas_a_acciones(
    teclado: Res<ButtonInput<KeyCode>>,
    mapa: Res<MapaEntradas>,
    estado_pausa: Res<State<EstadoPausa>>,
    mut writer: MessageWriter<AccionEjecutada>,
) {
    for (tecla, accion) in mapa.teclado.iter() {
        if teclado.just_pressed(*tecla) {
            if matches!(estado_pausa.get(), EstadoPausa::Pausa)
                && !matches!(accion, Accion::Pausar | Accion::Reiniciar | Accion::Salir)
            {
                continue;
            }

            writer.write(AccionEjecutada {
                accion: *accion,
            });
            println!("Acción ejecutada: {:?}", accion);
        }
    }
}
