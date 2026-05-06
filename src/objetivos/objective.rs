use bevy::prelude::*;

use crate::objetivos::{ObjectiveState, resources::ObjectiveType};

impl ObjectiveType {
    pub fn kills(&self) -> bool {
        matches!(self, Self::Exterminate | Self::Defense)
    }

    pub fn collect(&self) -> bool {
        matches!(self, Self::Artifacts)
    }

    pub fn timed(&self) -> bool {
        matches!(self, Self::Cameras)
    }

    pub fn completed(&self, state: &ObjectiveState) -> bool {
        match self {
            ObjectiveType::Hallway => true,
            ObjectiveType::Defense => state.progress >= state.target,
            ObjectiveType::Exterminate => state.progress >= state.target,
            ObjectiveType::Generators => state.progress >= state.target,
            ObjectiveType::Artifacts => state.progress >= state.target,
            ObjectiveType::Cameras => state.time_remaining == Some(0),
            ObjectiveType::Destroy => state.progress >= state.target,
            ObjectiveType::Rescue => state.progress >= state.target,
        } 
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Hallway" => Some(ObjectiveType::Hallway),
            "Defense" => Some(ObjectiveType::Defense),
            "Exterminate" => Some(ObjectiveType::Exterminate),
            "Generators" => Some(ObjectiveType::Generators),
            "Artifacts" => Some(ObjectiveType::Artifacts),
            "Cameras" => Some(ObjectiveType::Cameras),
            "Destroy" => Some(ObjectiveType::Destroy),
            "Rescue" => Some(ObjectiveType::Rescue),
            _ => None,
        }
    }
}