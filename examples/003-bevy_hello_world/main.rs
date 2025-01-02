// example-start: 1 {0|3-5|3-5,8}
use bevy::prelude::*;

fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
// example-end: 1
