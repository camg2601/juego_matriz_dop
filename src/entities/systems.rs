use bevy::prelude::*;
// normal
// rapido
// tanque
// rango
// soporte
// limitador

use std::collections::HashMap;

use crate::{entities::{components::{Destroy, Enemy}, entity_types::EnemyType, resources::{EnemyHealths, EnemySpawnTimer, EnemyWeights}}, estados::EstadoJuego, objetivos::{ObjectiveState, ObjectiveType}};

fn pick_enemy(weights: &HashMap<EnemyType, f32>) -> EnemyType {
    let total: f32 = weights.values().sum();

    let mut rng = fastrand::Rng::new();
    let mut value = rng.f32() * total;

    for (e_type, weight) in weights {
        if value < *weight {
            return *e_type;
        }
        value -= *weight;
    }

    EnemyType::Normal
}

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    weights: Res<EnemyWeights>,
    healths: Res<EnemyHealths>,
    windows: Query<&Window>,
    mut obj_state: ResMut<ObjectiveState>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    println!("{:?}", obj_state.wave_spawned);

    if obj_state.has_waves {
        if obj_state.wave_spawned {
            return;
        }

        obj_state.wave_spawned = true;
    }

    let window = windows.single().unwrap();

    let mut rng = fastrand::Rng::new();

    let quantity = if obj_state.has_waves {
        obj_state.wave_size.unwrap_or(0) as usize
    } else {
        rng.usize(2..6)
    };

    for _ in 0..quantity {
        let e_type = pick_enemy(&weights.weights);

        let health = &healths.healths.get(&e_type).unwrap_or(&100);

        let x = rng.f32() * window.width() - window.width() / 2.0;
        let y = rng.f32() * window.height() - window.height() / 2.0;

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy { e_type, health: **health },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new(format!("{:?}", e_type)),
                Transform::from_xyz(0.0, 25.0, 1.0),
            ));
        });
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    query: Query<Entity, With<Enemy>>,
    mut next_state: ResMut<NextState<EstadoJuego>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }

    next_state.set(EstadoJuego::InGame);
}

pub fn spawn_destroy(
    mut commands: Commands,
    obj_state: Res<ObjectiveState>,
) {
    if obj_state.objective != ObjectiveType::Destroy {
        return;
    }

    commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Enemy { e_type: EnemyType::Tanque, health: 1000 },
            Destroy,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new(format!("DESTROY")),
                Transform::from_xyz(0.0, 25.0, 1.0),
            ));
        });
}

