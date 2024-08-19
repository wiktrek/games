use bevy::prelude::*;
#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct ScoreText;
#[derive(Component)]
pub struct Pipe;
#[derive(Component)]
pub struct Score {
    pub value: i32
}
#[derive(Component)]
pub struct WindowSize {
    pub x: f32, 
    pub y: f32
}
// Constants
pub const BIRD_SIZE: f32 = 90.;