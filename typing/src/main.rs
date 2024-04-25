use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::*;
#[derive(Component)]
struct TextComponent;
#[derive(Component)]
struct ResultComponent;
#[derive(Component, Debug)]
pub struct TextValue {
    pub value: String,
    pub input: String,
    pub started: bool,
    pub time: f32,
    pub result: i32,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (add_text, check_text, timer.run_if(on_timer(Duration::from_millis(100))))
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
).insert(TextValue { input: String::new(), value: random_text, started: false, time: 0.0, result: 0});
    commands.spawn( 
        (Text2dBundle {
        text: Text::from_section("nothing", text_style.clone()),
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
        ..default()
        }, ResultComponent)
    );
}
fn timer(mut param_set: ParamSet<(Query<(&mut TextValue, &mut Text), With<TextComponent>>,Query<&mut Text, With<ResultComponent>>)>) {
    let mut query = param_set.p0();
    let started = query.get_single().expect("Err getting TextValue").0.started;
    let mut text = query.get_single_mut().expect("Err getting Time").0;
    if started {
        text.time = text.time + 0.1;
        // println!("{}", text.time)
    } else {
        if text.time != 0.0 {
            let v = text.input.clone();
            let result =  v.chars().collect::<Vec<char>>().len() as f32 / text.time;
            text.result = (result * 5.0) as i32;
        }
    }
    if text.result != 0 {
        param_set.p1().get_single_mut().expect("Err").sections[0].value = format!("{}", text.result);
        reset(param_set.p0());
    }
}
fn reset(mut query: Query<(&mut TextValue, &mut Text), With<TextComponent>>) {
    let mut q = query.get_single_mut().expect("err").0;
    let random = get_random_text();
    q.input = String::new();
    q.started = false;
    q.time = 0.0;
    q.result = 0;
    q.value = random.clone();
    query.get_single_mut().expect("Err").1.sections[0].value = random;
}
fn get_random_text() -> String {
    let texts = [
        "Insert random quote here",
        "Another random quote",
        "Random text...",
        "You, 1 seconds ago"
    ];
    let mut rng = rand::thread_rng();
    let random =  rng.gen_range(0..texts.len() -1);
    return texts[random].to_string();
}
fn add_text(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut TextValue ,&mut Transform), With<TextComponent>>) {    
    let mut text = query.get_single_mut().unwrap().0;
    let shift = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let hashmap = create_keycode_hashmap();
    let s = hashmap.iter().map(|(key, s)| {
        if *key == KeyCode::Backspace {
            if keyboard.pressed(*key) {
                text.started = true;
            return *s
            } else {
            return "nothing"
            }
        }
        if keyboard.just_pressed(*key) {
            text.started = true;
            *s
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
            "Stop" => return text.started = false,
            "Backspace" => return {
                let mut arr = text.input.chars().collect::<Vec<char>>();
                arr.pop();
                text.input = arr.into_iter().collect::<String>(); 
            },
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
fn check_text(mut query: Query<&mut TextValue, With<TextComponent>>) {
    let text_value = query.get_single().expect("Couldn't get text value");
    let input = text_value.input.clone();
    let text = text_value.value.clone();
    if input == text {
    println!("Perfect!");
    let mut get_mut = query.get_single_mut().expect("Error");
    get_mut.started = false;
    } else {
        println!("{} {}", text, input)
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
    map.insert(KeyCode::Period, ".");
    map.insert(KeyCode::Backspace, "Backspace");
    map.insert(KeyCode::Space, "Space");
    map.insert(KeyCode::Escape, "Stop");
    map
}