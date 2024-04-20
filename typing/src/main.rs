use bevy::prelude::*;

#[derive(Component)]
struct TextComponent;
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
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("translation", text_style.clone())
                .with_justify(text_justification),
            ..default()
        }, TextComponent)
);
}
fn check_text(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>,) {

}