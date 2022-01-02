use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

use crate::{game_state::AppState, paddle::{Paddle, PADDLE_SCALE_Y, PADDLE_SCALE_X}, Player};

pub const BALL_INITIAL_X_MIN: f32 = 140.0;
pub const BALL_INITIAL_X_MAX: f32 = 200.0;
pub const BALL_INITIAL_Y_MIN: f32 = -50.0;
pub const BALL_INITIAL_Y_MAX: f32 = 50.0;
pub const BALL_SCALE: f32 = 15.0;
const BOUNCE_VELOCITY_INCREASE: f32 = 1.1;

pub struct BallPlugin;

pub struct Ball {
    pub velocity: Vec2,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system(ball_collision.system())
            .add_system_set(SystemSet::on_update(AppState::Play).with_system(ball_movement.system()));
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Ball
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        sprite: Sprite::new(Vec2::new(BALL_SCALE, BALL_SCALE)),
        ..Default::default()
    })
    .insert( Ball{ velocity: Vec2::new(0.0,0.0) });
}

fn ball_movement(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = query.single_mut() {
        transform.translation.x += ball.velocity.x * time.delta_seconds();
        transform.translation.y += ball.velocity.y * time.delta_seconds();
    }
}

// I needed to use a QuerySet here because I need a mutable reference to the Transform for the Ball, and 
// I also need an immutable reference to the Transform for the paddle. You can't have an immutable and mutable
// reference of the same component at the same time. The problem is You also can't nest these Queries now because
// you can only have one reference to the QuerySet at a time. So I use the first Query to ensure that we have a ball still
// as well as saving the balls translation locally that I need to use in the second Query when I check for collision with the paddles.
// I go throught the second Query and check for collisions. Depending on which player I collided with, I need to move the balls
// x position to the left or right. So I also save that data locally as a boolean. Finally I use the first Query again and this 
// time I actually apply the changes needed to the balls translation and velocity.
fn ball_collision(
    mut q: QuerySet<(
        Query<(&mut Transform, &mut Ball)>,
        Query<(&Paddle, &Transform)>,
    )>,
    windows: Res<Windows>
) {

    let mut ball_translation = Vec3::new(0.0, 0.0, 0.0);
    let mut found_ball = false;

    if let Ok((ball_transform, _)) = q.q0_mut().single_mut() {
        found_ball = true;
        ball_translation = ball_transform.translation;
    }

    // I make sure we found the ball so I don't check collisions against a ball that doesn't exist.
    if found_ball {
        let mut collided_player_1 = false;
        let mut collided_player_2 = false;

        for (paddle, paddle_transform) in q.q1().iter() {
            let collides = collide(
                ball_translation, 
                Vec2::new(BALL_SCALE, BALL_SCALE),
                paddle_transform.translation,
                Vec2::new(PADDLE_SCALE_X, PADDLE_SCALE_Y));
            
            match collides {
                Some(collision) => {
                    match collision {
                        _ => {
                            match paddle.player {
                                Player::Player1 => {
                                    collided_player_1 = true;
                                }
                                Player::Player2 => {
                                    collided_player_2 = true;
                                }
                            }
                        }
                    }
                }
                None => (),
            }
        }

        if let Ok((mut ball_transform, mut ball)) = q.q0_mut().single_mut() {
            let window = windows.get_primary().unwrap();

            if ball_transform.translation.y > window.height() / 2.0 - BALL_SCALE / 2.0 {
                ball.velocity.y = -ball.velocity.y;
            }
            if ball_transform.translation.y < -window.height() / 2.0 + BALL_SCALE / 2.0 {
                ball.velocity.y = -ball.velocity.y;
            }

            if collided_player_1 {
                ball_transform.translation.x += BALL_SCALE / 2.0;
                ball.velocity.x = BOUNCE_VELOCITY_INCREASE * -ball.velocity.x;
            }
            if collided_player_2 {
                ball_transform.translation.x -= BALL_SCALE / 2.0;
                ball.velocity.x = BOUNCE_VELOCITY_INCREASE * -ball.velocity.x;
            }
            if collided_player_1 || collided_player_2 {
                // randomize ball y velocity
                let mut rng = thread_rng();
                let ball_y = rng.gen_range(BALL_INITIAL_Y_MIN..BALL_INITIAL_Y_MAX);
                ball.velocity.y = if ball.velocity.y > 0.0 { ball_y } else { -ball_y };
            }
        }
    }
}
