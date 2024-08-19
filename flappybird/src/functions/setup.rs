use bevy::prelude::*;
use super::*;
pub fn setup(
    mut commands: Commands,
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
