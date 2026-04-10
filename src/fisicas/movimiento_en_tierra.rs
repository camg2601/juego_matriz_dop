use bevy::prelude::*;
use crate::fisicas::componentes::*;
use crate::entradas::{Accion, AccionEjecutada};

pub fn sistema_salto_y_gravedad(
    mut reader: MessageReader<AccionEjecutada>,
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &mut Velocidad, &mut EnSuelo)>,
) {
    let fuerza_salto = 600.0;
    let gravedad = 1800.0;
    let suelo_y = 0.0;

    let mut quiere_saltar = false;

    for evento in reader.read() {
        if evento.accion == Accion::Saltar {
            quiere_saltar = true;
        }
    }

    for (mut transform, mut velocidad, mut en_suelo) in &mut query {

        if quiere_saltar && en_suelo.0 {
            velocidad.y = fuerza_salto;
            en_suelo.0 = false;
        }

        velocidad.y -= gravedad * time.delta_secs();
        transform.translation.y += velocidad.y * time.delta_secs();

        if transform.translation.y <= suelo_y {
            transform.translation.y = suelo_y;
            velocidad.y = 0.0;
            en_suelo.0 = true;
        }
    }
}
