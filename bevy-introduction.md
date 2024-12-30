---
title: Game development with Rust and Bevy
sub_title: An introduction to Bevy (Version 0.15.0)
theme:
  name: dark
options:
  command_prefix: "cmd:"
  implicit_slide_ends: true
---

What is Bevy?
=============

# Facts about Bevy

Getting started...
==================

# How to install Rust

<!-- cmd:end_slide -->

<!-- cmd:jump_to_middle -->

The basics
==========

Hello world
===========

<!-- include-code: examples/001-hello_world/main.rs§1 -->
```rust +line_numbers
fn main() {
    println!("Hello, world!");
}
```

```sh +exec
cargo run --example 001-hello_world
```

Simple Bevy app
===============

<!-- include-code: examples/002-bevy_app/main.rs§1 -->
```rust +line_numbers {1|4|all}
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

```sh +exec
cargo run --example 002-bevy_app
```

Systems
=======

<!-- include-code: examples/003-bevy_hello_world/main.rs§1 -->
```rust +line_numbers {3-5|8|all}
use bevy::prelude::*;

fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
```

```sh +exec
just run 003-bevy_hello_world
```

Components
==========

<!-- include-code: examples/004-dog/main.rs§1 -->
```rust +line_numbers {3,4|6|7|13|all}
use bevy::prelude::*;

#[derive(Component)]
struct Dog;

fn add_dogs(mut commands: Commands) {
    commands.spawn(Dog);
}

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, add_dogs);
    app.run();
}
```

```sh +exec
just run 004-dog
```

Queries
=======

<!-- include-code: examples/005-barking_dog/main.rs§1 -->
```rust +line_numbers {1|2-4|10|all}
fn bark(dogs: Query<&Dog>) {
    for _ in dogs.iter() {
        println!("Woof");
    }
}

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, add_dogs).add_systems(Update, bark);
    app.run();
}
```

```sh +exec
just run 005-barking_dog
```

Multiple entities
=================

<!-- include-code: examples/006-barking_dogs/main.rs§1 -->
```rust +line_numbers
fn add_dogs(mut commands: Commands) {
    commands.spawn(Dog);
    commands.spawn(Dog);
    commands.spawn(Dog);
}
```

```sh +exec
just run 006-barking_dogs
```

Multiple components
===================

<!-- include-code: examples/007-dogs_with_names/main.rs§1 -->
```rust +line_numbers {1,2|5-7|10|11|12|all}
#[derive(Component)]
struct Name(String);

fn add_dogs(mut commands: Commands) {
    commands.spawn((Dog, Name("Hachikō".to_string())));
    commands.spawn((Dog, Name("Laika".to_string())));
    commands.spawn((Dog, Name("Rantanplan".to_string())));
}

fn bark(dogs: Query<(&Dog, &Name)>) {
    for (_, name) in dogs.iter() {
        println!("{}: \"Woof\"", name.0);
    }
}
```

```sh +exec
just run 007-dogs_with_names
```

Reuse of components (1/2)
=========================

<!-- include-code: examples/008-cats/main.rs§1 -->
```rust +line_numbers {1,2|8|all}
#[derive(Component)]
struct Cat;

fn add_animals(mut commands: Commands) {
    commands.spawn((Dog, Name("Hachikō".to_string())));
    commands.spawn((Dog, Name("Laika".to_string())));
    commands.spawn((Dog, Name("Rantanplan".to_string())));
    commands.spawn((Cat, Name("Garfield".to_string())));
}
```

Reuse of components (2/2)
=========================

<!-- include-code: examples/008-cats/main.rs§2 -->
```rust +line_numbers {1-5|11|all}
fn meow(cats: Query<(&Cat, &Name)>) {
    for (_, name) in cats.iter() {
        println!("{}: \"Meow\"", name.0);
    }
}

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, add_animals)
        .add_systems(Update, (bark, meow));

    app.run();
}
```

```sh +exec
just run 008-cats
```

Plugins
=======

<!-- include-code: examples/009-bevy_minimal_plugins/main.rs§1 -->
```rust +line_numbers {4|all}
fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, add_dogs)
        .add_systems(Update, (bark, meow));

    app.run();
}
```

```sh +exec
just run 009-bevy_minimal_plugins
```

<!-- cmd:end_slide -->

<!-- cmd:jump_to_middle -->

Let's write a game!
===================

Open a window
=============

<!-- include-code: examples/010-bevy_default_plugins/main.rs§1 -->
```rust +line_numbers {4|all}
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

```sh +exec
just run 010-bevy_default_plugins
```

Sprites
=======

<!-- include-code: examples/011-sprite/main.rs§1 -->
```rust +line_numbers {1|2|3|10|all}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset_server.load("sprites/ball.png")));
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
```

```sh +exec
just run 011-sprite
```

Scale screen
============

<!-- include-code: examples/012-scale_screen/main.rs§1 -->
```rust +line_numbers {4-10|all}
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
```

```sh +exec
just run 012-scale_screen
```

Move sprite (1/2)
=================

<!-- include-code: examples/013-move_ball/main.rs§1 -->
```rust +line_numbers {1|3-6|22-24|all}
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
```

Move sprite (2/2)
=================

<!-- include-code: examples/013-move_ball/main.rs§2 -->
```rust +line_numbers {1|2-5|13|all}
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
```

```sh +exec
just run 013-move_ball
```

Walls (1/3)
===========

<!-- include-code: examples/014-add_walls/main.rs§1 -->
```rust +line_numbers {1-2|3|all}
const MAX_X: f32 = 1920.0;
const MAX_Y: f32 = 1200.0;
const WALL_THICKNESS: f32 = 20.0;
const BALL_SPEED: f32 = 400.0;
```

