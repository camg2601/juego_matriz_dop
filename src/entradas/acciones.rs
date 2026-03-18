use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Accion {
    Saltar,
    Nadar,
    SonidoA,
    SonidoB,
    Iniciar,
    Pausar,
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
    Salir,
    Flotar,
}

#[derive(Message)]
pub struct AccionEjecutada {
    pub accion: Accion,
}
