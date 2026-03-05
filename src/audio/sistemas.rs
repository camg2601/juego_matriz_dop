use bevy::prelude::*;
use bevy::audio::{AudioPlayer, PlaybackSettings};
use crate::entradas::AccionEjecutada;
use crate::mapas::MapaDeSonidos;


pub fn reproducir_sonidos(
    mut reader: MessageReader<AccionEjecutada>,
    mut mapa: ResMut<MapaDeSonidos>,
    mut commands: Commands,
) {
    for evento in reader.read() {

        let handle = if let Some(h) = mapa.sonidos.get(&evento.accion) {
            h.clone()
        } else {
            continue;
        };

        if let Some(entidad) = mapa.pista_activa.remove(&evento.accion) {
            commands.entity(entidad).despawn();
        }

        let nueva_entidad = commands.spawn((
            AudioPlayer::new(handle),
            PlaybackSettings::default(),
        )).id();

        mapa.pista_activa.insert(evento.accion, nueva_entidad);
    }
}
