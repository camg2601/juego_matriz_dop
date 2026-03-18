use bevy::prelude::*;
use crate::{EstadoJuego};
use crate::entradas::AccionEjecutada;
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
) {
    timer.segundos = 0.0;
    info!("Reiniciando juego...");
    next_state.set(estado_inicial.0);
}