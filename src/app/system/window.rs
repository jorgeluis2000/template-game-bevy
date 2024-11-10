use bevy::prelude::*;

use crate::{core::characters::player::PlayerPlugin, utils::constants::window::{WINDOW_HEIGHT, WINDOW_WEIGHT}};

pub struct WindowPlayGamePlugin;

impl Plugin for WindowPlayGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_plugins(PlayerPlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::linear_rgb(0.1, 0.1, 0.2),
            custom_size: Some(Vec2::new(WINDOW_WEIGHT, WINDOW_HEIGHT)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn(
        TextBundle::from_section(
            "Bienvenido al juego!",
            TextStyle {
                color: Color::WHITE,
                font_size: 60.0,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            align_self: AlignSelf::Center,
            ..Default::default()
        }),
    );
}
