use bevy::prelude::*;
use super::*;
pub fn get_window(window: Query<&Window>, mut query: Query<&mut WindowSize,With<Bird>>) {
    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    let (_x, _y) = match window.position {
        WindowPosition::At(v) => (v.x as f32, v.y as f32),
        _ => (0., 0.),
    };
    query.get_single_mut().unwrap().x = width;
    query.get_single_mut().unwrap().y = height;
    // dbg!(width, height, x, y);
}