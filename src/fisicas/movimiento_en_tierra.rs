use bevy::prelude::*;
use crate::fisicas::componentes::*;
use crate::entradas::{Accion, AccionEjecutada, teclado};

pub fn sistema_salto_y_gravedad(
    mut reader: MessageReader<AccionEjecutada>,
    teclado: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &mut Velocidad, &mut EnSuelo, &Gravedad, &Aceleracion)>,
) {
    let fuerza_salto = 600.0;
    let velocidad_max = 300.0;
    let friccion = 100.0;
    let suelo_y = 0.0;

    let mut quiere_saltar = false;
    let mut mover_izquierda = false;
    let mut mover_derecha = false;

    for evento in reader.read() {
        match evento.accion {
            Accion::Saltar => { quiere_saltar = true },
            Accion::MoverDerechaTierra => { mover_derecha = true },
            Accion::MoverIzquierdaTierra => { mover_izquierda = true},
            _ => { continue }
        }
    }

    for (mut transform, mut velocidad, mut en_suelo, gravedad, aceleracion) in &mut query {

        if mover_derecha {
            velocidad.x += aceleracion.t * time.delta_secs();
        } else if mover_izquierda {
            velocidad.x -= aceleracion.t * time.delta_secs();
        }

        velocidad.x = velocidad.x.clamp(-velocidad_max, velocidad_max);

        if !mover_derecha && !mover_izquierda && en_suelo.0 {
            if velocidad.x > 0.0 {
                velocidad.x -= friccion * time.delta_secs();
                if velocidad.x < 0.0 {
                    velocidad.x = 0.0;
                }
            } else if velocidad.x < 0.0 {
                velocidad.x += friccion * time.delta_secs();
                if velocidad.x > 0.0 {
                    velocidad.x = 0.0;
                }
            }
        }

        if quiere_saltar && en_suelo.0 {
            velocidad.y = fuerza_salto;
            en_suelo.0 = false;
        }

        velocidad.y -= gravedad.t * time.delta_secs();
        transform.translation.x += velocidad.x * time.delta_secs();
        transform.translation.y += velocidad.y * time.delta_secs();

        if transform.translation.y <= suelo_y {
            transform.translation.y = suelo_y;
            velocidad.y = 0.0;
            en_suelo.0 = true;
        }
    }
}