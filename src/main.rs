use bevy::prelude::*;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct HP(u32);

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, spawn_brick);

    app.add_systems(Update, (hello_bevy, hello_bricks));

    app.run();
}

fn spawn_brick(mut commands: Commands) {
    commands.spawn((Brick, HP(0x20)));
}

fn hello_bevy() {
    println!("Hello Bevy!");
}

fn hello_bricks(query: Query<&HP, With<Brick>>) {
    for brick in &query {
        println!("Found a brick with {} hp!", brick.0);
    }
}
