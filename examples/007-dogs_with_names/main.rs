use bevy::prelude::*;

#[derive(Component)]
struct Dog;

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

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, add_dogs).add_systems(Update, bark);

    app.run();
}
