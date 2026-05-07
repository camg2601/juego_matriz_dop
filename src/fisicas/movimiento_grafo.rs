use bevy::prelude::*;

use crate::core::CoreRequest;
use crate::entradas::{Accion, AccionEjecutada};
use crate::estados::EstadoJuego;
use crate::jugador::componentes::Jugador;
use crate::visualNodes::{CoreChannels, GameState, components::NodoSeleccion};

pub fn input_movimiento_grafo(
    mut reader: MessageReader<AccionEjecutada>,
    mut state: ResMut<GameState>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
    channels: Res<CoreChannels>,
    player_q: Query<&Transform, With<Jugador>>,
    nodos_q: Query<(&Transform, &NodoSeleccion)>,
) {
    let mut mover_grafo = false;

    for evento in reader.read() {
        match evento.accion {
            Accion::NavegarGrafo => { mover_grafo = true; },
            _ => { continue; }
        }
    }

    if mover_grafo {
        let player_tf = match player_q.single() {
            Ok(t) => t,
            _ => return,
        };

        for (tf, nodo) in nodos_q.iter() {
            let dist = player_tf.translation.distance(tf.translation);

            if dist < 40.0 {
                let _ = channels.tx.send(CoreRequest::PlayerEnteredNode {
                    node_id: nodo.id,
                    last_node: state.current_node,
                });

                state.last_node = state.current_node;
                state.current_node = Some(nodo.id);

                next_state.set(EstadoJuego::Transicion);
            }
        }
    }
}