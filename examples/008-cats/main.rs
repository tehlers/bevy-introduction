use bevy::prelude::*;

#[derive(Component)]
struct Dog;

#[derive(Component)]
struct Name(String);

// example-start: 1 {1,2|8|all}
#[derive(Component)]
struct Cat;

fn add_animals(mut commands: Commands) {
    commands.spawn((Dog, Name("Hachikō".to_string())));
    commands.spawn((Dog, Name("Laika".to_string())));
    commands.spawn((Dog, Name("Rantanplan".to_string())));
    commands.spawn((Cat, Name("Garfield".to_string())));
}
// example-end: 1

fn bark(dogs: Query<(&Dog, &Name)>) {
    for (_, name) in dogs.iter() {
        println!("{}: \"Woof\"", name.0);
    }
}

// example-start: 2 {1-5|11|all}
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
// example-end: 2
