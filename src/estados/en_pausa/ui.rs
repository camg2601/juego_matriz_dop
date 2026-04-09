use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct PausaUi;

pub fn mostrar_ui_pausa(mut commands: Commands) {
	info!("Entrando en PAUSA");

	commands
		.spawn((
			PausaUi,
			Node {
				width: percent(100.0),
				height: percent(100.0),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				position_type: PositionType::Absolute,
				..default()
			},
			BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
			ZIndex(100),
		))
		.with_children(|parent| {
			parent.spawn((
				Text::new("Pausa"),
				TextFont {
					font_size: 72.0,
					..default()
				},
				TextColor(Color::WHITE),
			));
		});
}

pub fn ocultar_ui_pausa(mut commands: Commands, query: Query<Entity, With<PausaUi>>) {
	info!("Saliendo de PAUSA");

	for entidad in &query {
		commands.entity(entidad).despawn_children();
		commands.entity(entidad).despawn();
	}
}
