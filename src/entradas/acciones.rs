
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Accion {
    Saltar,
    Nadar,
    SonidoA,
    SonidoB,
    Iniciar,
    Pausar,
    MoverseEnTierra,
    MoverseEnAgua,
    MoverAdelanteAgua,
    MoverAdelanteTierra,
    MoverAtrasAgua,
    MoverAtrasTierra,
    MoverIzquierdaAgua,
    MoverIzquierdaTierra,
    MoverDerechaAgua,
    MoverDerechaTierra,   
    Salir,
    Flotar,
    
}

#[derive(Message)]
pub struct AccionEjecutada {
    pub accion: Accion,
}


