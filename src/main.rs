use bevy::{prelude::*, input::system::exit_on_esc_system, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};
use rand::{thread_rng, Rng};

mod paddle;
mod ball;
mod score;

use paddle::PaddlePlugin;
use score::ScorePlugin;
use ball::{BallPlugin, Ball, BALL_INITIAL_X_MIN, BALL_INITIAL_X_MAX, BALL_INITIAL_Y_MAX, BALL_INITIAL_Y_MIN};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Play,
    Start,
}

pub enum Player {
    Player1,
    Player2,
}

struct GameStateText;
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
        .add_plugins(DefaultPlugins)
        .add_plugin(PaddlePlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_state(AppState::Start)
        .add_startup_system(setup.system())
        .add_system(update_fps_text.system())
        .add_system(exit_on_esc_system.system())
        .add_system(change_state_using_enter_key.system())
        .add_system(update_game_state_text.system())
        .add_system_set(SystemSet::on_enter(AppState::Start).with_system(enter_start_state.system()))
        .run();
}

fn setup(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>
) {
        // Get the window
        let window = windows.get_primary().unwrap();

        // 2D camera
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        // UI camera
        commands.spawn_bundle(UiCameraBundle::default());

        // 2D UI Text
        // Shows our current Game State
        commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(window.width() / 2.0 - 80.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Pong, Start!",
                TextStyle {
                    font: asset_server.load("fonts/font.ttf"),
                    font_size: 25.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(GameStateText);


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
}

fn enter_start_state(mut query: Query<(&mut Transform, &mut Ball)>) {
    if let Ok((mut transform, mut ball)) = query.single_mut() {
        // reset ball position to 0.0
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;

        // randomize ball velocity
        let mut rng = thread_rng();
        let ball_x = rng.gen_range(BALL_INITIAL_X_MIN..BALL_INITIAL_X_MAX);
        let ball_y = rng.gen_range(BALL_INITIAL_Y_MIN..BALL_INITIAL_Y_MAX);

        ball.velocity.x = ball_x;
        ball.velocity.y = ball_y;
    }


}

fn change_state_using_enter_key(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>
) {
    if keys.just_pressed(KeyCode::Return) {

        match app_state.current() {
            AppState::Play => app_state.set(AppState::Start).unwrap(),
            AppState::Start => app_state.set(AppState::Play).unwrap(),
        }
    }
}

fn update_game_state_text(app_state: Res<State<AppState>>, mut query: Query<&mut Text, With<GameStateText>>) {
    if let Ok(mut text) = query.single_mut() {
        text.sections[0].value = format!("Pong, {:?}!", app_state.current());
    }
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
