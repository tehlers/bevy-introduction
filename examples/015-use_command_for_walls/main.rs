use bevy::{prelude::*, render::camera::ScalingMode};

const MAX_X: f32 = 1920.0;
const MAX_Y: f32 = 1200.0;
const WALL_THICKNESS: f32 = 20.0;
const BALL_SPEED: f32 = 400.0;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

// example-start: 1 {0|1-6|8,24|9,16|10-15|18-22|all}
enum WallLocation {
    Top,
    Bottom,
    Left,
    Right,
}

impl WallLocation {
    fn position(&self) -> Vec3 {
        match self {
            WallLocation::Top => Vec3::new(0.0, MAX_Y / 2.0, 0.0),
            WallLocation::Bottom => Vec3::new(0.0, -MAX_Y / 2.0, 0.0),
            WallLocation::Left => Vec3::new(-MAX_X / 2.0, 0.0, 0.0),
            WallLocation::Right => Vec3::new(MAX_X / 2.0, 0.0, 0.0),
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            WallLocation::Bottom | WallLocation::Top => Vec3::new(MAX_X, WALL_THICKNESS, 0.0),
            WallLocation::Left | WallLocation::Right => Vec3::new(WALL_THICKNESS, MAX_Y, 0.0),
        }
    }
}
// example-end: 1

// example-start: 2 {0|1,3|2|5,12|6,11|7-10|all}
struct SpawnWall {
    location: WallLocation,
}

impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        world.spawn((
            Sprite::from_color(Color::WHITE, Vec2::ONE),
            Transform::from_translation(self.location.position()).with_scale(self.location.size()),
        ));
    }
}
// example-end: 2

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

    // example-start: 3 {0|1-3|1-6|1-9|all}
    commands.queue(SpawnWall {
        location: WallLocation::Top,
    });
    commands.queue(SpawnWall {
        location: WallLocation::Bottom,
    });
    commands.queue(SpawnWall {
        location: WallLocation::Left,
    });
    commands.queue(SpawnWall {
        location: WallLocation::Right,
    });
    // example-end: 3

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
