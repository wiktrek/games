/*
    I will fix pipes later
*/
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::prelude::*;
#[derive(Component)]
struct Bird;

#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct Pipe;
#[derive(Component)]
struct Score {
    value: i32
}
const BIRD_SIZE: f32 = 90.;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default()))
        .add_systems(Startup, (setup, spawn_bird))
        .add_systems(Update, (bird_gravity, jump, update_score, move_pipes, pipe_bind))
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
fn restart(mut commands: Commands) {
    spawn_bird(commands);
}
fn spawn_bird(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(BIRD_SIZE, BIRD_SIZE)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-160., 10., 0.),
            ..Default::default()
        },
     ..default()
    }, (Bird, Score { value: 0})));
    spawn_pipes(commands)
}
fn bird_gravity(time: Res<Time>, mut birds: Query<&mut Transform, With<Bird>>) {
    for mut bird in birds.iter_mut() {
            if bird.translation.y <= -(230. + BIRD_SIZE)  {
                // lost function()
                bird.translation.y = 0.;
            }
            bird.translation.y -= 200. * time.delta_seconds();
            
    }
}
fn jump(keyboard: Res<ButtonInput<KeyCode>>, mut bird: Query<&mut Transform, With<Bird>>) {
    if keyboard.just_released(KeyCode::Space) {
        let mut mut_bird = bird.get_single_mut().unwrap();
        if mut_bird.translation.y < 240. {
            println!("{}", mut_bird.translation.y);
            mut_bird.translation.y += 90.;
        }
    }
}

fn pipe_bind(keyboard: Res<ButtonInput<KeyCode>>, commands: Commands) {
    if keyboard.just_released(KeyCode::KeyK) {
        spawn_pipes(commands)
    };
}
fn update_score(bird: Query<&mut Score, With<Bird>>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    for mut text in score_text.iter_mut() {
        let score = bird.get_single().unwrap().value;
        text.sections[1].value = format!("{}", score);
    }
}
fn spawn_pipes(mut commands: Commands) {
    let pipe_sizes_y: Vec<i32> = vec![100, 200, 300, 400, 500, 600];
    let pipe_sizes_x: Vec<i32> = vec![100, 200, 250];
    let mut rng = rand::thread_rng();
    let x = pipe_sizes_x[rng.gen_range(0..pipe_sizes_x.len())] as f32;
    let y = pipe_sizes_y[rng.gen_range(0..pipe_sizes_y.len())] as f32;
    // down pipe
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(x,y)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(300., -440. + y, 0.),
            ..Default::default()
        },
     ..default()
    }, Pipe));
    // up pipe
    println!("{} {}", x, 440. - y);
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(x,y)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(300., 440. - y / 2., 0.),
            ..Default::default()
        },
     ..default()
    }, Pipe));
}
fn move_pipes(time: Res<Time>, mut pipes: Query<&mut Transform, With<Pipe>>) {
    for mut pipe in pipes.iter_mut() {
        pipe.translation.x -= 170. * time.delta_seconds()
    }
}
fn bird_touch(pipes: Query<(&mut Transform, Entity), With<Pipe>>, bird: Query<&mut Transform, With<Bird>>) {
}