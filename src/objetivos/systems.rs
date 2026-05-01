use bevy::prelude::*;
use crate::core::{CoreRequest, CoreResponse};
use crate::entities::components::Enemy;
use crate::objetivos::resources::ObjectiveState;
use crate::visualNodes::{GameState};
use crate::objetivos::*;

pub fn handleObjectives(
    mut state: ResMut<GameState>,
    mut objState: ResMut<ObjectiveState>,
) {
    
}