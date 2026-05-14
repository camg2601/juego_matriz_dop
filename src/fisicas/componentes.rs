use bevy::prelude::*;

#[derive(Component)]
pub struct Velocidad {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Gravedad {
    pub t: f32,
    pub a: f32,
}

#[derive(Component)]
pub struct Aceleracion {
    pub t: f32,
    pub a: f32,
}

#[derive(Component)]
pub struct EnSuelo(pub bool);