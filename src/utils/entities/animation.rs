use bevy::prelude::*;


#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}