use bevy::prelude::*;
use bevy::image::TextureAtlasLayout;

use crate::triangulo::TrianguloPlugin;

#[derive(Component)]
pub struct Triangulo;

#[derive(Resource, Clone)]
pub struct TrianguloAtlas {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub frame_count: usize,
}

const ATLAS_COLUMNS: u32 = 4;
const ATLAS_ROWS: u32 = 5;
const FRAME_SIZE: UVec2 = UVec2::new(256, 256);

pub fn spawn_triangulo(
    mut commands: Commands,
) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(1.0, 1.0, 1.0),
            Vec2::new(70.0, 70.0)
        ),
        Transform::from_xyz(0.0, 100.0, 0.0),
        Triangulo,
    ));
}

pub fn setup_triangulo_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("triangulo_sprite.png");
    let layout = TextureAtlasLayout::from_grid(
        FRAME_SIZE,
        ATLAS_COLUMNS,
        ATLAS_ROWS,
        None,
        None,
    );
    let layout_handle = layouts.add(layout);

    commands.insert_resource(TrianguloAtlas {
        image,
        layout: layout_handle,
        frame_count: (ATLAS_COLUMNS * ATLAS_ROWS) as usize,
    });
}

pub fn spawn_camera(
    mut commands: Commands
){
    commands.spawn(Camera2d);
}
