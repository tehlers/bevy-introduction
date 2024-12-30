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

<!-- cmd:jump_to_middle -->

Let's write a game!
===================

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