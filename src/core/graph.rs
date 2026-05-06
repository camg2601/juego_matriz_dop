use std::{collections::HashMap, sync::mpsc::{Receiver, Sender}};
use std::time::Instant;

use crate::core::{NodeDTO, GraphDTO};

const MAX_NODES: usize = 27;
const MAX_NODES_PER_LEVEL: usize = 3;

#[derive(Clone)]
pub struct Node {
    pub active: bool,
    pub occupied_by: Option<usize>,
    pub id: usize,
    pub seed: u32,
    pub objective: String,
    pub level: usize,
    pub clear: bool,
}

pub struct Graph {
    matrix: Vec<Vec<i32>>,
    nodes: Vec<Node>,
    objectives: Vec<&'static str>,
    weights: HashMap<String, f32>,
    seed: u32,
    last_objectives: Vec<String>,
    active_levels: Vec<usize>,
    max_active_levels: usize,
}

impl Graph {
    pub fn new(seed: u32) -> Self {
        let mut nodes = Vec::new();

        for i in 0..MAX_NODES {
            nodes.push(Node {
                active: false,
                occupied_by: None,
                id: i,
                seed: seed.wrapping_add((i as u32).wrapping_mul(0x9E3779B9)),
                objective: String::new(),
                level: 0,
                clear: false,
            });
        }

        let matrix = vec![vec![0; MAX_NODES]; MAX_NODES];

        let objectives = vec![
            "Exterminate",
            "Generators",
            "Artifacts",
            "Defense",
            "Cameras",
            "Destroy",
            "Rescue",
            "Hallway",
        ];

        let mut weights = HashMap::new();
        weights.insert("Hallway".to_string(), 4.0);
        weights.insert("Exterminate".to_string(), 2.5);
        weights.insert("Destroy".to_string(), 2.0);
        weights.insert("Generators".to_string(), 1.8);
        weights.insert("Artifacts".to_string(), 1.5);
        weights.insert("Cameras".to_string(), 1.2);
        weights.insert("Defense".to_string(), 1.0);
        weights.insert("Rescue".to_string(), 0.8);

        Self {
            nodes,
            matrix,
            objectives,
            weights,
            seed,
            last_objectives: Vec::new(),
            active_levels: Vec::new(),
            max_active_levels: 8,
        }
    }

    pub fn clear_node(&mut self, node_id: usize) {
        let node = &mut self.nodes[node_id];

        if !node.clear {
            node.clear = true;
        }
    }

    pub fn get_active_levels(&self) -> Vec<usize> {
        let mut levels: Vec<usize> = self.nodes
            .iter()
            .filter(|n| n.active)
            .map(|n| n.level)
            .collect();

        levels.sort();
        levels.dedup();
        levels
    }

    fn hash(x: u32) -> u32 {
        x.wrapping_mul(1664525).wrapping_add(1013904223)
    }

    // 🔀 Fisher-Yates shuffle determinista
    fn shuffle<T>(arr: &mut [T], mut seed: u32) {
        for i in (1..arr.len()).rev() {
            seed = Self::hash(seed);
            let j = (seed as usize) % (i + 1);
            arr.swap(i, j);
        }
    }

    fn level_multiplier(objective: &str, level: usize) -> f32 {
        match objective {
            "Hallway" => if level <= 2 { 2.5 } else { 0.7 },
            "Defense" => 1.0 + level as f32 * 0.2,
            "Exterminate" => 1.2,
            "Generators" => 1.0 + level as f32 * 0.1,
            "Artifacts" => 1.0 + level as f32 * 0.05,
            "Cameras" => if level > 3 { 1.5 } else { 1.0 },
            "Destroy" => 1.1,
            "Rescue" => if level > 4 { 1.3 } else { 0.6 },
            _ => 1.0,
        }
    }

    fn get_weight(&self, objective: &str, node: &Node, history: &[String]) -> f32 {
        let base = *self.weights.get(objective).unwrap_or(&1.0);

        let mut weight = base * Self::level_multiplier(objective, node.level);

        // penalizar repetición reciente
        if history.iter().rev().take(2).any(|o| o == objective) {
            weight *= 0.3;
        }

        weight
    }

    fn pick_objective(&self, node: &Node, history: &[String], seed: u32) -> String {
        let mut h = seed;

        // 🔥 más entropía
        h ^= self.seed;
        h ^= node.level as u32;
        h ^= history.len() as u32;
        h = Self::hash(h);

        let mut weighted = Vec::new();
        let mut total = 0.0;

        for obj in &self.objectives {
            let w = self.get_weight(obj, node, history);
            total += w;
            weighted.push((obj, w));
        }

        if total <= 0.0 {
            return "Hallway".to_string();
        }

        let mut value = (h as f32 / u32::MAX as f32) * total;

        for (obj, w) in weighted {
            if value < w {
                return obj.to_string();
            }
            value -= w;
        }

        self.objectives.last().unwrap().to_string()
    }

    fn violates_pattern(history: &[String], candidate: &str) -> bool {
        if history.len() < 2 {
            return false;
        }

        let last = &history[history.len() - 1];
        let prev = &history[history.len() - 2];

        last == candidate && prev == candidate
    }

