use bevy::{prelude::*, input::system::exit_on_esc_system};

const SPEED: f32 = 500.0; 

enum Player {
    Player1,
    Player2,
}

struct Paddle {
    player: Player,
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
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_system(paddle_movement.system())
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
        // Text with one section
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
                    "Hello, Pong!",
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
            });

        // Left Paddle
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0 - window.width() / 2.0 + 20.0, 0.0, 10.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        })
        .insert(Paddle { player: Player::Player1 });

        // Right Paddle
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0 + window.width() / 2.0 - 20.0, 0.0, 10.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        })
        .insert(Paddle { player: Player::Player2 });

        // Ball
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        });
}

fn paddle_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Paddle, &mut Transform)>
) {
    for (paddle, mut transform) in query.iter_mut() {

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

        transform.translation.y += direction * SPEED * time.delta_seconds();

    }
}
