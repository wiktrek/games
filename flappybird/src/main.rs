use bevy::{prelude::*, render::render_resource::BindGroupDescriptor};
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[derive(Component)]
struct Bird;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default()))
        .add_systems(Startup, (setup, spawn_bird))
        .add_systems(Update, (bird_gravity, jump))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}
fn spawn_bird(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(90., 90.)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10., 10., 0.),
            ..Default::default()
        },
     ..default()
    }, Bird));
}
fn bird_gravity(mut time: Res<Time>, mut birds: Query<&mut Transform, With<Bird>>) {
    for mut bird in birds.iter_mut() {
            bird.translation.y -= 50. * time.delta_seconds();
    }
}
fn jump(keyboard: Res<ButtonInput<KeyCode>>, mut bird: Query<&mut Transform, With<Bird>>) {
    if keyboard.just_released(KeyCode::Space) {
        bird.get_single_mut().unwrap().translation.y += 50.;
    }
}