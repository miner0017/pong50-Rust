use bevy::prelude::*;

use crate::AppState;

pub const BALL_INITIAL_X_MIN: f32 = 140.0;
pub const BALL_INITIAL_X_MAX: f32 = 200.0;
pub const BALL_INITIAL_Y_MIN: f32 = -50.0;
pub const BALL_INITIAL_Y_MAX: f32 = 50.0;

pub struct BallPlugin;

pub struct Ball {
    pub velocity: Vec2,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
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
        sprite: Sprite::new(Vec2::new(15.0, 15.0)),
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