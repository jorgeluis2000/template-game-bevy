use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Event)]
pub struct CameraShakeEvent;

#[derive(Debug, Resource, Clone, Copy, Default, PartialEq, Eq)]
pub enum CameraState {
    #[default]
    Following,
    Shaking,
}

#[derive(Component)]
pub struct MainCamera;