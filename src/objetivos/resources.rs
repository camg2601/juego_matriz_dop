use bevy::{ecs::resource::Resource, log::tracing_subscriber::fmt::time};

#[derive(Debug, Clone)]
pub enum ObjectiveType {
    Hallway,
    Defense,
    Exterminate,
    Generators,
    Artifacts,
    Cameras,
    Destroy,
    Rescue,
}

#[derive(Resource)]
pub struct ObjectiveState {
    pub objective: ObjectiveType,
    pub progress: u32,
    pub target: u32,
    pub time_penalty: Option<u32>,
    pub time_remaining: Option<u32>,
    pub requiresKills: bool,
    pub requiresCollect: bool,
    pub isTimed: bool,
    pub completed: bool,
}


