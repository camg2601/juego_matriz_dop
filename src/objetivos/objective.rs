use crate::{core::NodeDTO, objetivos::{ObjectiveState, resources::ObjectiveType}};

impl ObjectiveType {
    pub fn target_num(&self, level: &usize) -> u32 {
        match self {
            ObjectiveType::Hallway => 0,
            ObjectiveType::Exterminate => 10 + *level as u32 * 2,
            ObjectiveType::Defense
            | ObjectiveType::Cameras
            | ObjectiveType::Generators
            | ObjectiveType::Artifacts
            | ObjectiveType::Destroy
            | ObjectiveType::Rescue => 0,
        }
    }

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
            ObjectiveType::Cameras => state.time_remaining == Some(0),
            ObjectiveType::Defense
            | ObjectiveType::Exterminate
            | ObjectiveType::Generators
            | ObjectiveType::Artifacts
            | ObjectiveType::Destroy
            | ObjectiveType::Rescue => state.progress >= state.target,
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