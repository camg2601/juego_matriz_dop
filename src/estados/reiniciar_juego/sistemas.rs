use bevy::prelude::*;
use crate::{EstadoJuego};
use crate::entradas::AccionEjecutada;
use crate::fisicas::{EnSuelo, Velocidad};
use crate::jugador::componentes::Jugador;
use crate::ui::TimerJuego;

#[derive(Resource)]
pub struct EstadoInicialJuego(pub EstadoJuego);

impl Default for EstadoInicialJuego {
    fn default() -> Self {
        Self(EstadoJuego::JugandoEnTierra)
    }
}

pub fn limpiar_acciones_pendientes(mut mensajes: ResMut<Messages<AccionEjecutada>>) {
    mensajes.clear();
}

pub fn reiniciar_juego(
    mut next_state: ResMut<NextState<EstadoJuego>>,
    estado_inicial: Res<EstadoInicialJuego>,
    mut timer: ResMut<TimerJuego>,
    mut jugador_query: Query<(&mut Transform, &mut Velocidad, &mut EnSuelo), With<Jugador>>,
) {
    timer.segundos = 0.0;

    for (mut transform, mut velocidad, mut en_suelo) in &mut jugador_query {
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
        velocidad.y = 0.0;
        en_suelo.0 = true;
    }

    info!("Reiniciando juego...");
    next_state.set(estado_inicial.0);
}