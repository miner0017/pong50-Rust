use bevy::{prelude::*, input::system::exit_on_esc_system, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};

mod paddle;
mod ball;
mod score;
mod game_state;

use paddle::PaddlePlugin;
use game_state::GameStatePlugin;
use score::ScorePlugin;
use ball::BallPlugin;

pub enum Player {
    Player1,
    Player2,
}

struct Server(Player);

pub struct LoadedAudio {
    paddle_hit: Handle<AudioSource>,
    score: Handle<AudioSource>,
    wall_hit: Handle<AudioSource>
}

struct FPSText;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Pong50".to_string(),
            width: 1080.0,
            height: 720.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(Server(Player::Player1))
        .add_plugins(DefaultPlugins)
        .add_plugin(PaddlePlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup.system())
        .add_system(update_fps_text.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // 2D UI Text
    // FPS Text
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/font.ttf"),
                        font_size: 15.0,
                        color: Color::ORANGE_RED,
                    },
                    
                },
                TextSection {
                    value: " fps".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/font.ttf"),
                        font_size: 15.0,
                        color: Color::YELLOW,
                    },
                    
                },
            ],
            alignment: Default::default(),
        },
        ..Default::default()
    }).insert(FPSText);

    commands.insert_resource( LoadedAudio{
        paddle_hit: asset_server.load("sounds/paddle_hit.wav"),
        score: asset_server.load("sounds/score.wave"),
        wall_hit: asset_server.load("sounds/wall_hit.wav"),
    })
}

fn update_fps_text(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FPSText>>
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        text.sections[0].value = format!("{:.1}", fps);
    }
}
