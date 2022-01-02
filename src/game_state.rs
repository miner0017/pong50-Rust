use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{ball::{Ball, BALL_INITIAL_X_MAX, BALL_INITIAL_X_MIN, BALL_INITIAL_Y_MIN, BALL_INITIAL_Y_MAX}, Server, Player, score::Scoreboard};

pub struct GameStatePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Play,
    Start,
    Serve,
}

struct GameStateText;

impl Plugin for GameStatePlugin{
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_state(AppState::Start)
            .add_system(change_state_using_enter_key.system())
            .add_system(update_game_state_text.system())
            .add_system_set(SystemSet::on_enter(AppState::Start).with_system(enter_start_state.system()))
            .add_system_set(SystemSet::on_enter(AppState::Serve).with_system(enter_serve_state.system()));
    }
}

fn setup(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>
) {
    let window = windows.get_primary().unwrap();

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
}

fn enter_start_state(
    mut query: Query<(&mut Transform, &mut Ball)>,
    mut scoreboard: ResMut<Scoreboard>,
    server: Res<Server>
) {
    if let Ok((mut transform, mut ball)) = query.single_mut() {
        // reset scores to 0
        scoreboard.player1 = 0;
        scoreboard.player2 = 0;

        // reset ball position to 0.0
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;


        let mut serve = 1.0;

        match server.0 {
            Player::Player2 => serve = -1.0,
            _ => (),
        }


        // randomize ball velocity with relation to server
        let mut rng = thread_rng();
        let ball_x = serve * rng.gen_range(BALL_INITIAL_X_MIN..BALL_INITIAL_X_MAX);
        let ball_y = rng.gen_range(BALL_INITIAL_Y_MIN..BALL_INITIAL_Y_MAX);

        ball.velocity.x = ball_x;
        ball.velocity.y = ball_y;
    }


}

fn enter_serve_state(
    mut query: Query<(&mut Transform, &mut Ball)>,
    server: Res<Server>    
) {
    if let Ok((mut transform, mut ball)) = query.single_mut() {
        // reset ball position to 0.0
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;

        let mut serve = 1.0;

        match server.0 {
            Player::Player2 => serve = -1.0,
            _ => (),
        }


        // randomize ball velocity with relation to server
        let mut rng = thread_rng();
        let ball_x = serve * rng.gen_range(BALL_INITIAL_X_MIN..BALL_INITIAL_X_MAX);
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
            AppState::Play => (),
            AppState::Start => app_state.set(AppState::Serve).unwrap(),
            AppState::Serve => app_state.set(AppState::Play).unwrap()
        }
    }
}

fn update_game_state_text(app_state: Res<State<AppState>>, mut query: Query<&mut Text, With<GameStateText>>) {
    if let Ok(mut text) = query.single_mut() {
        text.sections[0].value = format!("Pong, {:?}!", app_state.current());
    }
}
