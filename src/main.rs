use bevy::{prelude::*, input::system::exit_on_esc_system};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>) {
        // 2D camera
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        // 2D Text 
        commands.spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Hello, World!",
                TextStyle {
                    font: asset_server.load("fonts/font.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });

        // Left Paddle

        // Right Paddle

        // Ball
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        });
}
