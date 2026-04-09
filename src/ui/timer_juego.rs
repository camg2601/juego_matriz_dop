use bevy::prelude::*;

use crate::EstadoJuego;
use crate::estados::EstadoPausa;

#[derive(Resource, Default)]
pub struct TimerJuego {
    pub segundos: f32,
}

#[derive(Component)]
struct TimerJuegoContenedor;

#[derive(Component)]
struct TimerJuegoTexto;

pub struct TimerJuegoPlugin;

impl Plugin for TimerJuegoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimerJuego>()
            .add_systems(Startup, crear_ui_timer)
            .add_systems(
                Update,
                actualizar_timer
                    .run_if(
                        in_state(EstadoJuego::JugandoEnTierra)
                            .or(in_state(EstadoJuego::JugandoEnAgua)),
                    )
                    .run_if(in_state(EstadoPausa::Activo)),
            )
            .add_systems(Update, (actualizar_texto_timer, actualizar_visibilidad_timer));
    }
}

fn crear_ui_timer(mut commands: Commands) {
    commands
        .spawn((
            TimerJuegoContenedor,
            Node {
                position_type: PositionType::Absolute,
                right: px(16.0),
                top: px(16.0),
                padding: UiRect::axes(px(12.0), px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.45)),
            ZIndex(30),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                TimerJuegoTexto,
                Text::new("Tiempo: 00:00"),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn actualizar_timer(time: Res<Time>, mut timer: ResMut<TimerJuego>) {
    timer.segundos += time.delta_secs();
}

fn actualizar_texto_timer(timer: Res<TimerJuego>, mut query: Query<&mut Text, With<TimerJuegoTexto>>) {
    if !timer.is_changed() {
        return;
    }

    let total_segundos = timer.segundos.max(0.0).floor() as u32;
    let minutos = total_segundos / 60;
    let segundos = total_segundos % 60;

    for mut texto in &mut query {
        texto.0 = format!("Tiempo: {:02}:{:02}", minutos, segundos);
    }
}

fn actualizar_visibilidad_timer(
    estado_juego: Res<State<EstadoJuego>>,
    estado_pausa: Res<State<EstadoPausa>>,
    mut query: Query<&mut Visibility, With<TimerJuegoContenedor>>,
) {
    let visible = matches!(
        estado_juego.get(),
        EstadoJuego::JugandoEnTierra | EstadoJuego::JugandoEnAgua
    ) || matches!(estado_pausa.get(), EstadoPausa::Pausa);

    for mut visibilidad in &mut query {
        *visibilidad = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
