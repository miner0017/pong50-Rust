use bevy::{prelude::*, input::system::exit_on_esc_system};

const SPEED: f32 = 500.0;
const PADDLE_SCALE_X: f32 = 20.0;
const PADDLE_SCALE_Y: f32 = 100.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Play,
    Start,
}

enum Player {
    Player1,
    Player2,
}

struct GameStateText;

struct Paddle {
    player: Player,
}

struct Ball {
    velocity: Vec2,
}

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
        .add_state(AppState::Start)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_system(paddle_movement.system())
        .add_system(change_state_using_enter_key.system())
        .add_system(update_game_state_text.system())
        .add_system_set(SystemSet::on_enter(AppState::Start).with_system(enter_start_state.system()))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>
) {
        // Get the window
        let window = windows.get_primary_mut().unwrap();

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
                "0",
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
        });

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
                "0",
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
        });

        // Left Paddle
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0 - window.width() / 2.0 + 20.0, 0.0, 10.0),
            sprite: Sprite::new(Vec2::new(PADDLE_SCALE_X, PADDLE_SCALE_Y)),
            ..Default::default()
        })
        .insert(Paddle { player: Player::Player1 });

        // Right Paddle
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0 + window.width() / 2.0 - 20.0, 0.0, 10.0),
            sprite: Sprite::new(Vec2::new(PADDLE_SCALE_X, PADDLE_SCALE_Y)),
            ..Default::default()
        })
        .insert(Paddle { player: Player::Player2 });

        // Ball
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        })
        .insert( Ball{ velocity: Vec2::new(0.0, 0.0) });
}

fn paddle_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut query: Query<(&Paddle, &mut Transform)>
) {
    let window = windows.get_primary().unwrap();

    for (paddle, mut transform) in query.iter_mut() {

        // Get Paddles movement direction based on key pressed.
        let mut direction: f32 = 0.0;

        match paddle.player {
            Player::Player1 => {
                if input.pressed(KeyCode::W) {
                    direction = 1.0
                } else if input.pressed(KeyCode::S) {
                    direction = -1.0
                }
            }
            Player::Player2 => {
                if input.pressed(KeyCode::Up) {
                    direction = 1.0
                } else if input.pressed(KeyCode::Down) {
                    direction = -1.0
                }
            }
        }
        
        // Clamp our Paddles within the top and bottom of the screen
        let mut y_translation = transform.translation.y + direction * SPEED * time.delta_seconds();
        let max_height = window.height() / 2.0 - PADDLE_SCALE_Y / 2.0;
        let min_height = -window.height() / 2.0 + PADDLE_SCALE_Y / 2.0;

        if y_translation > max_height {
            y_translation = max_height;
        } else if y_translation < min_height {
            y_translation = min_height;
        }

        // Apply our transformation
        transform.translation.y = y_translation;

    }
}

fn enter_start_state(mut query: Query<&mut Transform, With<Ball>>) { 
    if let Ok(mut transform) = query.single_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
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
