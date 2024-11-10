use bevy::prelude::*;


#[derive(Debug, Component, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub handle: usize,
    pub speed: f32,
}


#[derive(Debug, Resource, Clone, Copy, Default, PartialEq, Eq, Reflect)]
#[reflect(Resource)]
pub enum PlayerState {
    #[default]
    Standing,
    Running,
    Dashing,
    Jumping,
    Climbing,
}