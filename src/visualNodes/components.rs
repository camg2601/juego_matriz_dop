use bevy::prelude::*;

#[derive(Component)]
pub struct NodoSeleccion {
    pub id: usize,
}

#[derive(Component)]
pub struct TextoUI;

#[derive(Component)]
pub struct MiniMapa;

#[derive(Message)]
pub struct Completed {
    pub complete: bool,
}

#[derive(Message)]
pub struct Wave {
    pub complete: bool,
}