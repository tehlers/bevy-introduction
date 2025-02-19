use bevy::prelude::*;

#[derive(Component)]
struct Dog;

// example-start: 1 {0|1,2|5-7|10,14|11,13|12|1-2,5-7,10-14}
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
// example-end: 1

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, add_dogs).add_systems(Update, bark);

    app.run();
}
