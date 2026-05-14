use bevy::post_process::motion_blur::node;
use bevy::prelude::*;
use rand::rng;
use crate::core::{CoreRequest, CoreResponse};
use crate::entities::components::{Destroy, Enemy};
use crate::jugador::componentes::Jugador;
use crate::objetivos::components::{Artifact, Defense, Generator, GeneratorText};
use crate::objetivos::resources::ObjectiveState;
use crate::visualNodes::components::Completed;
use crate::visualNodes::{GameState};
use crate::objetivos::*;

pub fn visual_artifacts(
    mut commands: Commands,
    mut reader: MessageReader<EnemyKilledEvent>,
    obj_state: Res<ObjectiveState>,
) {
    if obj_state.objective != ObjectiveType::Artifacts || obj_state.completed {
        return;
    }

    for event in reader.read() {
        let mut rng = fastrand::Rng::new();

        let drop_chance = 0.3;

        if rng.f64() > drop_chance {
            continue;
        }
        
        commands.spawn((
            Artifact,
            Sprite {
                color: Color::srgb(0.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Transform::from_translation(event.pos),
        ));
    }
}

pub fn visual_defense(
    mut commands: Commands,
    obj_state: Res<ObjectiveState>,
) {
    if obj_state.objective != ObjectiveType::Defense || obj_state.completed {
        return;
    }

    commands.spawn((
        Defense {
            health: 100,
        },
        Sprite {
            color: Color::srgb(0.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn visual_generators(
    mut commands: Commands,
    windows: Query<&Window>,
    mut obj_state: ResMut<ObjectiveState>,
) {

    if obj_state.objective != ObjectiveType::Generators || obj_state.completed {
        return;
    }

    let window = windows.single().unwrap();

    let mut rng = fastrand::Rng::new();

    let quantity = 4;

    for _ in 0..quantity {

        let x = rng.f32() * window.width() - window.width() / 2.0;
        let y = rng.f32() * window.height() - window.height() / 2.0;

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Generator {
                repaired: 0,
            },
        )).with_children(|parent| {
            parent.spawn((
                GeneratorText,
                Text2d::new("0 / 500"),
                Transform::from_xyz(0.0, 35.0, 1.0),
                ));
        });
    }
}

pub fn setup_objective(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    mut obj_state: ResMut<ObjectiveState>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
    query: Query<Entity, With<Enemy>>,
) {
    if !state.is_changed() {
        next_state.set(EstadoJuego::InGame);
        return;
    }

    let graph = match &state.graph {
        Some(g) => g,
        None => return,
    };

    let current = match state.current_node {
        Some(id) => id,
        None => return,
    };

    let node = match graph.nodes.iter().find(|n| n.id == current) {
        Some(n) => n,
        None => return,
    };

    let objective = match ObjectiveType::from_str(&node.objective) {
        Some(o) => o,
        None => return,
    };

    if node.clear {
        obj_state.completed = true;
        return;
    }

    obj_state.objective = objective.clone();
    obj_state.progress = if objective == ObjectiveType::Destroy {
        1000
    } else {
        0
    };

    obj_state.target = objective.target_num(&node.level);
    obj_state.requires_kills = objective.kills();
    obj_state.requires_collect = objective.collect();
    obj_state.is_timed = objective.timed();
    obj_state.has_waves = objective.waves();

    if objective.waves() {
        obj_state.wave_size = Some(objective.wave_size(&node.level));
    }

    obj_state.completed = objective.completed(&obj_state);

    println!(
        "🎯 Objetivo configurado: {:?} (target: {})",
        obj_state.objective, obj_state.target
    );
}

pub fn handle_objectives(
    mut obj_state: ResMut<ObjectiveState>,
    mut query: Query<(Entity, &Enemy), Without<Jugador>>,
    mut destroy_query: Query<(Entity, &Enemy, &Destroy)>,
    mut reader: MessageReader<EnemyKilledEvent>,
    mut state: ResMut<GameState>,
    mut commands: Commands,
    mut writer: MessageWriter<Completed>,
) {
    let graph = match &state.graph {
        Some(g) => g,
        None => return,
    };

    let current = match state.current_node {
        Some(id) => id,
        None => return,
    };

    let node = match graph.nodes.iter().find(|n| n.id == current) {
        Some(n) => n,
        None => return,
    };

    let objective = match ObjectiveType::from_str(&node.objective) {
        Some(o) => o,
        None => return,
    };

    for _event in reader.read() {
        match obj_state.objective {
            ObjectiveType::Hallway 
            | ObjectiveType::Rescue
            | ObjectiveType::Destroy
            | ObjectiveType::Generators
            | ObjectiveType::Cameras 
            | ObjectiveType::Artifacts => { continue; },
            ObjectiveType::Defense => {
                if query.iter().count() == 1 {
                    obj_state.progress += 1;

                    obj_state.wave_spawned = false;
                }
            },
            ObjectiveType::Exterminate => {
                obj_state.progress += 1;
            },
        }
    }

    for (_entity, enemy, Destroy) in destroy_query.iter() {
        obj_state.progress = enemy.health;
    }

    if objective.completed(&obj_state) {
        writer.write(Completed {
            complete: true,
        });
    }
}