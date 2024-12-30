use bevy::prelude::*;

#[derive(Component)]
struct Dog;

// example-start: 1
fn add_dogs(mut commands: Commands) {
    commands.spawn(Dog);
    commands.spawn(Dog);
    commands.spawn(Dog);
}
// example-end: 1

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
