use bevy::prelude::*;

use crate::utils::{
    constants::{
        player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WEIGHT},
        window::{WINDOW_HEIGHT, WINDOW_WEIGHT},
    },
    entities::{animation::AnimationIndices, player::{Player, PlayerState}},
};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (player_movement, animate_sprite));
        app.insert_resource(PlayerState::Standing);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Configurando Texturas jugador");
    let texture: Handle<Image>  = asset_server.load("sprites/player-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout     = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 3 };
    commands.spawn((
        Player {
            handle: 0,
            speed: 300.0,
        },
        SpriteBundle {
            // sprite: Sprite {
            //     color: Color::linear_rgb(0.0, 1.0, 0.0),
            //     custom_size: Some(Vec2::new(PLAYER_SIZE_WEIGHT, PLAYER_SIZE_HEIGHT)),
            //     ..Default::default()
            // },
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    if let Some((player, mut transform)) = query.iter_mut().next() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 2.0; // Mover hacia arriba
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 2.0; // Mover hacia abajo
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 2.0; // Mover hacia la izquierda
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 2.0; // Mover hacia la derecha
        }

        // Normaliza el vector de movimiento para que el jugador no se mueva más rápido en diagonal
        direction = direction.normalize_or_zero();

        // Ajustar el movimiento en función del tiempo por fotograma
        let movement = direction * player.speed * time.delta_seconds(); // delta_seconds() da el tiempo entre fotogramas

        // Actualiza la posición del jugador, respetando los límites de la pantalla
        let mut new_position = transform.translation + movement;

        let limit_x = WINDOW_WEIGHT - PLAYER_SIZE_WEIGHT;
        let limit_y = WINDOW_HEIGHT - PLAYER_SIZE_HEIGHT;
        // Limitar la posición del jugador dentro de los márgenes de la pantalla
        new_position.x = new_position.x.clamp(-limit_x / 2.0, limit_x / 2.0);
        new_position.y = new_position.y.clamp(-limit_y / 2.0, limit_y / 2.0);

        // Actualizar la posición del jugador
        transform.translation = new_position;
    }
}


fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}