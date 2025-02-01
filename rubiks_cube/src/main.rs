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
        match i {
            0..=8 => {
                let a = commands.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(Color::from(basic::YELLOW))),
                    Transform::from_xyz(
                        x*64.0,y*64.0,z*64.0
                    ).with_scale(Vec3::splat(64.))
                )).insert(Side{ color: Side_Color::Red, num: i as i32}).id();
            },
            9..=17 => {
                let a = commands.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(Color::from(basic::GREEN))),
                    Transform::from_xyz(
                        0.0, 0.0, 0.0
                    ).with_scale(Vec3::splat(64.))
                )).insert(Side{ color: Side_Color::Green, num: i as i32}).id();
            },
            18..=26 => {
                let a = commands.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(Color::from(basic::YELLOW))),
                    Transform::from_xyz(
                        0.0, 0.0, 0.0
                    ).with_scale(Vec3::splat(64.))
                )).insert(Side{ color: Side_Color::Yellow, num: i as i32}).id();
            },
            27..=35 => {
                let a = commands.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(Color::Srgba(Srgba {red:255.0, blue:148.0, green:112.0,alpha:1.0}))),
                    Transform::from_xyz(
                        0.0, 0.0, 0.0
                    ).with_scale(Vec3::splat(64.))
                )).insert(Side{ color: Side_Color::Orange, num: i as i32}).id();
            },
            36..=44 => {
                let a = commands.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(Color::from(basic::WHITE))),
                    Transform::from_xyz(
                        0.0, 0.0, 0.0
                    ).with_scale(Vec3::splat(64.))
                )).insert(Side{ color: Side_Color::White, num: i as i32}).id();
        }
        45..=53 => {
            let a = commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(basic::WHITE))),
                Transform::from_xyz(
                    0.0, 0.0, 0.0
                ).with_scale(Vec3::splat(64.))
            )).insert(Side{ color: Side_Color::White, num: i as i32}).id();
        }
        _ => return
    }}
}
