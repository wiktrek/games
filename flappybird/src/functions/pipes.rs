use bevy::prelude::*;
use rand::prelude::*;
use super::*;
pub fn spawn_pipes(commands: Commands, window_size: Query<&mut WindowSize, With<Bird>>) {
    let x = window_size.get_single().unwrap().x;
    let y = window_size.get_single().unwrap().y;
    random(commands,Vec2 { x , y});
    fn random(mut commands: Commands, window_size: Vec2) {
        // get random y
        fn y() -> f32{
            let mut rng = rand::thread_rng();
            let pipe_sizes_y: Vec<i32> = vec![100, 200, 300, 400, 500, 600];
            let y = pipe_sizes_y[rng.gen_range(0..pipe_sizes_y.len())] as f32;
            y
        }
        let x = 200.;
        let y1 = y();
        let y2 = y();
        if y1 + y2 + 140. > window_size.y {
          return random(commands, window_size)
        }
        // down
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(255., 50., 50., 1.),
                custom_size: Some(Vec2::new(x, y1)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(300., -(window_size.y / 2.) + y1 /  2., 0.),
                ..Default::default()
            },
            ..default()
        }, Pipe));
        // up
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(255., 50., 50., 1.),
                custom_size: Some(Vec2::new(x, y2)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(300., (window_size.y / 2.) - y2 /  2., 0.),
                ..Default::default()
            },
            ..default()
        }, Pipe));
    }
}
pub fn move_pipes(time: Res<Time>, mut pipes: Query<&mut Transform, With<Pipe>>) {
    for mut pipe in pipes.iter_mut() {
        pipe.translation.x -= 170. * time.delta_seconds()
    }
}