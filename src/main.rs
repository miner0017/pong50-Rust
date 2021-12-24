use bevy::{prelude::*, input::system::exit_on_esc_system};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // 2d camera
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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
}
