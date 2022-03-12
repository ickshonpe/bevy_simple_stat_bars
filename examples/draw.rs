use bevy::math::vec2;
use bevy::prelude::*;
use bevy_status_bars_2d::prelude::*;


fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let bar = StatusBar {
        value: 0.8,
        z_depth: 900.0,
        thickness: 30.0,
        full_length: 300.0,
        full_color: Color::WHITE,
        empty_color: Color::BLACK,
        alignment: vec2(0.5, 0.5),
        border: Some(StatusBarBorder {
            color: Color::DARK_GRAY,
            thickness: 5.0,
        }),
        orientation: StatusBarOrientation::Horizontal,
        ..Default::default()
    };
    let transform = Transform::default();

    let mut ec = commands.spawn();
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(
        setup
    )
    .run();
}