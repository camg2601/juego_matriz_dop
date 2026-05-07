use bevy::ecs::system::Commands;
use bevy::prelude::*;

use crate::entities::components::Enemy;
use crate::entradas::*;
use crate::jugador::componentes::Jugador;

pub fn attack_enemy (
    mut commands: Commands,
    mut query: Query<(Entity, &mut Enemy, &mut Transform), Without<Jugador>>,
    mut reader: MessageReader<AccionEjecutada>,
    player_q: Query<&Transform, With<Jugador>>,
) {
    
    let damage: u32 = 100;
    let mut attack: bool = false;

    for evento in reader.read() {
        match evento.accion {
            Accion::Atacar => { attack = true; },
            _ => { continue }
        }
    }

    for (entity, mut enemy, transform)  in &mut query {
        let player_tf = match player_q.single() {
            Ok(t) => t,
            _ => return,
        };

        if attack {
            let dist = player_tf.translation.distance(transform.translation);

            if dist <= 10.0 {
                enemy.health = enemy.health.saturating_sub(damage);
            }
        }
    }
}