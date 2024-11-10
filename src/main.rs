use app::system::camera::CameraGamePlugin;

use bevy::prelude::*;
use utils::{constants::window::{WINDOW_HEIGHT, WINDOW_WEIGHT}, entities::player::{Player, PlayerState}};

mod app;
mod core;
mod utils;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cube Move".into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: (WINDOW_WEIGHT, WINDOW_HEIGHT).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(CameraGamePlugin)
        .register_type::<PlayerState>()
        .register_type::<Player>()
        .run();
}
