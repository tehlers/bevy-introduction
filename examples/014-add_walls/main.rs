use bevy::{prelude::*, render::camera::ScalingMode};

const MAX_X: f32 = 1920.0;
const MAX_Y: f32 = 1200.0;
const WALL_THICKNESS: f32 = 20.0;
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
                min_width: MAX_X,
                min_height: MAX_Y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Top wall
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform::from_xyz(0.0, MAX_Y / 2.0, 0.0).with_scale(Vec3::new(
            MAX_X,
            WALL_THICKNESS,
            0.0,
        )),
    ));

    // Bottom wall
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform::from_xyz(0.0, -MAX_Y / 2.0, 0.0).with_scale(Vec3::new(
            MAX_X,
            WALL_THICKNESS,
            0.0,
        )),
    ));

    // Left wall
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform::from_xyz(-MAX_X / 2.0, 0.0, 0.0).with_scale(Vec3::new(
            WALL_THICKNESS,
            MAX_Y,
            0.0,
        )),
    ));

    // Right wall
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform::from_xyz(MAX_X / 2.0, 0.0, 0.0).with_scale(Vec3::new(
            WALL_THICKNESS,
            MAX_Y,
            0.0,
        )),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball.png")),
        Ball {
            velocity: Vec2::new(0.5, 0.5).normalize() * BALL_SPEED,
        },
    ));
}

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
