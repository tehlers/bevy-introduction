use bevy::prelude::*;

#[derive(Component)]
struct Dog;

fn add_dogs(mut commands: Commands) {
    commands.spawn(Dog);
}

// example-start: 1 {1|2-4|10|all}
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
// example-start: 1
