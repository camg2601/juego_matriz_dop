use bevy::prelude::*;
use crossbeam_channel::{Sender, Receiver};

use crate::core::{CoreRequest, CoreResponse};
use crate::core::GraphDTO;
use crate::objetivos::ObjectiveState;

#[derive(Resource)]
pub struct CoreChannels {
    pub tx: Sender<CoreRequest>,
    pub rx: Receiver<CoreResponse>,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub current_node: Option<usize>,
    pub last_node: Option<usize>,
    pub graph: Option<GraphDTO>,
}