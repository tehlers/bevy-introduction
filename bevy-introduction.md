---
title: Game development with Rust and Bevy
sub_title: An introduction to Bevy 0.15.0
theme:
  name: dark
options:
  command_prefix: "cmd:"
---

Hello world
===========

<!-- include-code: examples/001-hello_world/main.rs§1 -->
```rust +line_numbers
fn main() {
    println!("Hello, world!");
}
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
