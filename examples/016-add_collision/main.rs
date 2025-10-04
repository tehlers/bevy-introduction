// example-start: 5 {0|2}
use bevy::{
    camera::ScalingMode,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};
// example-end: 5

const MAX_X: f32 = 1920.0;
const MAX_Y: f32 = 1200.0;
const WALL_THICKNESS: f32 = 20.0;
// example-start: 3 {0|all}
const BALL_RADIUS: f32 = 12.0;
// example-end: 3
const BALL_SPEED: f32 = 600.0;

// example-start: 1 {0|all}
#[derive(Component)]
struct Collider;
// example-end: 1

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

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

struct SpawnWall {
    location: WallLocation,
}

// example-start: 2 {0|6}
impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        world.spawn((
            Sprite::from_color(Color::WHITE, Vec2::ONE),
            Transform::from_translation(self.location.position()).with_scale(self.location.size()),
            Collider,
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

// example-start: 4 {0|1,4|2|2,5|3|3,6|7-13|all}
fn check_for_collisions(
    mut balls: Query<(&mut Ball, &Transform)>,
    obstacles: Query<&Transform, With<Collider>>,
) {
    for (mut ball, ball_transform) in &mut balls {
        for obstacle in &obstacles {
            let collision = ball_collision(
                BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS),
                Aabb2d::new(
                    obstacle.translation.truncate(),
                    obstacle.scale.truncate() / 2.,
                ),
            );
            // ...
            // example-end: 4

            // example-start: 7 {0|1,19|2-3|5-10|12-18|all}
            if let Some(collision) = collision {
                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision {
                    Collision::Left => reflect_x = ball.velocity.x > 0.0,
                    Collision::Right => reflect_x = ball.velocity.x < 0.0,
                    Collision::Top => reflect_y = ball.velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball.velocity.y > 0.0,
                }

                if reflect_x {
                    ball.velocity.x = -ball.velocity.x;
                }

                if reflect_y {
                    ball.velocity.y = -ball.velocity.y;
                }
            }
            // example-end: 7
        }
    }
}

// example-start: 6 {0|1-6|8,28|9-11|13-27|all}
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
// example-end: 6

fn main() {
    let mut app = App::new();

    // example-start: 8 {0|3}
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_velocity, check_for_collisions))
        .run();
    // example-end: 8
}
