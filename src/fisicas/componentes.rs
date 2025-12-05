use bevy::prelude::*;

#[derive(Component)]
pub struct Velocidad {
    pub y: f32,
}

#[derive(Component)]
pub struct Gravedad;

#[derive(Component)]
pub struct EnSuelo(pub bool);
