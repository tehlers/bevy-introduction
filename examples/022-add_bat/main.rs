// example-start: 5 {0|2}
// example-start: 9 {0|6}
use bevy::{
    input::mouse::MouseMotion,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    render::camera::ScalingMode,
    window::PrimaryWindow,
};
// example-end: 5
// example-end: 9

const MAX_X: f32 = 1920.0;
const MAX_Y: f32 = 1200.0;
const WALL_THICKNESS: f32 = 20.0;
const BALL_RADIUS: f32 = 12.0;
const BALL_SPEED: f32 = 600.0;
const MARGIN: f32 = 12.0;
const STONE_SIZE: Vec2 = Vec2::new(82.0, 28.0);
// example-start: 2 {0|all}
// example-start: 6 {0|2-3}
const BAT_SIZE: Vec2 = Vec2::new(124.0, 28.0);
// example-end: 2
const BAT_LEFT_BORDER: f32 = -(MAX_X / 2.0) + WALL_THICKNESS + BAT_SIZE.x / 2.0;
const BAT_RIGHT_BORDER: f32 = -BAT_LEFT_BORDER;
// example-end: 6

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

// example-start: 1 {1-2|6|1-2,6}
#[derive(Component)]
struct Bat;

#[derive(Clone, Copy)]
enum Obstacle {
    Bat,
    Stone,
    Wall,
}
// example-end: 1

#[derive(Component)]
struct Collider {
    size: Option<Vec2>,
    obstacle: Obstacle,
}

#[derive(Component)]
struct Stone;

#[derive(Event)]
struct CollisionEvent {
    obstacle: Obstacle,
}

#[derive(Component)]
struct Despawning(Timer);

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

impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        world.spawn((
            Sprite::from_color(Color::WHITE, Vec2::ONE),
            Transform::from_translation(self.location.position()).with_scale(self.location.size()),
            Collider {
                size: None,
                obstacle: Obstacle::Wall,
            },
        ));
    }
}

struct SpawnStone {
    x: f32,
    y: f32,
}

impl Command for SpawnStone {
    fn apply(self, world: &mut World) {
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(STONE_SIZE.x as u32, STONE_SIZE.y as u32), /*tile_size*/
            10,                                                   /*columns*/
            1,                                                    /*rows*/
            None,                                                 /*padding*/
            None,                                                 /*offset*/
        );
        let texture_atlas_layouts = world.get_resource_mut::<Assets<TextureAtlasLayout>>();
        let texture_atlas_layout = texture_atlas_layouts.unwrap().add(layout);

        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            world.spawn((
                Sprite::from_atlas_image(
                    asset_server.load("sprites/stone-animated.png"),
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: 0,
                    },
                ),
                Transform::from_xyz(self.x, self.y, 0.0),
                Collider {
                    size: Some(STONE_SIZE),
                    obstacle: Obstacle::Stone,
                },
                Stone,
            ));
        }
    }
}

// example-start: 10 {0|4|6-7|4,6-7}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    primary_window.cursor_options.visible = false;
    // example-end: 10

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
        Transform::from_xyz(
            0.0,
            -MAX_Y / 2.0 + WALL_THICKNESS + MARGIN + BALL_RADIUS * 2.0,
            0.0,
        ),
        Ball {
            velocity: Vec2::new(0.5, 0.5).normalize() * BALL_SPEED,
        },
    ));

    // example-start: 3 {0|2|3|4-6|8|all}
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/bat.png")),
        Transform::from_xyz(0.0, -MAX_Y / 2.0 + WALL_THICKNESS + MARGIN, 0.0),
        Collider {
            size: Some(BAT_SIZE),
            obstacle: Obstacle::Bat,
        },
        Bat,
    ));
    // example-end: 3

    for x in (((-MAX_X / 2.0 + WALL_THICKNESS / 2.0 + MARGIN + STONE_SIZE.x / 2.0 + 3.0) as i32)
        ..(MAX_X / 2.0) as i32)
        .step_by((STONE_SIZE.x + MARGIN) as usize)
    {
        for y in (0..((MAX_Y / 2.0 - WALL_THICKNESS / 2.0 - MARGIN - STONE_SIZE.y / 2.0) as i32))
            .step_by((STONE_SIZE.y + MARGIN) as usize)
        {
            commands.queue(SpawnStone {
                x: x as f32,
                y: y as f32,
            });
        }
    }
}

