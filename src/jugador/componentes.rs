use bevy::prelude::*;

#[derive(Component)]
pub struct Jugador;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovimientoJugador {
    Quieto,
    CaminarDerecha,
    CaminarIzquierda,
    Saltar,
    Nadar,
}

#[derive(Component)]
pub struct EstadoVisualJugador {
    pub movimiento_actual: MovimientoJugador,
    pub timer: Timer,
    pub frame_actual: usize,
    pub total_frames: usize,
}

impl Default for EstadoVisualJugador {
    fn default() -> Self {
        Self {
            movimiento_actual: MovimientoJugador::Quieto,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame_actual: 0,
            total_frames: 10,
        }
    }
}

impl EstadoVisualJugador {
    pub fn reiniciar_animacion(&mut self, nuevo_movimiento: MovimientoJugador) {
        self.movimiento_actual = nuevo_movimiento;
        self.frame_actual = 0;
        self.timer.reset();
    }

    pub fn es_animado(&self) -> bool {
        !matches!(self.movimiento_actual, MovimientoJugador::Quieto)
    }
}
