use bevy::prelude::*;
use crate::fisicas::componentes::*;
use crate::entradas::{Accion, AccionEjecutada};

pub fn sistema_movimiento_en_agua(
    mut reader: MessageReader<AccionEjecutada>,
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &mut Velocidad)>,
) {
    // Parámetros físicos del agua
    let fuerza_nado = 400.0;     // Fuerza al "nadar"
    let gravedad_agua = 300.0;  // Mucho menor que en tierra
    let rozamiento = 0.92;      // Amortiguación del movimiento
    let flotabilidad = 250.0;   // Empuje hacia arriba

    let mut quiere_nadar = false;

    // Leer acciones
    for evento in reader.read() {
        if evento.accion == Accion::Flotar {
            quiere_nadar = true;
        }
    }

    for (mut transform, mut velocidad) in &mut query {
        // Impulso hacia arriba al nadar
        if quiere_nadar {
            velocidad.y += 3.0;
            println!("velocidad.y al nadar: {}", velocidad.y);
        }
/* */
        // Gravedad reducida por el agua
               

        // Aplicar desplazamiento
        transform.translation.y += velocidad.y * time.delta_secs();
    }
}
