use bevy::{prelude::*, render::camera::ScalingMode};

// example-start: 1 {0|1|1,3-6|1,3-6,22-24}
const BALL_SPEED: f32 = 400.0;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

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

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball.png")),
        Ball {
            velocity: Vec2::new(0.5, 0.5).normalize() * BALL_SPEED,
        },
    ));
}
// example-end: 1

// example-start: 2 {0|1,6|2,5|3-4|1-6,13}
fn apply_velocity(mut balls: Query<(&Ball, &mut Transform)>, time: Res<Time>) {
    for (ball, mut transform) in &mut balls {
        transform.translation.x += ball.velocity.x * time.delta_secs();
        transform.translation.y += ball.velocity.y * time.delta_secs();
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, apply_velocity)
        .run();
}
// example-end: 2
