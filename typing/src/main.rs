use bevy::{input::keyboard::{Key, KeyboardInput}, prelude::*};
use rand::*;
#[derive(Component)]
struct TextComponent;
#[derive(Component, Debug)]
pub struct TextValue {
    pub value: String,
    pub input: String
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            check_text,
        )
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_justification = JustifyText::Center;
    commands.spawn(Camera2dBundle::default());
    let random_text = get_random_text();
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(&random_text, text_style.clone())
                .with_justify(text_justification),
            ..default()
        }, TextComponent)
).insert(TextValue { input: String::new(), value: random_text});
}
fn get_random_text() -> String {
    let texts = [
        "Insert random quote here",
        "Another random quote",
    ];
    let mut rng = rand::thread_rng();
    let random =  rng.gen_range(0..texts.len() -1);
    return texts[random].to_string();
}
fn check_text(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut TextValue ,&mut Transform), With<TextComponent>>) {    
    let mut text = query.get_single_mut().unwrap().0;
    let shift = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    println!("{:?}", text);
    if keyboard.just_pressed(KeyCode::KeyA) {
        println!("yay you pressed A!");
        check_shift("a", shift, text.as_mut())
    }
    fn check_shift(text_to_add: &str, shift: bool, text: &mut TextValue) {
        if shift {
            // fix: handle cases like numbers etc...
            text.input += &text_to_add.to_uppercase();
        } else {
            text.input += &text_to_add.to_lowercase();
        }
    }
}