use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDTO {
    pub id: usize,
    pub level: usize,
    pub objective: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDTO {
    pub nodes: Vec<NodeDTO>,
    pub edges: Vec<(usize, usize)>,
}