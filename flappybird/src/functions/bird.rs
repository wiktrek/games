use bevy::prelude::*;
use super::*;
pub fn bird_gravity(time: Res<Time>, mut birds: Query<(&mut Transform, &mut WindowSize), With<Bird>>) {
    let mut bird = birds.get_single_mut().unwrap();
    if bird.0.translation.y <= -(bird.1.y / 2. - BIRD_SIZE / 2.) { 
        // (230. + BIRD_SIZE)
        // lost function()
        bird.0.translation.y = 0.;
    }
    bird.0.translation.y -= 200. * time.delta_seconds();
}
pub fn jump(keyboard: Res<ButtonInput<KeyCode>>, mut bird: Query<&mut Transform, With<Bird>>) {
    if keyboard.just_released(KeyCode::Space) {
        let mut mut_bird = bird.get_single_mut().unwrap();
        if mut_bird.translation.y < 240. {
            println!("{}", mut_bird.translation.y);
            mut_bird.translation.y += 90.;
        }
        return
    }
}
pub fn bird_touch(pipes: Query<(&mut Transform, Entity), With<Pipe>>, bird: Query<&mut Transform, With<Bird>>) {
}
pub fn spawn_bird(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0., 255., 0.),
            custom_size: Some(Vec2::new(BIRD_SIZE, BIRD_SIZE)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-160., 10., 0.),
            ..Default::default()
        },
     ..default()
    }, (Bird, Score { value: 0}, WindowSize { x: 1280., y: 720.})));
}