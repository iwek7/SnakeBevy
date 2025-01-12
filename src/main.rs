mod cars;
mod sneko;
mod systems;

use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};

use std::cmp::PartialEq;
use std::env;
use std::ops::Div;

use bevy::prelude::*;
use rand::Rng;

// example command `cargo run sneko`
fn main() {
    let args: Vec<String> = env::args().collect();
    let game_name = &args[1];
    println!("Lounching game {}", game_name);
    match game_name.as_str() {
        "sneko" => sneko::lounch_snake(),
        "cars" => cars::launch_cars(),
        _ => {
            println!("This game does not exist");
        }
    }
    if game_name == "sneko" {
        sneko::lounch_snake();
    } else {
    }
    //
    // cars::launch_cars();
}
