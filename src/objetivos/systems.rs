use bevy::prelude::*;
use crate::core::{CoreRequest, CoreResponse};
use crate::entities::components::Enemy;
use crate::visualNodes::{GameState};

pub fn exterminate(
    mut state: ResMut<GameState>,
    query: Query<Entity, With<Enemy>>,
) {
    
}