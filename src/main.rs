mod components;
mod config;
mod cars;
mod systems;
mod sneko;

use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};

use std::cmp::PartialEq;
use std::ops::Div;

use bevy::prelude::*;
use rand::Rng;
fn main() {
    sneko::lounch_snake();
    // cars::launch_cars();
}


