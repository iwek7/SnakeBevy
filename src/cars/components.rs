use std::cmp::{max, min};
use bevy::prelude::*;
use crate::cars::config::NUMBER_OF_TRACKS;
use crate::cars::systems::calculate_midline;

#[derive(Component)]
pub struct PlayerCar {
    pub current_line: i32,
}

impl PlayerCar {
    pub fn new() -> Self {
        Self { current_line: calculate_midline() }
    }

    pub fn try_move_up(&mut self) {
        self.current_line = min(NUMBER_OF_TRACKS - 1, self.current_line + 1);
    }

    pub fn try_move_down(&mut self) {
        self.current_line = max(0, self.current_line - 1);
    }
}

#[derive(Component)]
pub struct EnemyCar {}

impl EnemyCar {
    pub fn new() -> Self {
        Self { }
    }
}

#[derive(Resource)]
pub struct GameState {}

impl GameState {
    pub fn new() -> Self {
        Self {}
    }
}