    fn pick_with_rules(&self, node: &Node, history: &[String]) -> String {
        let mut h = node.seed;

        for _ in 0..3 {
            let candidate = self.pick_objective(node, history, h);

            if !Self::violates_pattern(history, &candidate) {
                return candidate;
            }

            h = Self::hash(h);
        }

        self.pick_objective(node, history, h)
    }

    fn connection_count(&self, node_id: usize) -> usize {
        self.matrix[node_id]
            .iter()
            .filter(|&&v| v == 1)
            .count()
    }

    fn desired_connections(&self, node_id: usize, level: usize) -> usize {
        let mut h = self.nodes[node_id].seed ^ self.seed ^ level as u32;
        h = Self::hash(h);

        // entre 1 y 3 conexiones
        ((h % 3) + 1) as usize
    }

    pub fn create_edge(&mut self, level_index: usize) {
        let mut edge_nodes = Vec::new();
        let mut free_nodes = Vec::new();
        let mut selected_nodes = Vec::new();

        // 🔍 recolectar nodos
        for node in &self.nodes {
            if self.matrix[node.id][node.id] == -1 {
                edge_nodes.push(node.id);
            }

            if !node.active {
                free_nodes.push(node.id);
            }
        }

        // 🔀 mezclar nodos libres
        let mut h = self.seed ^ level_index as u32;
        Self::shuffle(&mut free_nodes, h);

        // 🔥 calcular cantidad con más entropía
        for &id in &edge_nodes {
            h ^= self.nodes[id].seed;
            h = Self::hash(h);
        }

        let mut count = if level_index == 1 {
            1
        } else {
            ((h % MAX_NODES_PER_LEVEL as u32) + 1) as usize
        };

        count = count.min(free_nodes.len());

        for i in 0..count {
            let node_id = free_nodes[i];

            let obj = if level_index == 1 {
                "Hallway".to_string()
            } else {
                self.pick_with_rules(&self.nodes[node_id], &self.last_objectives)
            };

            let node = &mut self.nodes[node_id];

            node.active = true;
            node.level = level_index;
            node.objective = obj.clone();

            selected_nodes.push(node_id);

            if level_index != 1 {
                self.last_objectives.push(obj);

                if self.last_objectives.len() > 5 {
                    self.last_objectives.remove(0);
                }
            }
        }

        for &id in &selected_nodes {
            self.matrix[id][id] = -1;
        }

        if !edge_nodes.is_empty() {
            let mut shuffled_edges = edge_nodes.clone();
            let mut shuffled_targets = selected_nodes.clone();

            let mut h = self.seed ^ level_index as u32;

            Self::shuffle(&mut shuffled_edges, h);
            h = Self::hash(h);
            Self::shuffle(&mut shuffled_targets, h);

            let pairs = shuffled_edges.len().max(shuffled_targets.len());

            for i in 0..pairs {
                let origin = shuffled_edges[i % shuffled_edges.len()];
                let target = shuffled_targets[i % shuffled_targets.len()];

                self.matrix[origin][target] = 1;
                self.matrix[target][origin] = 1;
            }

            // limpiar estado de frontera anterior
            for &id in &edge_nodes {
                self.matrix[id][id] = 0;
            }
        }

        let mut h = self.seed ^ (level_index as u32).wrapping_mul(12345);

        for &origin in &edge_nodes {
            let current = self.connection_count(origin);
            let desired = self.desired_connections(origin, level_index);

            if current >= desired {
                continue;
            }

            let mut attempts = 0;

            while self.connection_count(origin) < desired && attempts < 5 {
                h = Self::hash(h);

                let target = selected_nodes[(h as usize) % selected_nodes.len()];

                if self.matrix[origin][target] == 0 {
                    self.matrix[origin][target] = 1;
                    self.matrix[target][origin] = 1;
                }

                attempts += 1;
            }
        }

        self.active_levels.push(level_index);

        if self.active_levels.len() > self.max_active_levels {
            let old_level = self.active_levels.remove(0);
            self.reset_level(old_level);
        }
    }

    pub fn get_node_level(&self, node_id: usize) -> Option<usize> {
        self.nodes
            .iter()
            .find(|n| n.id == node_id)
            .map(|n| n.level)
    }

    pub fn reset_level(&mut self, level_index: usize) {
        for node in &mut self.nodes {
            if node.level == level_index {
                let id = node.id;

                for i in 0..self.matrix.len() {
                    self.matrix[id][i] = 0;
                    self.matrix[i][id] = 0;
                }

                node.active = false;
                node.occupied_by = None;
                node.objective.clear();
                node.level = 0;

                node.seed = Self::hash(node.seed ^ (level_index as u32) ^ self.seed);
            }
        }

        if self.last_objectives.len() > 3 {
            self.last_objectives.drain(0..2);
        } 
    }

    pub fn to_dto(&self) -> GraphDTO {
        let mut edges = Vec::new();

        for i in 0..self.matrix.len() {
            for j in (i + 1)..self.matrix.len() {
                if self.matrix[i][j] == 1 {
                    edges.push((i, j));
                }
            }
        }

        let nodes = self.nodes
            .iter()
            .filter(|n| n.active)
            .map(|n| NodeDTO {
                id: n.id,
                level: n.level,
                objective: n.objective.clone(),
                clear: n.clear,
            })
            .collect();

        GraphDTO { nodes, edges }
    }
}