
use bevy::prelude::*;
use std::collections::HashMap;
use crate::entradas::Accion;

#[derive(Resource, Default)]
pub struct MapaDeSonidos {
   // Mapeo: Tecla -> Handle del AudioSource (el archivo de sonido)
   pub sonidos: HashMap<Accion, Handle<AudioSource>>,
  
   // Mapeo: Tecla -> Entidad (Entity) que está reproduciendo el audio actualmente.
   // Esto es lo que nos permite detenerlo.
   pub pista_activa: HashMap<Accion, Entity>,
}

