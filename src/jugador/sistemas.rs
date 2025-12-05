use bevy::prelude::*;
use crate::fisicas::{Velocidad, Gravedad, EnSuelo};
use crate::jugador::componentes::Jugador;

pub fn configurar_visual(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Jugador,
        Velocidad { y: 0.0 },
        Gravedad,
        EnSuelo(true),
    ));
}
