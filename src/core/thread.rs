use crossbeam_channel::{Sender, Receiver};
use std::thread;

use crate::core::dto::GraphDTO;
use crate::core::graph::Graph;


#[derive(Debug)]
pub enum CoreRequest {
    StartGame {},
    PlayerEnteredNode { node_id: usize },
}

#[derive(Debug)]
pub enum CoreResponse {
    GraphUpdated(GraphDTO),
}

pub fn start_core_thread(
    rx: Receiver<CoreRequest>,
    tx: Sender<CoreResponse>,
) {
    thread::spawn(move || {
        let mut graph: Option<Graph> = None;
        let mut current_level = 1;

        while let Ok(request) = rx.recv() {
            match request {

                CoreRequest::StartGame { } => {
                    let mut rng = fastrand::Rng::new();
                    let mut g = Graph::new(rng.u32(10000..99999));

                    g.create_edge(current_level);
                    current_level += 1;

                    g.create_edge(current_level);
                    current_level += 1;


                    graph = Some(g);

                    let dto = graph.as_ref().unwrap().to_dto();
                    let _ = tx.send(CoreResponse::GraphUpdated(dto));
                }

                CoreRequest::PlayerEnteredNode { node_id: _ } => {
                    println!("Jugador entro al nodo");

                    if let Some(g) = graph.as_mut() {
                        g.create_edge(current_level);
                        current_level += 1;

                        let dto = g.to_dto();
                        let _ = tx.send(CoreResponse::GraphUpdated(dto));
                    }
                }
            }
        }

        println!("Core thread terminado");
    });
}