use bevy::prelude::*;
use crate::visualNodes::{CoreChannels, GameState};
use crate::core::{CoreRequest, CoreResponse};
use crate::jugador::componentes::Jugador;
use crate::visualNodes::components::*;

// =========================
// 📩 Recibir grafo del core
// =========================

pub fn recibir_grafo(
    mut state: ResMut<GameState>,
    channels: Res<CoreChannels>,
) {
    while let Ok(msg) = channels.rx.try_recv() {
        match msg {
            CoreResponse::GraphUpdated(dto) => {
                state.graph = Some(dto.clone());

                // si no hay nodo actual, tomar el primero
                if state.current_node.is_none() {
                    if let Some(n) = dto.nodes.first() {
                        state.current_node = Some(n.id);
                    }
                }

                println!("📡 Grafo actualizado");
            }
        }
    }
}

// =========================
// 🧱 Mostrar opciones
// =========================

pub fn spawn_opciones(
    mut commands: Commands,
    state: Res<GameState>,
    query: Query<Entity, With<NodoSeleccion>>,
) {
    if !state.is_changed() {
        return;
    }

    // limpiar anteriores
    for e in query.iter() {
        commands.entity(e).despawn();
    }

    let graph = match &state.graph {
        Some(g) => g,
        None => return,
    };

    let current = match state.current_node {
        Some(id) => id,
        None => return,
    };

    // buscar conexiones
    let mut targets = Vec::new();

    for (a, b) in &graph.edges {
        if *a == current {
            targets.push(*b);
        }
        if *b == current {
            targets.push(*a);
        }
    }

    println!("Current node: {}", current);
    println!("Edges: {:?}", graph.edges);
    println!("Targets: {:?}", targets);

    // dibujar cuadros
    for (i, target) in targets.iter().enumerate() {

        // 🔍 buscar info del nodo en el DTO
        let node_data = graph.nodes.iter().find(|n| n.id == *target);

        let (nivel, objetivo) = if let Some(n) = node_data {
            (n.level, n.objective.clone())
        } else {
            (0, "??".to_string())
        };

        let count = targets.len().max(1);
        let screen_w = 1000.0;
        let spacing = screen_w / (count as f32 + 1.0);

        let x = (i as f32 + 1.0) * spacing - screen_w / 2.0;
        let y = 0.0;

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            NodoSeleccion { id: *target },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new(format!("L{}: {}", nivel, objetivo)),
                Transform::from_xyz(0.0, 30.0, 1.0), // arriba del cuadro
            ));
        });
    }
}

// =========================
// 🎮 Input para moverse
// =========================

pub fn input_movimiento_grafo(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    channels: Res<CoreChannels>,
    player_q: Query<&Transform, With<Jugador>>,
    nodos_q: Query<(&Transform, &NodoSeleccion)>,
) {
    if !keyboard.just_pressed(KeyCode::KeyM) {
        return;
    }

    let player_tf = match player_q.single() {
        Ok(t) => t,
        _ => return,
    };

    for (tf, nodo) in nodos_q.iter() {
        let dist = player_tf.translation.distance(tf.translation);

        if dist < 40.0 {
            // cambiar nodo actual
            state.current_node = Some(nodo.id);

            // avisar al core
            let _ = channels.tx.send(CoreRequest::PlayerEnteredNode {
                node_id: nodo.id,
            });

            println!("➡️ Movido a nodo {}", nodo.id);
        }
    }
}

// =========================
// 📝 UI
// =========================

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextoUI,
    ));
}

// =========================
// 🔄 Actualizar UI
// =========================

pub fn actualizar_ui(
    state: Res<GameState>,
    mut query: Query<&mut Text, With<TextoUI>>,
) {
    let graph = match &state.graph {
        Some(g) => g,
        None => return,
    };

    let current = match state.current_node {
        Some(id) => id,
        None => return,
    };

    if let Some(node) = graph.nodes.iter().find(|n| n.id == current) {
        for mut text in query.iter_mut() {
            **text = format!(
                "Nivel: {} | Objetivo: {}",
                node.level,
                node.objective
            );
        }
    }
}

pub fn iniciar_partida(
    channels: Res<CoreChannels>,
) {
    let _ = channels.tx.send(CoreRequest::StartGame {});
}