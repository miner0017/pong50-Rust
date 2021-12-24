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
        // UI camera
        commands.spawn_bundle(UiCameraBundle::default());
        // Text with one section
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Relative,
                    position: Rect {
                        top: Val::Px(0.0),
                        right: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Hello, Pong!",
                    TextStyle {
                        font: asset_server.load("fonts/font.ttf"),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
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
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        });
}
