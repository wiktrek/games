use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use std::time::Duration;
mod functions;
use functions::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default()))
        .add_systems(Startup, (setup, spawn_bird))
        .add_systems(Update, (bird_gravity, update_score, move_pipes, jump, spawn_pipes.run_if(on_timer(Duration::from_secs(3))), get_window.run_if(on_timer(Duration::from_millis(100))),))
        .run();
}
fn update_score(bird: Query<&mut Score, With<Bird>>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    for mut text in score_text.iter_mut() {
        let score = bird.get_single().unwrap().value;
        text.sections[1].value = format!("{}", score);
    }
}
