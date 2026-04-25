use bevy::prelude::*;
// normal
// rapido
// tanque
// rango
// soporte
// limitador

use std::collections::HashMap;

use crate::entities::{components::Enemy, entity_types::EnemyType, resources::{EnemyHealths, EnemyWeights}};

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
    weights: Res<EnemyWeights>,
    healths: Res<EnemyHealths>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();

    let mut rng = fastrand::Rng::new();

    let quantity = rng.usize(2..6);

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
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

