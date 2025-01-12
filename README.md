# Bevy introduction

An introduction to the game engine [Bevy](https://bevyengine.org) made with [presenterm](https://mfontanini.github.io/presenterm/).
The core concepts of Bevy are explained using a series of examples that result in a simple breakout game at the end.

## Installation

You need to install the Rust toolchain to execute the examples.
Follow the instructions on [Rust's official setup page](https://www.rust-lang.org/learn/get-started).
If you are using Linux, you should also check the [official Bevy Linux dependencies page](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).

## Execution

Once you have installed all dependencies, you can run the examples with

    cargo run --example EXAMPLE

[Presenterm](https://mfontanini.github.io/presenterm/) must be installed in order to display the presentation with

    presenterm --enable-snippet-execution --present bevy-introduction.md

The project contains a [Justfile](https://just.systems/) that provides frequently used commands.

## Development

To compile and lint all examples and execute the tests call

    just check

Update the examples in the presentation with

    just update-example-code

## Examples

### [001-hello_world](examples/001-hello_world/main.rs)

"Hello, world" example generated with `cargo init`

    just run 001-hello_world

### [002-bevy_app](examples/002-bevy_app/main.rs)

A Bevy app without any functionality

    just run 002-bevy_app

### [003-bevy_hello_world](examples/003-bevy_hello_world/main.rs)

"Hello, world" as a Bevy app

    just run 003-bevy_hello_world

### [004-dog](examples/004-dog/main.rs)

Component representing a `Dog`

    just run 004-dog

### [005-barking_dog](examples/005-barking_dog/main.rs)

System making the `Dog` bark

    just run 005-barking_dog

### [006-barking_dogs](examples/006-barking_dogs/main.rs)

More `Dog`s

    just run 006-barking_dogs

### [007-dogs_with_names](examples/007-dogs_with_names/main.rs)

Additional component to assign `Name`s to `Dog`s

    just run 007-dogs_with_names

### [008-cats](examples/008-cats/main.rs)

A new component `Cat` that is used together with a `Name`

    just run 008-cats

### [009-bevy_minimal_plugins](examples/009-bevy_minimal_plugins/main.rs)

The minimal Bevy plugins to add e.g. schedules

    just run 009-bevy_minimal_plugins

### [010-bevy_default_plugins](examples/010-bevy_default_plugins/main.rs)

The default plugins to open an empty window

    just run 010-bevy_default_plugins

### [011-sprite](examples/011-sprite/main.rs)

Sprite of a ball

    just run 011-sprite

### [012-scale_screen](examples/012-scale_screen/main.rs)

Fixed scaling of screen

    just run 012-scale_screen

### [013-move_ball](examples/013-move_ball/main.rs)

Moving sprite

    just run 013-move_ball

### [014-add_walls](examples/014-add_walls/main.rs)

Walls around the playing field

    just run 014-add_walls

### [015-use_command_for_walls](examples/015-use_command_for_walls/main.rs)

Simplified wall creation using a `Command`

    just run 015-use_command_for_walls

### [016-add_collision](examples/016-add_collision/main.rs)

Ball is reflected at walls

    just run 016-add_collision

### [017-add_stones](examples/017-add_stones/main.rs)

Playing field is filled with stones

    just run 017-add_stones

### [018-stone_collision](examples/018-stone_collision/main.rs)

Ball collides with stones

    just run 018-stone_collision

### [019-despawn_stones](examples/019-despawn_stones/main.rs)

Stones are despawned when hit by the ball

    just run 019-despawn_stones

### [020-animate_despawning](examples/020-animate_despawning/main.rs)

Despawning of stones is animated

    just run 020-animate_despawning

### [021-add_sounds](examples/021-add_sounds/main.rs)

Event on collision triggers sound effects

    just run 021-add_sounds

### [022-add_bat](examples/022-add_bat/main.rs)

Bat that is moved by mouse motions

    just run 022-add_bat

### [023-add_title](examples/023-add_title/main.rs)

Initial game state with title screen

    just run 023-add_title

### [024-despawn_with_state_change](examples/024-despawn_with_state_change/main.rs)

Despawn entities on game state change

    just run 024-despawn_with_state_change

### [025-remove_bottom_wall](examples/025-remove_bottom_wall/main.rs)

Game can be lost

    just run 025-remove_bottom_wall

### [026-add_score](examples/026-add_score/main.rs)

Resource containing a score

    just run 026-add_score

### [027-add_test](examples/027-add_test/main.rs)

Test of score event processing

    just run 027-add_test

## Acknowledgements

The font used is [Allerta Stencil](https://github.com/google/fonts/tree/main/ofl/allertastencil). It was published under the terms and conditions of the [OFL](assets/fonts/OFL.txt).

Sound effects were obtained from [Kenney](https://www.kenney.nl) and [Zapsplat](https://www.zapsplat.com).

## License

All code in this repository is licensed under the [MIT License](LICENSE).

The assets included in this repository fall under different open licenses.
