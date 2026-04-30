use bevy::prelude::*;
use crate::entradas::{Accion, AccionEjecutada};
use crate::estados::EstadoJuego;
use crate::visualNodes::{CoreChannels, GameState};
use crate::core::{CoreRequest, CoreResponse};
use crate::visualNodes::components::*;

pub fn recibir_grafo(
    mut state: ResMut<GameState>,
    channels: Res<CoreChannels>,
    mut reader: MessageReader<AccionEjecutada>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
) {
    while let Ok(msg) = channels.rx.try_recv() {
        match msg {
            CoreResponse::GraphUpdated(dto) => {
                state.graph = Some(dto.clone());

                let mut sale_transicion = false;

                for evento in reader.read() {
                    match evento.accion {
                        Accion::NavegarGrafo => { sale_transicion = true },
                        _ => { continue }
                    }
                }

                if sale_transicion {
                    next_state.set(EstadoJuego::JugandoEnAgua);
                }

                if state.current_node.is_none() {
                    if let Some(start_node) = dto.nodes
                        .iter()
                        .find(|n| n.level == 1)
                    {
                        state.current_node = Some(start_node.id);

                        let _ = channels.tx.send(
                            CoreRequest::PlayerEnteredNode {
                                node_id: start_node.id,
                            }
                        );
                    }
                }
            }
        }
    }
}

pub fn spawn_opciones(
    mut commands: Commands,
    state: Res<GameState>,
    query: Query<Entity, With<NodoSeleccion>>,
) {
    if !state.is_changed() {
        return;
    }

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

    let mut targets = Vec::new();

    for (a, b) in &graph.edges {
        if *a == current {
            targets.push(*b);
        }
        if *b == current {
            targets.push(*a);
        }
    }

    for (i, target) in targets.iter().enumerate() {

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
                Transform::from_xyz(0.0, 30.0, 1.0),
            ));
        });
    }
}

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

pub fn minimapa(
    mut commands: Commands,
    state: Res<GameState>,
    time: Res<Time>,
    query: Query<Entity, With<MiniMapa>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }

    let graph = match &state.graph {
        Some(g) => g,
        None => return,
    };

    let current = state.current_node;

    let origin = Vec2::new(300.0, 350.0); // ajusta esto

    let width = 300.0;
    let height = 200.0;


    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.5),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(
            origin.x + width / 2.0,
            origin.y - height / 2.0,
            0.0,
        ),
        MiniMapa,
    ));

    let mut levels: Vec<usize> = graph
        .nodes
        .iter()
        .map(|n| n.level)
        .filter(|lvl| *lvl > 0)
        .collect();

    levels.sort();
    levels.dedup();

    let levels_count = levels.len().max(1);
    let x_spacing = width / (levels_count as f32 + 1.0);

    let mut positions = std::collections::HashMap::new();

    for (lvl_idx, lvl) in levels.iter().enumerate() {
        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.level == *lvl)
            .collect();

        let x = origin.x + (lvl_idx as f32 + 1.0) * x_spacing;

        let count = nodes.len().max(1);
        let y_spacing = height / (count as f32 + 1.0);

        for (i, node) in nodes.iter().enumerate() {
            let y = origin.y - (i as f32 + 1.0) * y_spacing;

            positions.insert(node.id, Vec3::new(x, y, 10.0));
        }
    }

    for (a, b) in &graph.edges {
        if let (Some(pa), Some(pb)) = (positions.get(a), positions.get(b)) {
            let delta = *pb - *pa;
            let length = delta.length();

            let angle = delta.y.atan2(delta.x) + std::f32::consts::FRAC_PI_2;

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(2.0, length)), // 👈 ojo aquí
                    ..default()
                },
                Transform {
                    translation: (*pa + *pb) / 2.0,
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
                MiniMapa,
            ));
        }
    }

    for (id, pos) in positions {
        let mut color = Color::srgb(1.0, 0.0, 0.0);

        if Some(id) == current {
            let t = time.elapsed_secs();
            let blink = (t * 5.0).sin() > 0.0;

            color = if blink {
                Color::WHITE
            } else {
                Color::srgb(1.0, 0.0, 0.0)
            };
        }

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            Transform::from_translation(pos),
            MiniMapa,
        ));
    }
}

pub fn iniciar_partida(
    channels: Res<CoreChannels>,
) {
    let _ = channels.tx.send(CoreRequest::StartGame {});
}