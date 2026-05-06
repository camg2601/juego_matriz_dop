use bevy::post_process::motion_blur::node;
use bevy::prelude::*;
use crate::core::{CoreRequest, CoreResponse};
use crate::entities::components::Enemy;
use crate::objetivos::resources::ObjectiveState;
use crate::visualNodes::{GameState};
use crate::objetivos::*;

fn hallway (
    mut commands: Commands,
) {

}

fn exterminate () {

}

pub fn setup_objective(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    mut obj_state: ResMut<ObjectiveState>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
) {

    println!("{:?}", obj_state.completed);

    if !state.is_changed() {
        next_state.set(EstadoJuego::InGame);
        return;
    }

    let graph = match &state.graph {
        Some(g) => g,
        None => return,
    };

    println!("graph");

    let current = match state.current_node {
        Some(id) => id,
        None => return,
    };

    println!("current");

    let node = match graph.nodes.iter().find(|n| n.id == current) {
        Some(n) => n,
        None => return,
    };

    println!("{:?}", node);

    // 🔹 convertir objetivo
    let objective = match ObjectiveType::from_str(&node.objective) {
        Some(o) => o,
        None => return,
    };

    println!("objective");

    if node.clear {
        obj_state.completed = true;
        return;
    }

    println!("clear");

    obj_state.objective = objective.clone();
    obj_state.progress = 0;
    obj_state.completed = false;

    match objective {
        ObjectiveType::Hallway => {
            obj_state.target = 0;
            obj_state.requires_kills = false;
            obj_state.requires_collect = false;
            obj_state.is_timed = false;
        }

        ObjectiveType::Exterminate => {
            obj_state.target = 10 + node.level as u32 * 2;
            obj_state.requires_kills = true;
            obj_state.requires_collect = false;
            obj_state.is_timed = false;
        }

        ObjectiveType::Defense
        |ObjectiveType::Rescue 
        | ObjectiveType::Generators
        | ObjectiveType::Artifacts
        | ObjectiveType::Cameras
        | ObjectiveType::Destroy => {
            obj_state.target = 0;
            obj_state.requires_kills = false;
            obj_state.requires_collect = false;
            obj_state.is_timed = false;
        }
    }
    
    println!("match");

    println!(
        "🎯 Objetivo configurado: {:?} (target: {})",
        obj_state.objective, obj_state.target
    );

    

    println!(
        "Cambiando estado"
    );
}

pub fn handleObjectives(
    mut state: ResMut<GameState>,
    mut objState: ResMut<ObjectiveState>,
) {
    
}