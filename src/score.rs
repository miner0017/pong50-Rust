use bevy::prelude::*;

use crate::{Player, AppState, ball::Ball};
pub struct ScorePlugin;

struct Scoreboard {
    player1: u32,
    player2: u32,
}

struct ScoreText(Player);

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(Scoreboard {
                player1: 0,
                player2: 0,
            })
            .add_startup_system(setup.system())
            .add_system(scored.system())
            .add_system(update_scoreboard.system());
    }
}

fn setup(
    mut commands: Commands,
    windows: Res<Windows>,
    scoreboard: Res<Scoreboard>,
    asset_server: Res<AssetServer>
) {
    let window = windows.get_primary().unwrap();
    
    // Player1 Score Text
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(window.height() / 2.0 - 40.0),
                left: Val::Px(window.width() / 2.0 - 80.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            scoreboard.player1.to_string(),
            TextStyle {
                font: asset_server.load("fonts/font.ttf"),
                font_size: 80.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    }).insert(ScoreText(Player::Player1));

    // Player2 Score Text
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(window.height() / 2.0 - 40.0),
                left: Val::Px(window.width() / 2.0 + 40.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            scoreboard.player2.to_string(),
            TextStyle {
                font: asset_server.load("fonts/font.ttf"),
                font_size: 80.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    }).insert(ScoreText(Player::Player2));
}

fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<(&mut Text, &ScoreText)>,
) {
    for (mut text, scoretext) in query.iter_mut() {
        match scoretext.0 {
            Player::Player1 => {
                text.sections[0].value = scoreboard.player1.to_string();
            }
            Player::Player2 => {
                text.sections[0].value = scoreboard.player2.to_string();
            }
        }
    }
}

fn scored(
    mut scoreboard: ResMut<Scoreboard>,
    mut app_state: ResMut<State<AppState>>,
    query: Query<&Transform, With<Ball>>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();

    for transform in query.iter() {
        if transform.translation.x > window.width() / 2.0 {
            scoreboard.player1 += 1;
            app_state.set(AppState::Start).unwrap();
        } else if transform.translation.x < -window.width() / 2.0 {
            scoreboard.player2 += 1;
            app_state.set(AppState::Start).unwrap();
        }
    }
}
