use bevy::prelude::*;

mod inicio;
mod sistemas;

use sistemas::*;
use inicio::*;

pub struct TrianguloPlugin;

impl Plugin for TrianguloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_triangulo_atlas)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, spawn_con_click)
            .add_systems(Update, gravedad)
            .add_systems(Update, animar_triangulo)
            .add_message::<ColisionJugadorTriangulo>()
            .init_resource::<ColisionesActivas>()
            .add_systems(Update, detectar_colision_jugador)
            .add_systems(Update, reaccionar_colision)
            .add_systems(Update, actualizar_rebote)
            .add_systems(Update, actualizar_despawn);
    }
}
