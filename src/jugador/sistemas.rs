use bevy::prelude::*;

use crate::EstadoJuego;
use crate::fisicas::{EnSuelo, Gravedad, Velocidad};
use crate::jugador::componentes::{EstadoVisualJugador, Jugador, MovimientoJugador};

const SPRITE_CAMINAR_DERECHA: &str = "caminar hacia la derecha.png";
const SPRITE_CAMINAR_IZQUIERDA: &str = "caminar hacia la izquierda.png";
const SPRITE_NADAR: &str = "nadando.png";
const SPRITE_SALTAR: &str = "saltar.png";

const FRAME_W_CAMINAR: u32 = 100;
const FRAME_H_CAMINAR: u32 = 100;
const FRAME_W_CAMINAR_DERECHA: u32 = 100;
const FRAME_H_CAMINAR_DERECHA: u32 = 200;
const FRAME_W_CAMINAR_IZQUIERDA: u32 = 100;
const FRAME_H_CAMINAR_IZQUIERDA: u32 = 200;
const FRAME_W_NADAR: u32 = 100;
const FRAME_H_NADAR: u32 = 200;
const FRAME_W_SALTAR: u32 = 100;
const FRAME_H_SALTAR: u32 = 200;
const COLUMNAS: u32 = 10;
const FILAS: u32 = 1;

#[derive(Resource)]
pub struct RecursosJugador {
    pub atlas_caminar_derecha: Handle<TextureAtlasLayout>,
    pub atlas_caminar_izquierda: Handle<TextureAtlasLayout>,
    pub atlas_nadar: Handle<TextureAtlasLayout>,
    pub atlas_saltar: Handle<TextureAtlasLayout>,
    pub textura_caminar_derecha: Handle<Image>,
    pub textura_caminar_izquierda: Handle<Image>,
    pub textura_nadar: Handle<Image>,
    pub textura_saltar: Handle<Image>,
}

pub fn configurar_visual(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let atlas_caminar_derecha = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(FRAME_W_CAMINAR_DERECHA, FRAME_H_CAMINAR_DERECHA),
        COLUMNAS,
        FILAS,
        None,
        None,
    ));
    let atlas_caminar_izquierda = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(FRAME_W_CAMINAR_IZQUIERDA, FRAME_H_CAMINAR_IZQUIERDA),
        COLUMNAS,
        FILAS,
        None,
        None,
    ));
    let atlas_nadar = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(FRAME_W_NADAR, FRAME_H_NADAR),
        COLUMNAS,
        FILAS,
        None,
        None,
    ));
    let atlas_saltar = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(FRAME_W_SALTAR, FRAME_H_SALTAR),
        COLUMNAS,
        FILAS,
        None,
        None,
    ));

    let textura_caminar_derecha: Handle<Image> = asset_server.load(SPRITE_CAMINAR_DERECHA);
    let textura_caminar_izquierda: Handle<Image> = asset_server.load(SPRITE_CAMINAR_IZQUIERDA);
    let textura_nadar: Handle<Image> = asset_server.load(SPRITE_NADAR);
    let textura_saltar: Handle<Image> = asset_server.load(SPRITE_SALTAR);

    commands.insert_resource(RecursosJugador {
        atlas_caminar_derecha: atlas_caminar_derecha.clone(),
        atlas_caminar_izquierda: atlas_caminar_izquierda.clone(),
        atlas_nadar: atlas_nadar.clone(),
        atlas_saltar: atlas_saltar.clone(),
        textura_caminar_derecha: textura_caminar_derecha.clone(),
        textura_caminar_izquierda: textura_caminar_izquierda.clone(),
        textura_nadar: textura_nadar.clone(),
        textura_saltar: textura_saltar.clone(),
    });

    commands.spawn((
        Sprite::from_atlas_image(
            textura_caminar_derecha,
            TextureAtlas {
                layout: atlas_caminar_derecha,
                index: 0,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Jugador,
        Velocidad { y: 0.0 },
        Gravedad,
        EnSuelo(true),
        EstadoVisualJugador::default(),
    ));
}

pub fn sistema_seleccion_movimiento(
    teclado: Res<ButtonInput<KeyCode>>,
    estado_juego: Res<State<EstadoJuego>>,
    query_fisica: Query<&EnSuelo, With<Jugador>>,
    mut query_visual: Query<&mut EstadoVisualJugador, With<Jugador>>,
) {
    let en_suelo = query_fisica.iter().next().map(|valor| valor.0).unwrap_or(true);

    for mut estado_visual in &mut query_visual {
        let nuevo_movimiento = match estado_juego.get() {
            EstadoJuego::JugandoEnAgua => MovimientoJugador::Nadar,
            EstadoJuego::JugandoEnTierra => {
                if !en_suelo {
                    MovimientoJugador::Saltar
                } else if teclado.pressed(KeyCode::KeyA) {
                    MovimientoJugador::CaminarIzquierda
                } else if teclado.pressed(KeyCode::KeyD) {
                    MovimientoJugador::CaminarDerecha
                } else {
                    MovimientoJugador::Quieto
                }
            }
            _ => MovimientoJugador::Quieto,
        };

        if nuevo_movimiento != estado_visual.movimiento_actual {
            estado_visual.reiniciar_animacion(nuevo_movimiento);
        }
    }
}

pub fn sistema_animacion_jugador(
    time: Res<Time>,
    recursos: Res<RecursosJugador>,
    mut query: Query<(&mut EstadoVisualJugador, &mut Sprite), With<Jugador>>,
) {
    for (mut estado_visual, mut sprite) in &mut query {
        if estado_visual.es_animado() {
            estado_visual.timer.tick(time.delta());
            if estado_visual.timer.just_finished() {
                estado_visual.frame_actual =
                    (estado_visual.frame_actual + 1) % estado_visual.total_frames;
            }
        } else {
            estado_visual.frame_actual = 0;
        }

        let (textura, atlas_layout, frame_index) = match estado_visual.movimiento_actual {
            MovimientoJugador::Quieto => (
                recursos.textura_caminar_derecha.clone(),
                recursos.atlas_caminar_derecha.clone(),
                0,
            ),
            MovimientoJugador::CaminarDerecha => (
                recursos.textura_caminar_derecha.clone(),
                recursos.atlas_caminar_derecha.clone(),
                estado_visual.frame_actual,
            ),
            MovimientoJugador::CaminarIzquierda => (
                recursos.textura_caminar_izquierda.clone(),
                recursos.atlas_caminar_izquierda.clone(),
                estado_visual.frame_actual,
            ),
            MovimientoJugador::Saltar => (
                recursos.textura_saltar.clone(),
                recursos.atlas_saltar.clone(),
                estado_visual.frame_actual,
            ),
            MovimientoJugador::Nadar => (
                recursos.textura_nadar.clone(),
                recursos.atlas_nadar.clone(),
                estado_visual.frame_actual,
            ),
        };

        sprite.image = textura;
        sprite.texture_atlas = Some(TextureAtlas {
            layout: atlas_layout,
            index: frame_index,
        });
    }
}
