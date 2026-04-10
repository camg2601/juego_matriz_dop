use bevy::prelude::*;
use crate::mapas::MapaDeSonidos;
use crate::entradas::Accion;


// -----------------------------------------------------------------------------
// STARTUP: Cargar sonidos y registrar el recurso
// -----------------------------------------------------------------------------
pub fn cargar_sonidos(
   asset_server: Res<AssetServer>,
   mut commands: Commands,
) {
   let mut mapa_de_sonidos = MapaDeSonidos::default();

   // Cargar los archivos de sonido (Asegúrate de que estos archivos existen en la carpeta "assets/sonidos/")
   mapa_de_sonidos.sonidos.insert(Accion::SonidoA, asset_server.load("sonidos/s.ogg"));
   mapa_de_sonidos.sonidos.insert(Accion::SonidoB, asset_server.load("sonidos/u.ogg"));

   // Registrar como recurso global
   commands.insert_resource(mapa_de_sonidos);

   info!("Sonidos cargados y mapa global inicializado.");
}