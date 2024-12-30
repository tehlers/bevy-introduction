use bevy::prelude::*;

#[derive(Component)]
struct Cat;

#[derive(Component)]
struct Dog;

#[derive(Component)]
struct Name(String);

fn add_dogs(mut commands: Commands) {
    commands.spawn((Dog, Name("Hachikō".to_string())));
    commands.spawn((Dog, Name("Laika".to_string())));
    commands.spawn((Dog, Name("Rantanplan".to_string())));
    commands.spawn((Cat, Name("Garfield".to_string())));
}

fn bark(dogs: Query<(&Dog, &Name)>) {
    for (_, name) in dogs.iter() {
        println!("{}: \"Woof\"", name.0);
    }
}

fn meow(cats: Query<(&Cat, &Name)>) {
    for (_, name) in cats.iter() {
        println!("{}: \"Meow\"", name.0);
    }
}

// example-start: 1 {4|all}
fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, add_dogs)
        .add_systems(Update, (bark, meow));

    app.run();
}
// example-end: 1
