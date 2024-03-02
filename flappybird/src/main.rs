use bevy::{prelude::*, render::render_resource::encase::internal::SizeValue};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::prelude::*;
#[derive(Component)]
struct Bird;

#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct Tube;
#[derive(Component)]
struct Score {
    value: i32
}
const BIRD_SIZE: f32 = 90.;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default()))
        .add_systems(Startup, (setup, spawn_bird, spawn_tubes))
        .add_systems(Update, (bird_gravity, jump, update_score, move_tubes))
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
}
fn bird_gravity(time: Res<Time>, mut birds: Query<&mut Transform, With<Bird>>) {
    for mut bird in birds.iter_mut() {
            if bird.translation.y <= -(230. + BIRD_SIZE)  {
                // lost function()
                bird.translation.y = 0.;
            }
            bird.translation.y -= 100. * time.delta_seconds();
            
    }
}
fn jump(keyboard: Res<ButtonInput<KeyCode>>, mut bird: Query<&mut Transform, With<Bird>>) {
    if keyboard.just_released(KeyCode::Space) {
        bird.get_single_mut().unwrap().translation.y += 180.;
    }
}
fn update_score(bird: Query<&mut Score, With<Bird>>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    for mut text in score_text.iter_mut() {
        let score = bird.get_single().unwrap().value;
        text.sections[1].value = format!("{}", score);
    }
}
fn spawn_tubes(mut commands: Commands) {
    let tube_sizes_y: Vec<i32> = vec![100, 200, 300];
    let tube_sizes_x: Vec<i32> = vec![300, 200];
    let mut rng = rand::thread_rng();
    let x = tube_sizes_x[rng.gen_range(0..tube_sizes_x.len())] as f32;
    let y = tube_sizes_y[rng.gen_range(0..tube_sizes_y.len())] as f32;

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(x,y)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(300., -290., 0.),
            ..Default::default()
        },
     ..default()
    }, Tube));
}
fn move_tubes(time: Res<Time>, mut tubes: Query<&mut Transform, With<Tube>>) {
    for mut tube in tubes.iter_mut() {
        tube.translation.x -= 50. * time.delta_seconds()
    }
}
fn bird_touch(tubes: Query<(&mut Transform, Sprite), With<Tube>>, bird: Query<&mut Transform, With<Bird>>) {
}