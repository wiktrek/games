// #[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy::{prelude::*, sprite::MeshMaterial2d,     color::palettes::basic};
enum Side_Color {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    White
}

#[derive(Component)]
struct Side {
    color: Side_Color,
    num: i32
}
fn main() {
    App::new()
        .add_plugins((DefaultPlugins,EmbeddedAssetPlugin::default(), WorldInspectorPlugin::new()))
        .add_systems(Startup, setup)
        // .add_systems(Update, )
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    create_rubiks_cube(commands, meshes, materials);
}
fn create_rubiks_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..53 {
        let (x,y,z) = get_xyz(i);
        let mut color: Color;
        match i {
            0..=8 => color = Color::from(basic::YELLOW),
            9..=17 => color = Color::Srgba(Srgba {red:255.0, blue:148.0, green:112.0,alpha:1.0}),
            18..=26 => color = Color::from(basic::WHITE),
            27..=35 => color = Color::from(basic::RED),
            36..=44 => color = Color::from(basic::GREEN),
            45..=53 => color = Color::from(basic::BLUE),
            _ => return
        }
        let a = commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, y, z).with_scale(Vec3::splat(64.))
        )).insert(Side{ color: Side_Color::Red, num: i as i32}).id();
    }
}
fn get_xyz(i: i32) -> (f32,f32,f32) {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;
    if (i + 1) % 9 == 1 {
        (x,y,z) = (0.0, 0.0, 0.0)}
    if (i + 1) % 9 == 2 {
        (x,y,z) = (1.0, 0.0, 0.0)}
    if (i + 1) % 9 == 3 {
        (x,y,z) = (2.0, 0.0, 0.0)
    }
    if (i + 1) % 9 == 4 {
        (x,y,z) = (0.0, 1.0, 0.0)
    }
    if (i + 1) % 9 == 5 {
        (x,y,z) = (1.0, 1.0, 0.0)
    }
    if (i + 1) % 9 == 6 {
        (x,y,z) = (2.0, 1.0, 0.0)
    }
    if (i + 1) % 9 == 7 {
        (x,y,z) = (0.0, 2.0, 0.0)
    }
    if (i + 1) % 9 == 8 {
        (x,y,z) = (1.0, 2.0, 0.0)
    }
    if (i + 1) % 9 == 0 {   
        (x,y,z) = (2.0, 2.0, 0.0)
    };
    return (x * 64.0, y * 64.0, z * 64.0)
}