fn apply_velocity(mut balls: Query<(&Ball, &mut Transform)>, time: Res<Time>) {
    for (ball, mut transform) in &mut balls {
        transform.translation.x += ball.velocity.x * time.delta_secs();
        transform.translation.y += ball.velocity.y * time.delta_secs();
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut balls: Query<(&mut Ball, &Transform)>,
    obstacles: Query<(Entity, &Transform, &Collider, Option<&Stone>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (mut ball, ball_transform) in &mut balls {
        for (entity, obstacle, collider, maybe_stone) in &obstacles {
            let collision = ball_collision(
                BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS),
                Aabb2d::new(
                    obstacle.translation.truncate(),
                    collider.size.unwrap_or(obstacle.scale.truncate()) / 2.,
                ),
            );

            if let Some(collision) = collision {
                collision_events.send(CollisionEvent {
                    obstacle: collider.obstacle,
                });

                if maybe_stone.is_some() {
                    commands
                        .entity(entity)
                        .insert(Despawning(Timer::from_seconds(0.01, TimerMode::Repeating)));
                }

                // Reflect the ball's velocity when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // Reflect only if the velocity is in the opposite direction of the collision
                // This prevents the ball from getting stuck inside the bar
                match collision {
                    Collision::Left => reflect_x = ball.velocity.x > 0.0,
                    Collision::Right => reflect_x = ball.velocity.x < 0.0,
                    Collision::Top => reflect_y = ball.velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball.velocity.y > 0.0,
                }

                // Reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball.velocity.x = -ball.velocity.x;
                }

                // Reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball.velocity.y = -ball.velocity.y;
                }
            }
        }
    }
}

// example-start: 7 {0|1|2|3-6|all}
fn move_bat(mut motion: EventReader<MouseMotion>, mut bat_query: Query<&mut Transform, With<Bat>>) {
    for event in motion.read() {
        for mut bat in &mut bat_query {
            bat.translation.x += event.delta.x * 2.0;
            bat.translation.x = bat.translation.x.clamp(BAT_LEFT_BORDER, BAT_RIGHT_BORDER);
        }
    }
}
// example-end: 7

enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Returns `Some` if `ball` collides with `bounding_box`.
// The returned `Collision` is the side of `bounding_box` that `ball` hit.
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

fn despawn_stones(
    mut commands: Commands,
    time: Res<Time>,
    mut stones: Query<(Entity, &mut Sprite, &mut Despawning)>,
) {
    for (entity, mut sprite, mut despawning) in &mut stones {
        despawning.0.tick(time.delta());
        if despawning.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index < 9 {
                    atlas.index += 1;
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn play_sounds(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in collision_events.read() {
        match event.obstacle {
            // example-start: 4
            Obstacle::Bat => commands.spawn((
                AudioPlayer::new(asset_server.load("sounds/bat.ogg")),
                PlaybackSettings::DESPAWN,
            )),
            // example-end: 4
            Obstacle::Stone => commands.spawn((
                AudioPlayer::new(asset_server.load("sounds/stone.ogg")),
                PlaybackSettings::DESPAWN,
            )),
            Obstacle::Wall => commands.spawn((
                AudioPlayer::new(asset_server.load("sounds/wall.ogg")),
                PlaybackSettings::DESPAWN,
            )),
        };
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // example-start: 8 {0|7}
        .add_systems(
            Update,
            (
                apply_velocity,
                check_for_collisions,
                despawn_stones,
                move_bat,
                play_sounds,
            ),
        )
        // example-end: 8
        .add_event::<CollisionEvent>()
        .run();
}
