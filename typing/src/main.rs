use std::collections::HashMap;

use bevy::{input::keyboard::{Key, KeyboardInput}, prelude::*, utils::hashbrown::hash_map};
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
    // println!("{:?}", text);
    // if keyboard.just_pressed(KeyCode::KeyA) {
        
    //     check_shift("a", shift, text.as_mut())
    // }
    let hashmap = create_keycode_hashmap();
    let s = hashmap.iter().map(|(key, s)| {
        if keyboard.just_pressed(*key) {
            println!("yay you pressed {}!", s);
            s
        } else {
            "nothing"
        }
    }).collect::<Vec<&str>>();
    // remove "nothing" from Vec
    let string = s.into_iter().filter(|s| *s != "nothing").collect::<Vec<&str>>();
    if !string.is_empty() {
        check_shift(string[0], shift, text.as_mut());
    }
    
    fn check_shift(text_to_add: &str, shift: bool, text: &mut TextValue) {
        match text_to_add {
            "Backspace" => return text.input = text.input.chars().collect::<Vec<char>>().pop().into_iter().collect::<String>(),
            "Space" => return text.input += " ",
            _ => {
            }
        }
        if shift {
            // fix: handle cases like numbers etc...
            match text_to_add {
            "1" => text.input += "!",
            _ => text.input += &text_to_add.to_uppercase()
            }
        } else {
            text.input += &text_to_add.to_lowercase();
        }
        println!("{}", text.input)
    }
}
fn create_keycode_hashmap() -> HashMap<KeyCode, &'static str> {
let mut map = HashMap::default();
    map.insert(KeyCode::KeyA, "A");
    map.insert(KeyCode::KeyB, "B");
    map.insert(KeyCode::KeyC, "C");
    map.insert(KeyCode::KeyD, "D");
    map.insert(KeyCode::KeyE, "E");
    map.insert(KeyCode::KeyF, "F");
    map.insert(KeyCode::KeyG, "G");
    map.insert(KeyCode::KeyH, "H");
    map.insert(KeyCode::KeyI, "I");
    map.insert(KeyCode::KeyJ, "J");
    map.insert(KeyCode::KeyK, "K");
    map.insert(KeyCode::KeyL, "L");
    map.insert(KeyCode::KeyM, "M");
    map.insert(KeyCode::KeyN, "N");
    map.insert(KeyCode::KeyO, "O");
    map.insert(KeyCode::KeyP, "P");
    map.insert(KeyCode::KeyQ, "Q");
    map.insert(KeyCode::KeyR, "R");
    map.insert(KeyCode::KeyS, "S");
    map.insert(KeyCode::KeyT, "T");
    map.insert(KeyCode::KeyU, "U");
    map.insert(KeyCode::KeyV, "V");
    map.insert(KeyCode::KeyW, "W");
    map.insert(KeyCode::KeyX, "X");
    map.insert(KeyCode::KeyY, "Y");
    map.insert(KeyCode::KeyZ, "X");
    map.insert(KeyCode::Digit1, "1");
    map.insert(KeyCode::Digit2, "2");
    map.insert(KeyCode::Digit3, "3");
    map.insert(KeyCode::Digit4, "4");
    map.insert(KeyCode::Digit5, "5");
    map.insert(KeyCode::Digit6, "6");
    map.insert(KeyCode::Digit7, "7");
    map.insert(KeyCode::Digit8, "8");
    map.insert(KeyCode::Digit9, "9");
    map.insert(KeyCode::Backspace, "Backspace");
    map.insert(KeyCode::Space, "Space");
    map
}