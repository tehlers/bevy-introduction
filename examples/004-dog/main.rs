// example-start: 1 {3,4|6|7|13|all}
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
// example-end: 1