<!-- include-code: examples/014-add_walls/main.rs§2 -->
```rust +line_numbers {1|3|4-8|all}
    // Top wall
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform::from_xyz(0.0, MAX_Y / 2.0, 0.0).with_scale(Vec3::new(
            MAX_X,
            WALL_THICKNESS,
            0.0,
        )),
    ));
```

Walls (2/3)
===========

<!-- include-code: examples/014-add_walls/main.rs§3 -->
```rust +line_numbers {1-9|11-19|21-29|all}
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
```

Walls (3/3)
===========

```sh +exec
just run 014-add_walls
```

Command (1/3)
=============

<!-- include-code: examples/015-use_command_for_walls/main.rs§1 -->
```rust +line_numbers {1-6|8|9|10-15|18-22|all}
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
```

Command (2/3)
=============

<!-- include-code: examples/015-use_command_for_walls/main.rs§2 -->
```rust +line_numbers {1|2|5|6|7-10|all}
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
```

Command (3/3)
=============

<!-- include-code: examples/015-use_command_for_walls/main.rs§3 -->
```rust +line_numbers {1-3|4-6|7-9|10-12|all}
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
```

```sh +exec
just run 015-use_command_for_walls
```

Collision (1/4)
===============

<!-- include-code: examples/016-add_collision/main.rs§1 -->
```rust +line_numbers {7-8|4|all}
const MAX_X: f32 = 1920.0;
const MAX_Y: f32 = 1200.0;
const WALL_THICKNESS: f32 = 20.0;
const BALL_RADIUS: f32 = 12.0;
const BALL_SPEED: f32 = 600.0;

#[derive(Component)]
struct Collider;
```

<!-- include-code: examples/016-add_collision/main.rs§2 -->
```rust +line_numbers {0|6|all}
impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        world.spawn((
            Sprite::from_color(Color::WHITE, Vec2::ONE),
            Transform::from_translation(self.location.position()).with_scale(self.location.size()),
            Collider,
        ));
    }
}
```

Collision (2/4)
===============

<!-- include-code: examples/016-add_collision/main.rs§3 -->
```rust +line_numbers {1|2|5|3|6|7-13|all}
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
```

<!-- include-code: examples/016-add_collision/main.rs§4 -->
```rust +line_numbers {0|2|all}
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    render::camera::ScalingMode,
};
```

Collision (3/4)
===============

<!-- include-code: examples/016-add_collision/main.rs§5 -->
```rust +line_numbers {1-6|10|11-13|15-29|all}
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
```

Collision (4/4)
===============

<!-- include-code: examples/016-add_collision/main.rs§6 -->
```rust +line_numbers {1|8-13|15-23|all}
            if let Some(collision) = collision {
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
```

```sh +exec
just run 016-add_collision
```

Stones (1/2)
============

<!-- include-code: examples/017-add_stones/main.rs§1 -->
```rust +line_numbers {1-4|8|9-13|all}
struct SpawnStone {
    x: f32,
    y: f32,
}

impl Command for SpawnStone {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            world.spawn((
                Sprite::from_image(asset_server.load("sprites/stone.png")),
                Transform::from_xyz(self.x, self.y, 0.0),
                Collider,
            ));
        }
    }
}
```

Stones (2/2)
============

<!-- include-code: examples/017-add_stones/main.rs§2 -->
```rust +line_numbers {1|2|all}
const MARGIN: f32 = 12.0;
const STONE_SIZE: Vec2 = Vec2::new(82.0, 28.0);
```

<!-- include-code: examples/017-add_stones/main.rs§3 -->
```rust +line_numbers {0|1-4|5-7|8-11|all}
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
```

```sh +exec
just run 017-add_stones
```

Collision with stones (1/3)
===========================

<!-- include-code: examples/018-stone_collision/main.rs§1 -->
```rust +line_numbers {3|all}
#[derive(Component)]
struct Collider {
    size: Option<Vec2>,
}
```

<!-- include-code: examples/018-stone_collision/main.rs§2 -->
```rust +line_numbers {0|6|all}
impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        world.spawn((
            Sprite::from_color(Color::WHITE, Vec2::ONE),
            Transform::from_translation(self.location.position()).with_scale(self.location.size()),
            Collider { size: None },
        ));
    }
}
```

Collision with stones (2/3)
===========================

<!-- include-code: examples/018-stone_collision/main.rs§3 -->
```rust +line_numbers {7-9|all}
impl Command for SpawnStone {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            world.spawn((
                Sprite::from_image(asset_server.load("sprites/stone.png")),
                Transform::from_xyz(self.x, self.y, 0.0),
                Collider {
                    size: Some(STONE_SIZE),
                },
            ));
        }
    }
}
```

Collision with stones (3/3)
===========================

<!-- include-code: examples/018-stone_collision/main.rs§4 -->
```rust +line_numbers {3|6|11|all}
fn check_for_collisions(
    mut balls: Query<(&mut Ball, &Transform)>,
    obstacles: Query<(&Transform, &Collider)>,
) {
    for (mut ball, ball_transform) in &mut balls {
        for (obstacle, collider) in &obstacles {
            let collision = ball_collision(
                BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS),
                Aabb2d::new(
                    obstacle.translation.truncate(),
                    collider.size.unwrap_or(obstacle.scale.truncate()) / 2.,
                ),
            );
```

```sh +exec
just run 018-stone_collision
```

Caveats and things to keep in mind
==================================

Documentation and resources
===========================

<!-- cmd:end_slide -->

<!-- cmd:jump_to_middle -->

Thanks for listening and now write some games!
==============================================
