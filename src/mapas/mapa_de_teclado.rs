use bevy::prelude::*;
use std::collections::HashMap;
use crate::entradas::Accion;

#[derive(Resource)]
pub struct MapaEntradas {
    pub teclado: HashMap<KeyCode, Accion>,
}
//ESTO DEBERÍA HACERSE EN EL STARTUP CUANDO HAGA EL PLUGIN
impl Default for MapaEntradas {
    fn default() -> Self {
        let mut teclado = HashMap::new();

        teclado.insert(KeyCode::KeyW, Accion::Saltar);
        teclado.insert(KeyCode::KeyU, Accion::SonidoA);
        teclado.insert(KeyCode::KeyB, Accion::SonidoB);
        teclado.insert(KeyCode::KeyG, Accion::MoverseEnAgua);
        teclado.insert(KeyCode::KeyT, Accion::MoverseEnTierra);
        teclado.insert(KeyCode::KeyS, Accion::Salir);
        teclado.insert(KeyCode::KeyP, Accion::Pausar);
        teclado.insert(KeyCode::KeyF, Accion::Flotar);
       

        Self { teclado }
    }
}
