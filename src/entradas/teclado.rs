use bevy::prelude::*;
use crate::entradas::AccionEjecutada;
use crate::mapas::MapaEntradas;

pub fn traducir_entradas_a_acciones(
    teclado: Res<ButtonInput<KeyCode>>,
    mapa: Res<MapaEntradas>,
    mut writer: MessageWriter<AccionEjecutada>,
) {
    for (tecla, accion) in mapa.teclado.iter() {
        if teclado.pressed(*tecla) {
            writer.write(AccionEjecutada {
                accion: *accion,
            });
            println!("Acción ejecutada: {:?}", accion);
        }
    }
}