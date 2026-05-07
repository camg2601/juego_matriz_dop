use bevy::prelude::*;
use crate::entradas::AccionEjecutada;
use crate::mapas::MapaEntradas;

pub fn traducir_entradas_a_acciones(
    teclado: Res<ButtonInput<KeyCode>>,
    mapa: Res<MapaEntradas>,
    mut writer: MessageWriter<AccionEjecutada>,
) {
    for (tecla, accion) in mapa.teclado.iter() {
        let activa = if *tecla == KeyCode::KeyM || *tecla == KeyCode::KeyE {
            teclado.just_pressed(*tecla)
        } else {
            teclado.pressed(*tecla)
        };

        if activa {
            writer.write(AccionEjecutada {
                accion: *accion,
            });
        }
    }
}