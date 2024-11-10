use bevy::prelude::*;

use crate::utils::{
    constants::camera::{
        CAMERA_MIN_MOVE_DISTANCE, CAMERA_MOVE_INTERPOLATE, CAMERA_ORIGIN, CAMERA_SCALE,
        CAMERA_TRANSFORM,
    },
    entities::{
        camera::{CameraState, MainCamera},
        player::Player,
    },
};

use super::window::WindowPlayGamePlugin;

pub struct CameraGamePlugin;

impl Plugin for CameraGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.insert_resource(CameraState::Following);
        app.add_plugins(WindowPlayGamePlugin);
        app.add_systems(Update, camera_follow);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            transform: Transform::from_xyz(
                CAMERA_TRANSFORM.0,
                CAMERA_TRANSFORM.1,
                CAMERA_TRANSFORM.2,
            ),
            projection: OrthographicProjection {
                scale: CAMERA_SCALE,
                viewport_origin: Vec2::new(CAMERA_ORIGIN.0, CAMERA_ORIGIN.1),
                ..default()
            },
            ..default()
        },
    ));
}

fn camera_follow(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    camera_state: Res<CameraState>,
) {
    if player.is_empty() {
        return;
    }
    if *camera_state != CameraState::Following {
        return;
    }
    let player_translation = player.single().translation.truncate();
    let camera_translation = camera.single().translation.truncate();
    let mut camera_transform = camera.single_mut();
    if camera_translation.distance(player_translation) < 0.1 {
        return;
    }
    if camera_translation.distance(player_translation) < CAMERA_MIN_MOVE_DISTANCE {
        camera_transform.translation.x = player_translation.x;
        camera_transform.translation.y = player_translation.y;
    }
    let camera_next_pos =
        camera_translation + (player_translation - camera_translation) * CAMERA_MOVE_INTERPOLATE;
    camera_transform.translation.x = camera_next_pos.x;
    camera_transform.translation.y = camera_next_pos.y;
}
