use bevy::post_process::motion_blur::node;
use bevy::prelude::*;
use crate::core::{CoreRequest, CoreResponse};
use crate::entities::components::Enemy;
use crate::jugador::componentes::Jugador;
use crate::objetivos::resources::ObjectiveState;
use crate::visualNodes::components::Completed;
use crate::visualNodes::{GameState};
use crate::objetivos::*;

fn visualObjective (
    mut commands: Commands,
) {

}

pub fn setup_objective(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    mut obj_state: ResMut<ObjectiveState>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
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
    obj_state.progress = 0;

    obj_state.target = objective.target_num(&node.level);
    obj_state.requires_kills = objective.kills();
    obj_state.requires_collect = objective.collect();
    obj_state.is_timed = objective.timed();

    obj_state.completed = objective.completed(&obj_state);

    println!(
        "🎯 Objetivo configurado: {:?} (target: {})",
        obj_state.objective, obj_state.target
    );
}

pub fn handle_objectives(
    mut obj_state: ResMut<ObjectiveState>,
    mut query: Query<(Entity, &Enemy), Without<Jugador>>,
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
        obj_state.progress += 1;

        println!("{:?}", obj_state.progress);
        println!("{:?}", obj_state.target);
    }

    if objective.completed(&obj_state) {
        writer.write(Completed {
            complete: true,
        });
    }
}