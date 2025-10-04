use bevy::{camera::ScalingMode, prelude::*};

// example-start: 1 {0|4-10}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1920.0,
                min_height: 1200.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn(Sprite::from_image(asset_server.load("sprites/ball.png")));
}
// example-end: 1

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
