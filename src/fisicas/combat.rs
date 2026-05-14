use bevy::ecs::system::Commands;
use bevy::prelude::*;

use crate::entities::components::Enemy;
use crate::entradas::*;
use crate::jugador::componentes::Jugador;
use crate::objetivos::{ObjectiveState, ObjectiveType};
use crate::objetivos::components::{Artifact, Dead, EnemyKilledEvent, Generator, GeneratorText};

pub fn attack_enemy (
    mut commands: Commands,
    mut query: Query<(Entity, &mut Enemy, &mut Transform), Without<Jugador>>,
    mut reader: MessageReader<AccionEjecutada>,
    mut writer: MessageWriter<EnemyKilledEvent>,
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

            if dist <= 30.0 {
                enemy.health = enemy.health.saturating_sub(damage);

                if enemy.health <= 0 {
                    writer.write(EnemyKilledEvent {
                        entity,
                        pos: transform.translation,
                    });

                    commands.entity(entity)
                        .insert(Dead)
                        .despawn();
                }
            }
        }
    }
}

pub fn collect_artifacts(
    mut commands: Commands,
    player_q: Query<&Transform, With<Jugador>>,
    artifact_q: Query<(Entity, &Transform), With<Artifact>>,
    mut obj_state: ResMut<ObjectiveState>,
    mut reader: MessageReader<AccionEjecutada>,
) {

    if obj_state.objective != ObjectiveType::Artifacts {
        return;
    }

    let mut ejecutar_accion = false;

    for evento in reader.read() {
        match evento.accion {
            Accion::Contexto => { ejecutar_accion = true; },
            _ => { continue }
        }
    }

    let player_transform = match player_q.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    for (entity, transform) in &artifact_q {

        if ejecutar_accion {
            let distance = player_transform
            .translation
            .distance(transform.translation);

            if distance < 20.0 {

                commands.entity(entity).despawn();

                obj_state.progress += 1;

            }
        }
    }
}

pub fn repair_generator (
    mut commands: Commands,
    player_q: Query<&Transform, With<Jugador>>,
    mut generator_q: Query<(Entity, &Transform, &mut Generator, &mut Sprite, &Children)>,
    mut obj_state: ResMut<ObjectiveState>,
    mut reader: MessageReader<AccionEjecutada>,
    mut text_q: Query<&mut Text2d, With<GeneratorText>>
) {

    if obj_state.objective != ObjectiveType::Generators {
        return;
    }

    let mut ejecutar_accion = false;

    let repair_rate = 10;

    for evento in reader.read() {
        match evento.accion {
            Accion::Contexto => {
                ejecutar_accion = true;
            }

            _ => continue
        }
    }

    let player_transform = match player_q.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    for (entity, transform, mut generator, mut sprite, children) in &mut generator_q {

        if ejecutar_accion && generator.repaired < 500 {

            let distance = player_transform
                .translation
                .distance(transform.translation);

            if distance < 40.0 {

                generator.repaired += repair_rate;

                for child in children {
                    if let Ok(mut text) = text_q.get_mut(*child) {
                        *text = Text2d::new(format!(
                            "{} / 500",
                            generator.repaired
                        ));
                    }
                }

                if generator.repaired >= 500 {
                    sprite.color = Color::srgb(0.0, 1.0, 0.0);

                    obj_state.progress += 1;
                }
            }
        }
    }
}