use bevy::prelude::*;
use bevy::image::TextureAtlas;
use bevy::log::info;
use super::inicio::{Triangulo, TrianguloAtlas};
use rand::Rng;
use std::collections::HashSet;
use crate::jugador::componentes::Jugador;

#[derive(Component)]
pub struct TrianguloAnim {
    pub timer: Timer,
    pub frame_count: usize,
}

#[derive(Message)]
pub struct ColisionJugadorTriangulo {
    pub jugador: Entity,
    pub triangulo: Entity,
}

#[derive(Resource, Default)]
pub struct ColisionesActivas(pub HashSet<(Entity, Entity)>);

#[derive(Component)]
pub struct Rebote {
    pub velocidad_y: f32,
}

#[derive(Component)]
pub struct DespawnTimer {
    pub timer: Timer,
}

pub fn gravedad(
    mut query: Query<&mut Transform, With<Triangulo>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= 200.0 * time.delta_secs();
    }
}

pub fn spawn_con_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    atlas: Res<TrianguloAtlas>,
) {
    if buttons.just_pressed(MouseButton::Left) {

        let mut rng = rand::rng();
        let position = rng.random_range(-500.0..500.0);

        commands.spawn((
            Sprite {
                image: atlas.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas.layout.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..default()
            },
            Transform::from_xyz(position, 400.0, 0.0),
            Triangulo,
            TrianguloAnim {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                frame_count: atlas.frame_count,
            },
        ));
    }
}

pub fn animar_triangulo(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut TrianguloAnim), With<Triangulo>>,
) {
    for (mut sprite, mut anim) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = (atlas.index + 1) % anim.frame_count;
            }
        }
    }
}

pub fn detectar_colision_jugador(
    mut colisiones: ResMut<ColisionesActivas>,
    mut eventos: MessageWriter<ColisionJugadorTriangulo>,
    q_jugador: Query<(Entity, &Transform, &Sprite), With<Jugador>>,
    q_triangulo: Query<(Entity, &Transform, &Sprite), With<Triangulo>>,
) {
    let (jugador_entity, jugador_transform, jugador_sprite) = match q_jugador.single() {
        Ok(data) => data,
        Err(_) => return,
    };

    let jugador_size = sprite_size(jugador_sprite);
    if jugador_size == Vec2::ZERO {
        return;
    }

    for (triangulo_entity, triangulo_transform, triangulo_sprite) in &q_triangulo {
        let triangulo_size = sprite_size(triangulo_sprite);
        if triangulo_size == Vec2::ZERO {
            continue;
        }

        let colisiona = aabb_overlap(
            jugador_transform.translation,
            jugador_size,
            triangulo_transform.translation,
            triangulo_size,
        );

        let key = (jugador_entity, triangulo_entity);
        if colisiona {
            if !colisiones.0.contains(&key) {
                colisiones.0.insert(key);
                eventos.write(ColisionJugadorTriangulo {
                    jugador: jugador_entity,
                    triangulo: triangulo_entity,
                });
            }
        } else {
            colisiones.0.remove(&key);
        }
    }
}

pub fn reaccionar_colision(
    mut commands: Commands,
    mut eventos: MessageReader<ColisionJugadorTriangulo>,
    q_triangulo: Query<(), With<Triangulo>>,
) {
    for evento in eventos.read() {
        if q_triangulo.get(evento.triangulo).is_ok() {
            commands.entity(evento.triangulo).insert((
                Rebote { velocidad_y: 220.0 },
                DespawnTimer {
                    timer: Timer::from_seconds(0.25, TimerMode::Once),
                },
            ));
            info!("Colision: triangulo toca al jugador");
        }
    }
}

pub fn actualizar_rebote(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Rebote), With<Triangulo>>,
) {
    for (entity, mut transform, mut rebote) in &mut query {
        let dt = time.delta_secs();
        transform.translation.y += rebote.velocidad_y * dt;
        rebote.velocidad_y -= 700.0 * dt;

        if rebote.velocidad_y <= 0.0 {
            commands.entity(entity).remove::<Rebote>();
        }
    }
}

pub fn actualizar_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnTimer), With<Triangulo>>,
) {
    for (entity, mut timer) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn sprite_size(sprite: &Sprite) -> Vec2 {
    sprite.custom_size.unwrap_or(Vec2::ZERO)
}

fn aabb_overlap(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    let a_min = a_pos.truncate() - (a_size / 2.0);
    let a_max = a_pos.truncate() + (a_size / 2.0);
    let b_min = b_pos.truncate() - (b_size / 2.0);
    let b_max = b_pos.truncate() + (b_size / 2.0);

    a_min.x <= b_max.x
        && a_max.x >= b_min.x
        && a_min.y <= b_max.y
        && a_max.y >= b_min.y
}
