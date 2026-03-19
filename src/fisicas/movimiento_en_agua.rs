use bevy::prelude::*;
use crate::fisicas::componentes::*;
use crate::entradas::{Accion, AccionEjecutada};

pub fn sistema_movimiento_en_agua(
    mut reader: MessageReader<AccionEjecutada>,
    time: Res<Time<Fixed>>,
    teclado: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocidad, &Gravedad)>,
) {
    // Parámetros físicos del agua
    let fuerza_nado = 400.0;     // Fuerza al "nadar"
    let aceleración_nado = 600.0;
    let gravedad_agua = 300.0;  // Mucho menor que en tierra
    let rozamiento = 0.92;      // Amortiguación del movimiento
    let flotabilidad = 250.0;   // Empuje hacia arriba
    let deceleracion = 200.0;

    let mut quiere_nadar = false;

    let mut nadar_arriba = false;
    let mut nadar_abajo = false;
    let mut nadar_derecha = false;
    let mut nadar_izquierda = false;

    for evento in reader.read() {
        match evento.accion {
            Accion::Saltar => { nadar_arriba = true },
            Accion::MoverAtrasAgua => { nadar_abajo = true },
            Accion::MoverDerechaTierra => { nadar_derecha = true},
            Accion::MoverIzquierdaTierra => { nadar_izquierda = true},
            _ => { continue }
        }
    }

    for (mut transform, mut velocidad, gravedad) in &mut query {
        // Impulso hacia arriba al nadar
        
/* */
        // Gravedad reducida por el agua

        if nadar_arriba {
            velocidad.y += aceleración_nado * time.delta_secs();
        } else if nadar_abajo {
            velocidad.y -= aceleración_nado * time.delta_secs();
        } else if nadar_derecha {
            velocidad.x += aceleración_nado * time.delta_secs();
        } else if nadar_izquierda {
            velocidad.x -= aceleración_nado * time.delta_secs();
        }


        if !nadar_derecha && !nadar_izquierda {
            if velocidad.x > 0.0 {
                velocidad.x -= deceleracion * time.delta_secs();
                if velocidad.x < 0.0 {
                    velocidad.x = 0.0;
                }
            } else if velocidad.x < 0.0 {
                velocidad.x += deceleracion * time.delta_secs();
                if velocidad.x > 0.0 {
                    velocidad.x = 0.0;
                }
            }
        }

        if !nadar_arriba && !nadar_abajo {
            if velocidad.y > 0.0 {
                velocidad.y -= deceleracion * time.delta_secs();
                if velocidad.y < 0.0 {
                    velocidad.y = 0.0;
                }
            } else if velocidad.y < 0.0 {
                velocidad.y += deceleracion * time.delta_secs();
                if velocidad.y > 0.0 {
                    velocidad.y = 0.0;
                }
            }
        }

        if velocidad.y == 0.0 {
            velocidad.y -= gravedad.a * time.delta_secs();
        }
               

        // Aplicar desplazamiento
        transform.translation.y += velocidad.y * time.delta_secs();
        transform.translation.x += velocidad.x * time.delta_secs();
    }
}