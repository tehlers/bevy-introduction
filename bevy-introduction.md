---
title: Game development with Rust and Bevy
sub_title: An introduction to Bevy (Version 0.15.0)
theme:
  name: dark
  override:
    footer:
      style: empty
options:
  command_prefix: "cmd:"
  implicit_slide_ends: true
---

What is Bevy?
=============

<!-- cmd: column_layout: [3, 1] -->

<!-- cmd: column: 0 -->

In their own words:

> A refreshingly simple data-driven game engine built in Rust
> Free and Open Source Forever!

* Work started in November 2019
* First public release in September 2020
* 37.000 stars on GitHub
* ECS architecture (Entity Component System)
* Focus on a simple data-oriented design
* Support for 2D and 3D games
* Built in Rust
* Cross-platform with support for Linux, MacOS, Windows, iOS and Android
* Active community with new releases roughly every 3-6 months

<!-- cmd: column: 1 -->

![Bevy logo](assets/images/bevy_bird_dark.png)
![Rust logo](assets/images/rust_logo.png)

Getting started...
==================

# Installing the Rust toolchain

The Rust toolchain is installed with [Rustup](https://rustup.rs/). Either install it with the package manager of your distribution or download the installations script. **Always check the downloaded code and do not blindly pipe it to `sh`.**

# Important tools

* `rustc` - the Rust compiler
* `cargo` - build tool and package manager
* `rust-analyzer` - LSP implementation for Rust

<!-- cmd:end_slide -->

<!-- cmd:jump_to_middle -->

The basics
==========

Hello world
===========

<!-- cmd: column_layout: [1, 1] -->

<!-- cmd: column: 0 -->

# Creating a new Rust project

```sh +exec
NEW_PROJECT=$(mktemp -d)
cd "$NEW_PROJECT"
cargo init hello_world
cd hello_world
cargo run
```

<!-- cmd: column: 1 -->

<!-- cmd:pause -->

## Cargo.toml

```toml +line_numbers
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

<!-- cmd:pause -->

## src/main.rs

<!-- include-code: examples/001-hello_world/main.rs§1 -->
```rust +line_numbers
fn main() {
    println!("Hello, world!");
}
```

Simple Bevy app
===============

<!-- cmd: column_layout: [1, 1] -->

<!-- cmd: column: 0 -->

## Cargo.toml

```toml +line_numbers {0|7}
[package]
name = "bevy-introduction"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.15.0"
```

<!-- cmd:pause -->
<!-- cmd: column: 1 -->

## examples/002-bevy_app/main.rs

<!-- include-code: examples/002-bevy_app/main.rs§1 -->
```rust +line_numbers {0|1|4|all}
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

<!-- cmd: reset_layout -->

<!-- cmd:pause -->
```sh +exec
cargo run --example 002-bevy_app
```

Entity Component System
=======================

<!-- cmd: column_layout: [1, 1, 1] -->

<!-- cmd:pause -->
<!-- cmd: column: 0 -->
# Entity

All "things" in a game are **entities**. e.g.
* the player
* enemies
* potions
* ...

At its core, it is an ID that is managed by Bevy.

<!-- cmd:pause -->
<!-- cmd: column: 1 -->
# Component

Each **entity** can have several **components** that define its properties, characteristics and capabilities. e.g.

* hitpoints
* position
* speed
* ...

<!-- cmd:pause -->
<!-- cmd: column: 2 -->
# System

The actual game logic is implemented in functions that are called **systems**. They are defined for a specific group of **component** types. e.g.

* change position of entities with speed
* ...

<!-- cmd:pause -->
<!-- cmd: reset_layout -->

> The ECS pattern encourages clean, decoupled designs by forcing you to break up your app data and logic into its core components. It also helps make your code faster by optimizing memory access patterns and making parallelism easier.

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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
```sh +exec
just run 010-bevy_default_plugins
```

Sprites
=======

<!-- cmd: column_layout: [3, 1] -->

<!-- cmd: column: 0 -->
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

<!-- cmd: column: 1 -->

## assets/sprites/ball.png
![ball sprite](assets/sprites/ball.png)

<!-- cmd:pause -->
<!-- cmd:reset_layout -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
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

<!-- cmd:pause -->
```sh +exec
just run 018-stone_collision
```

Despawn stones (1/2)
====================

<!-- include-code: examples/019-despawn_stones/main.rs§1 -->
```rust +line_numbers
#[derive(Component)]
struct Stone;
```

<!-- include-code: examples/019-despawn_stones/main.rs§2 -->
```rust +line_numbers {0|10|all}
impl Command for SpawnStone {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            world.spawn((
                Sprite::from_image(asset_server.load("sprites/stone.png")),
                Transform::from_xyz(self.x, self.y, 0.0),
                Collider {
                    size: Some(STONE_SIZE),
                },
                Stone,
            ));
        }
    }
}
```

Despawn stones (2/2)
====================

<!-- include-code: examples/019-despawn_stones/main.rs§3 -->
```rust +line_numbers {1|4|7|17-19|all}
fn check_for_collisions(
    mut commands: Commands,
    mut balls: Query<(&mut Ball, &Transform)>,
    obstacles: Query<(Entity, &Transform, &Collider, Option<&Stone>)>,
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
                if maybe_stone.is_some() {
                    commands.entity(entity).despawn();
                }
```

<!-- cmd:pause -->
```sh +exec
just run 019-despawn_stones
```

Animation (1/3)
===============

<!-- include-code: examples/020-animate_despawning/main.rs§1 -->
```rust +line_numbers {3-9|10,11|15-21|all}
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
                },
                Stone,
            ));
        }
    }
}
```

Animation (2/3)
===============

<!-- include-code: examples/020-animate_despawning/main.rs§2 -->
```rust +line_numbers
#[derive(Component)]
struct Despawning(Timer);
```

<!-- include-code: examples/020-animate_despawning/main.rs§3 -->
```rust +line_numbers {0|2-3|4|all}
                if maybe_stone.is_some() {
                    commands
                        .entity(entity)
                        .insert(Despawning(Timer::from_seconds(0.01, TimerMode::Repeating)));
                }
```

Animation (3/3)
===============

<!-- include-code: examples/020-animate_despawning/main.rs§4 -->
```rust +line_numbers {0|3|4|7|8|9-14|all}
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
```

<!-- include-code: examples/020-animate_despawning/main.rs§5 -->
```rust +line_numbers {0|3|all}
        .add_systems(
            Update,
            (apply_velocity, check_for_collisions, despawn_stones),
        )
```

<!-- cmd:pause -->
```sh +exec
just run 020-animate_despawning
```

Events and sounds (1/5)
=======================

<!-- include-code: examples/021-add_sounds/main.rs§1 -->
```rust +line_numbers {1-5|10|13-16|all}
#[derive(Clone, Copy)]
enum Obstacle {
    Stone,
    Wall,
}

#[derive(Component)]
struct Collider {
    size: Option<Vec2>,
    obstacle: Obstacle,
}

#[derive(Event)]
struct CollisionEvent {
    obstacle: Obstacle,
}
```

Events and sounds (2/5)
=======================

<!-- include-code: examples/021-add_sounds/main.rs§2 -->
```rust +line_numbers {6|all}
        world.spawn((
            Sprite::from_color(Color::WHITE, Vec2::ONE),
            Transform::from_translation(self.location.position()).with_scale(self.location.size()),
            Collider {
                size: None,
                obstacle: Obstacle::Wall,
            },
        ));
```

<!-- include-code: examples/021-add_sounds/main.rs§3 -->
```rust +line_numbers {0|12|all}
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
```

Events and sounds (3/5)
=======================

<!-- include-code: examples/021-add_sounds/main.rs§4 -->
```rust +line_numbers {0|5|18-20|all}
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
```

Events and sounds (4/5)
=======================

<!-- include-code: examples/021-add_sounds/main.rs§5 -->
```rust +line_numbers {0|3|6|8-11|12-15|all}
fn play_sounds(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in collision_events.read() {
        match event.obstacle {
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
```

Events and sounds (5/5)
=======================

<!-- include-code: examples/021-add_sounds/main.rs§6 -->
```rust +line_numbers {0|7|10|all}
        .add_systems(
            Update,
            (
                apply_velocity,
                check_for_collisions,
                despawn_stones,
                play_sounds,
            ),
        )
        .add_event::<CollisionEvent>()
```

<!-- cmd:pause -->
```sh +exec
just run 021-add_sounds
```

Input (1/5)
===========

<!-- include-code: examples/022-add_bat/main.rs§1 -->
```rust +line_numbers {1-2|6|1-2,6}
#[derive(Component)]
struct Bat;

#[derive(Clone, Copy)]
enum Obstacle {
    Bat,
    Stone,
    Wall,
}
```

<!-- include-code: examples/022-add_bat/main.rs§2 -->
```rust +line_numbers {0|all}
const BAT_SIZE: Vec2 = Vec2::new(124.0, 28.0);
```

<!-- include-code: examples/022-add_bat/main.rs§3 -->
```rust +line_numbers {0|2|3|4-6|8|all}
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/bat.png")),
        Transform::from_xyz(0.0, -MAX_Y / 2.0 + WALL_THICKNESS + MARGIN, 0.0),
        Collider {
            size: Some(BAT_SIZE),
            obstacle: Obstacle::Bat,
        },
        Bat,
    ));
```

Input (2/5)
===========

<!-- include-code: examples/022-add_bat/main.rs§4 -->
```rust +line_numbers
            Obstacle::Bat => commands.spawn((
                AudioPlayer::new(asset_server.load("sounds/bat.ogg")),
                PlaybackSettings::DESPAWN,
            )),
```

Input (3/5)
===========

<!-- include-code: examples/022-add_bat/main.rs§5 -->
```rust +line_numbers {0|2}
use bevy::{
    input::mouse::MouseMotion,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    render::camera::ScalingMode,
    window::PrimaryWindow,
};
```

<!-- include-code: examples/022-add_bat/main.rs§6 -->
```rust +line_numbers {0|2-3}
const BAT_SIZE: Vec2 = Vec2::new(124.0, 28.0);
const BAT_LEFT_BORDER: f32 = -(MAX_X / 2.0) + WALL_THICKNESS + BAT_SIZE.x / 2.0;
const BAT_RIGHT_BORDER: f32 = -BAT_LEFT_BORDER;
```

<!-- include-code: examples/022-add_bat/main.rs§7 -->
```rust +line_numbers {0|1|2|3-6|all}
fn move_bat(mut motion: EventReader<MouseMotion>, mut bat_query: Query<&mut Transform, With<Bat>>) {
    for event in motion.read() {
        for mut bat in &mut bat_query {
            bat.translation.x += event.delta.x * 2.0;
            bat.translation.x = bat.translation.x.clamp(BAT_LEFT_BORDER, BAT_RIGHT_BORDER);
        }
    }
}
```

Input (4/5)
===========

<!-- include-code: examples/022-add_bat/main.rs§8 -->
```rust +line_numbers {0|7}
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
```

<!-- include-code: examples/022-add_bat/main.rs§9 -->
```rust +line_numbers {0|6}
use bevy::{
    input::mouse::MouseMotion,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    render::camera::ScalingMode,
    window::PrimaryWindow,
};
```

<!-- include-code: examples/022-add_bat/main.rs§10 -->
```rust +line_numbers {0|4|6-7|4,6-7}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    primary_window.cursor_options.visible = false;
```

Input (5/5)
===========

```sh +exec
just run 022-add_bat
```

Game states (1/5)
=================

<!-- include-code: examples/023-add_title/main.rs§1 -->
```rust +line_numbers {0|2|4-5|1|3|all}
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Title,
    Game,
}
```

Game states (2/5)
=================

<!-- include-code: examples/023-add_title/main.rs§2 -->
```rust +line_numbers {0|all}
fn setup(mut commands: Commands, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = windows.single_mut();
    primary_window.cursor_options.visible = false;

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
    // --> moved to `setup_game`
}
```

<!-- include-code: examples/023-add_title/main.rs§3 -->
```rust +line_numbers {0|all}
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.queue(SpawnWall {
        location: WallLocation::Top,
    });
    // ...
```

Game states (3/5)
=================

<!-- include-code: examples/023-add_title/main.rs§4 -->
```rust +line_numbers {0|1|2|4-8|10-14|all}
fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/AllertaStencil-Regular.ttf");

    let text_font = TextFont {
        font: font.clone(),
        font_size: 128.0,
        ..default()
    };

    commands.spawn((
        Text2d::new("Breakout"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}
```

<!-- include-code: examples/023-add_title/main.rs§5 -->
```rust +line_numbers {0|1|2|all}
fn start_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Game);
}
```

Game states (4/5)
=================

<!-- include-code: examples/023-add_title/main.rs§6 -->
```rust +line_numbers {26|5|6|7|8-13|23|5-13,23,26}
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Title), setup_title)
        .add_systems(OnEnter(GameState::Game), setup_game)
        .add_systems(
            Update,
            (start_game)
                .run_if(in_state(GameState::Title))
                .run_if(input_just_pressed(KeyCode::Enter)),
        )
        .add_systems(
            Update,
            (
                apply_velocity,
                check_for_collisions,
                despawn_stones,
                move_bat,
                play_sounds,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_event::<CollisionEvent>()
        .init_state::<GameState>()
        .run();
}
```

Game states (5/5)
=================

<!-- include-code: examples/023-add_title/main.rs§7 -->
```rust +line_numbers {0|2}
use bevy::{
    input::{common_conditions::input_just_pressed, mouse::MouseMotion},
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    render::camera::ScalingMode,
    window::PrimaryWindow,
};
```

<!-- cmd:pause -->
```sh +exec
just run 023-add_title
```

Despawn on game state changes (1/6)
===================================

<!-- include-code: examples/024-despawn_with_state_change/main.rs§1 -->
```rust +line_numbers {0|1-2|all}
#[derive(Component)]
struct OnTitleScreen;

#[derive(Component)]
struct OnGameScreen;
```

Despawn on game state changes (2/6)
===================================

<!-- include-code: examples/024-despawn_with_state_change/main.rs§2 -->
```rust +line_numbers {0|14}
fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/AllertaStencil-Regular.ttf");

    let text_font = TextFont {
        font: font.clone(),
        font_size: 128.0,
        ..default()
    };

    commands.spawn((
        Text2d::new("Breakout"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        OnTitleScreen,
    ));
}
```

Despawn on game state changes (3/6)
===================================

<!-- include-code: examples/024-despawn_with_state_change/main.rs§3 -->
```rust +line_numbers {0|10}
impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        world.spawn((
            Sprite::from_color(Color::WHITE, Vec2::ONE),
            Transform::from_translation(self.location.position()).with_scale(self.location.size()),
            Collider {
                size: None,
                obstacle: Obstacle::Wall,
            },
            OnGameScreen,
        ));
    }
}
```

Despawn on game state changes (4/6)
===================================

<!-- include-code: examples/024-despawn_with_state_change/main.rs§4 -->
```rust +line_numbers {0|15}
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
                OnGameScreen,
            ));
```

Despawn on game state changes (5/6)
===================================

<!-- include-code: examples/024-despawn_with_state_change/main.rs§5 -->
```rust +line_numbers {0|11|11,22}
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
        OnGameScreen,
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/bat.png")),
        Transform::from_xyz(0.0, -MAX_Y / 2.0 + WALL_THICKNESS + MARGIN, 0.0),
        Collider {
            size: Some(BAT_SIZE),
            obstacle: Obstacle::Bat,
        },
        Bat,
        OnGameScreen,
    ));
```

Despawn on game state changes (6/6)
===================================

<!-- include-code: examples/024-despawn_with_state_change/main.rs§6 -->
```rust +line_numbers {0|1|2|3|all}
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
```

<!-- include-code: examples/024-despawn_with_state_change/main.rs§7 -->
```rust +line_numbers {0|3|3,5}
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Title), setup_title)
        .add_systems(OnExit(GameState::Title), despawn_screen::<OnTitleScreen>)
        .add_systems(OnEnter(GameState::Game), setup_game)
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
```

<!-- cmd:pause -->
```sh +exec
just run 024-despawn_with_state_change
```

Game over (1/3)
===============

<!-- include-code: examples/025-remove_bottom_wall/main.rs§1 -->
```rust +line_numbers {0|2-5|2-5,11-14|2-5,11-14,19-23}
enum WallLocation {
    Top,
    // Bottom,
    Left,
    Right,
}

impl WallLocation {
    fn position(&self) -> Vec3 {
        match self {
            WallLocation::Top => Vec3::new(0.0, MAX_Y / 2.0, 0.0),
            // WallLocation::Bottom => Vec3::new(0.0, -MAX_Y / 2.0, 0.0),
            WallLocation::Left => Vec3::new(-MAX_X / 2.0, 0.0, 0.0),
            WallLocation::Right => Vec3::new(MAX_X / 2.0, 0.0, 0.0),
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            /* WallLocation::Bottom | */
            WallLocation::Top => Vec3::new(MAX_X, WALL_THICKNESS, 0.0),
            WallLocation::Left | WallLocation::Right => Vec3::new(WALL_THICKNESS, MAX_Y, 0.0),
        }
    }
}
```

Game over (2/3)
===============

<!-- include-code: examples/025-remove_bottom_wall/main.rs§2 -->
```rust +line_numbers {0|all}
    commands.queue(SpawnWall {
        location: WallLocation::Top,
    });
    /*
    commands.queue(SpawnWall {
        location: WallLocation::Top,
    });
    */
    commands.queue(SpawnWall {
        location: WallLocation::Left,
    });
    commands.queue(SpawnWall {
        location: WallLocation::Right,
    });
```

Game over (3/3)
===============

<!-- include-code: examples/025-remove_bottom_wall/main.rs§3 -->
```rust +line_numbers {0|1-4|6|7|all}
fn check_for_game_over(
    balls: Query<&Transform, With<Ball>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for ball in &balls {
        if ball.translation.y < -MAX_Y / 2.0 {
            game_state.set(GameState::Title);
        }
    }
}
```

<!-- include-code: examples/025-remove_bottom_wall/main.rs§4 -->
```rust +line_numbers {0|6}
        .add_systems(
            Update,
            (
                apply_velocity,
                check_for_collisions,
                check_for_game_over,
                despawn_stones,
                move_bat,
                play_sounds,
            )
                .run_if(in_state(GameState::Game)),
        )
```

<!-- cmd:pause -->
```sh +exec
just run 025-remove_bottom_wall
```

Resources (1/4)
===============

<!-- include-code: examples/026-add_score/main.rs§1 -->
```rust +line_numbers {0|1|2|all}
#[derive(Default, Resource)]
struct Score(u64);
```

<!-- include-code: examples/026-add_score/main.rs§2 -->
```rust +line_numbers {0|1|2|3|4|all}
fn handle_score(mut collision_events: EventReader<CollisionEvent>, mut score: ResMut<Score>) {
    for event in collision_events.read() {
        if let Obstacle::Stone = event.obstacle {
            score.0 += 100;
        }
    }
}
```

Resources (2/4)
===============

<!-- include-code: examples/026-add_score/main.rs§3 -->
```rust +line_numbers {0|8|8,16}
        .add_systems(
            Update,
            (
                apply_velocity,
                check_for_collisions,
                check_for_game_over,
                despawn_stones,
                handle_score,
                move_bat,
                play_sounds,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_event::<CollisionEvent>()
        .init_state::<GameState>()
        .init_resource::<Score>()
        .run();
```

Resources (3/4)
===============

<!-- include-code: examples/026-add_score/main.rs§4 -->
```rust +line_numbers {0|1}
fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
```

<!-- include-code: examples/026-add_score/main.rs§5 -->
```rust +line_numbers {0|1-5|1-13|all}
    let score_font = TextFont {
        font: font.clone(),
        font_size: 64.0,
        ..default()
    };

    let mut score_text = commands.spawn((
        Text2d::new(format!("Last score: {}", score.0)),
        score_font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(0.0, -256.0, 0.0),
        OnTitleScreen,
    ));

    if score.0 == 0 {
        score_text.insert(Visibility::Hidden);
    }
```

Resources (4/4)
===============

<!-- include-code: examples/026-add_score/main.rs§6 -->
```rust +line_numbers {0|1|2|all}
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, mut score: ResMut<Score>) {
    score.0 = 0;
    // ...
```

<!-- cmd:pause -->
```sh +exec
just run 026-add_score
```

Testing (1/4)
=============

<!-- include-code: examples/027-add_test/main.rs§1 -->
```rust +line_numbers {0|1|1-2|1-2,5-6|all}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_increased_only_for_stones() {
        // ...
```

Testing (2/4)
=============

<!-- include-code: examples/027-add_test/main.rs§2 -->
```rust +line_numbers {0|1|3-5|7-8|10-14|15-16|all}
        let mut app = App::new();

        app.init_resource::<Score>()
            .add_event::<CollisionEvent>()
            .add_systems(Update, handle_score);

        app.update();
        assert_eq!(app.world().resource::<Score>().0, 0);

        app.world_mut()
            .resource_mut::<Events<CollisionEvent>>()
            .send(CollisionEvent {
                obstacle: Obstacle::Bat,
            });
        app.update();
        assert_eq!(app.world().resource::<Score>().0, 0);
        // ...
```

Testing (3/4)
=============

<!-- include-code: examples/027-add_test/main.rs§3 -->
```rust +line_numbers {2-8|10-16|all}
        // ...
        app.world_mut()
            .resource_mut::<Events<CollisionEvent>>()
            .send(CollisionEvent {
                obstacle: Obstacle::Wall,
            });
        app.update();
        assert_eq!(app.world().resource::<Score>().0, 0);

        app.world_mut()
            .resource_mut::<Events<CollisionEvent>>()
            .send(CollisionEvent {
                obstacle: Obstacle::Stone,
            });
        app.update();
        assert_eq!(app.world().resource::<Score>().0, 100);
```

Testing (4/4)
=============

```sh +exec
cargo test --example 027-add_test
```

Caveats and things to keep in mind
==================================

Documentation and resources
===========================

<!-- cmd:end_slide -->

<!-- cmd:jump_to_middle -->

Thanks for listening and now write some games!
==============================================
