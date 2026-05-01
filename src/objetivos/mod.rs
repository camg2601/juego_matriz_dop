use bevy::prelude::Plugin;
use bevy::prelude::App;
use bevy::prelude::Startup;
use bevy::prelude::Update;

pub mod systems;
pub mod components;
pub mod resources;
pub mod events;

pub use resources::*;

pub struct ObjectivesPlugin;

impl Plugin for ObjectivesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            ObjectiveState {
                completed: false,
                objective: ObjectiveType::Hallway,
                progress: 0,
                requiresCollect: false,
                requiresKills: false,
                target: 0,
                time_penalty: Some(0),
                time_remaining: Some(0),
                isTimed: false,
            }
        );
    }
}