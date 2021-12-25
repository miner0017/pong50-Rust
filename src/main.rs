use bevy::{prelude::*, input::system::exit_on_esc_system};

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

        // 2D Text 
        // UI camera
        commands.spawn_bundle(UiCameraBundle::default());
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
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0 - window.width() / 2.0 + 20.0, 0.0, 10.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        });

        // Right Paddle
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0 + window.width() / 2.0 - 20.0, 0.0, 10.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        });

        // Ball
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(15.0, 15.0)),
            ..Default::default()
        });
}
