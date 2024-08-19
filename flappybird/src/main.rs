use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::prelude::*;
use std::time::Duration;
mod functions;
use functions::*;
const BIRD_SIZE: f32 = 90.;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default()))
        .add_systems(Startup, (setup, spawn_bird))
        .add_systems(Update, (bird_gravity, update_score, move_pipes, get_bind, spawn_pipes.run_if(on_timer(Duration::from_secs(3))), get_window.run_if(on_timer(Duration::from_millis(100))),))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts\\FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());
        commands.spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(ScoreText);
}
fn spawn_bird(mut commands: Commands) {
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
fn bird_gravity(time: Res<Time>, mut birds: Query<(&mut Transform, &mut WindowSize), With<Bird>>) {
    let mut bird = birds.get_single_mut().unwrap();
    if bird.0.translation.y <= -(bird.1.y / 2. - BIRD_SIZE / 2.) { 
        // (230. + BIRD_SIZE)
        // lost function()
        bird.0.translation.y = 0.;
    }
    bird.0.translation.y -= 200. * time.delta_seconds();
}
fn get_bind(keyboard: Res<ButtonInput<KeyCode>>, mut bird: Query<&mut Transform, With<Bird>>, commands: Commands) {
    // jump
    if keyboard.just_released(KeyCode::Space) {
        let mut mut_bird = bird.get_single_mut().unwrap();
        if mut_bird.translation.y < 240. {
            println!("{}", mut_bird.translation.y);
            mut_bird.translation.y += 90.;
        }
        return
    }
}
fn update_score(bird: Query<&mut Score, With<Bird>>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    for mut text in score_text.iter_mut() {
        let score = bird.get_single().unwrap().value;
        text.sections[1].value = format!("{}", score);
    }
}
fn spawn_pipes(mut commands: Commands, window_size: Query<&mut WindowSize, With<Bird>>) {
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
fn move_pipes(time: Res<Time>, mut pipes: Query<&mut Transform, With<Pipe>>) {
    for mut pipe in pipes.iter_mut() {
        pipe.translation.x -= 170. * time.delta_seconds()
    }
}
fn bird_touch(pipes: Query<(&mut Transform, Entity), With<Pipe>>, bird: Query<&mut Transform, With<Bird>>) {
}
