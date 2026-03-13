use crate::entradas::Accion;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Resource, Deserialize)]
pub struct MapaEntradas {
    pub teclado: HashMap<KeyCode, Accion>,
}
//ESTO DEBERÍA HACERSE EN EL STARTUP CUANDO HAGA EL PLUGIN
impl Default for MapaEntradas {
    fn default() -> Self {
        if let Ok(json_str) = fs::read_to_string("config.json") {
            match serde_json::from_str::<MapaEntradas>(&json_str) {
                Ok(config) => {
                    println!("config.json cargada exitosamente!");
                    return config;
                }
                Err(e) => {
                    eprintln!(
                        "config.json existe pero no se pudo leer bien: {}. Usando predeterminados.",
                        e
                    );
                }
            }
        } else {
            println!("No se encontró config.json. Usando mapa de teclado predeterminado.");
        }

        // Se deja el mapa predeterminado por si es utilizado por otra version de la clase para prevenir errores
        let mut teclado = HashMap::new();

        teclado.insert(KeyCode::KeyW, Accion::Saltar);
        teclado.insert(KeyCode::KeyU, Accion::SonidoA);
        teclado.insert(KeyCode::KeyB, Accion::SonidoB);
        teclado.insert(KeyCode::ArrowUp, Accion::Arriba);
        teclado.insert(KeyCode::ArrowDown, Accion::Abajo);
        teclado.insert(KeyCode::ArrowLeft, Accion::Izquierda);
        teclado.insert(KeyCode::ArrowRight, Accion::Derecha);
        teclado.insert(KeyCode::KeyS, Accion::Salir);
        teclado.insert(KeyCode::KeyP, Accion::Pausar);
        teclado.insert(KeyCode::KeyF, Accion::Flotar);
        teclado.insert(KeyCode::KeyN, Accion::Nadar);
        teclado.insert(KeyCode::Enter, Accion::Iniciar);

        Self { teclado }
    }
}
