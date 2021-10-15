use bevy_study::bevy::prelude::*;

fn main() {
    App::new().add_system(hello_world_system).run();
}

fn hello_world_system() {
    println!("hello world");
}