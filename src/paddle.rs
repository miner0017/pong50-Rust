use bevy::prelude::*;

use crate::{Player, ball::{Ball, BALL_SCALE}};

const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_SCALE_X: f32 = 20.0;
pub const PADDLE_SCALE_Y: f32 = 100.0;


pub struct PaddlePlugin;

pub struct Paddle {
    pub player: Player,
}

struct PaddleAI;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system(paddle_movement.system())
            .add_system(paddle_ai_movement.system());
    }
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>
) {

    // Get the window
    let window = windows.get_primary_mut().unwrap();

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
    .insert(Paddle { player: Player::Player2 })
    .insert(PaddleAI);
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
            Player::Player2 => ()
            // Commented out so I can add AI for Player2
            //{
            //    if input.pressed(KeyCode::Up) {
            //        direction = 1.0
            //    } else if input.pressed(KeyCode::Down) {
            //        direction = -1.0
            //    }
            //}
        }
        
        // Calculate paddle movement
        let mut y_translation = transform.translation.y + direction * PADDLE_SPEED * time.delta_seconds();

        // Clamp our Paddles within the top and bottom of the screen
        let max_height = window.height() / 2.0 - PADDLE_SCALE_Y / 2.0;
        let min_height = -window.height() / 2.0 + PADDLE_SCALE_Y / 2.0;

        if y_translation > max_height {
            y_translation = max_height;
        } else if y_translation < min_height {
            y_translation = min_height;
        }

        // Apply our paddles movement
        transform.translation.y = y_translation;

    }
}

// I need an immutable reference for the balls Transform as well as a mutable reference
// to the paddle Transform. Going to have to do that QuerySet thing again.
fn paddle_ai_movement(
    mut query: QuerySet<(
        Query<&Transform, With<Ball>>,
        Query<&mut Transform, With<PaddleAI>>
    )>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    let mut ball_y = 0.0;
    if let Ok(ball_transform) = query.q0().single() {
        ball_y = ball_transform.translation.y;
    }

    if let Ok(mut paddle_transform) = query.q1_mut().single_mut() {
        let paddle_y = paddle_transform.translation.y;

        let mut direction: f32 = 0.0;
        if paddle_y + PADDLE_SCALE_Y / 4.0 < ball_y {
            direction = 1.0;
        } else if paddle_y - PADDLE_SCALE_Y / 4.0 > ball_y {
            direction = -1.0;
        }

        let window = windows.get_primary().unwrap();

        // Calculate paddle movement
        let mut y_translation = paddle_transform.translation.y + direction * PADDLE_SPEED * time.delta_seconds();
        // Clamp our Paddles within the top and bottom of the screen
        let max_height = window.height() / 2.0 - PADDLE_SCALE_Y / 2.0;
        let min_height = -window.height() / 2.0 + PADDLE_SCALE_Y / 2.0;

        if y_translation > max_height {
            y_translation = max_height;
        } else if y_translation < min_height {
            y_translation = min_height;
        }

        // Apply our paddles movement
        paddle_transform.translation.y = y_translation;
    }
}
