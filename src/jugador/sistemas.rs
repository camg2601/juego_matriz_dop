use bevy::prelude::*;
use crate::fisicas::{Velocidad, Gravedad, EnSuelo, Aceleracion};
use crate::jugador::componentes::Jugador;

pub fn configurar_visual(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Jugador,
        Velocidad { x: 0.0, y: 0.0 },
        Gravedad { t: 1800.0, a: 0.0 },
        Aceleracion { t: 2000.0, a: 600.0 },
        EnSuelo(true),
    ));
}