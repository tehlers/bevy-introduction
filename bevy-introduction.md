---
title: Game development with Rust and Bevy
sub_title: An introduction to Bevy (Version 0.15.0)
theme:
  name: dark
options:
  command_prefix: "cmd:"
---

What is Bevy?
=============

# Facts about Bevy

<!-- cmd:end_slide -->

Getting started...
==================

# How to install Rust

<!-- cmd:end_slide -->

<!-- cmd:jump_to_middle -->

The basics
==========

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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

<!-- cmd:end_slide -->

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
                min_height: 1080.0,
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

<!-- cmd:end_slide -->

Caveats and things to keep in mind
==================================

<!-- cmd:end_slide -->

Documentation and resources
===========================

<!-- cmd:end_slide -->

<!-- cmd:jump_to_middle -->

Thanks for listening and now write some games!
==============================================
