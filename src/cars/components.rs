use crate::cars::config::{NUMBER_OF_LINES, SPAWN_TIMEOUT};
use crate::cars::systems::calculate_midline;
use bevy::prelude::*;
use std::cmp::{max, min};

#[derive(Component)]
pub struct PlayerCar {
    pub current_line: i32,
}

impl PlayerCar {
    pub fn new() -> Self {
        Self {
            current_line: calculate_midline(),
        }
    }

    pub fn try_move_up(&mut self) {
        self.current_line = min(NUMBER_OF_LINES - 1, self.current_line + 1);
    }

    pub fn try_move_down(&mut self) {
        self.current_line = max(0, self.current_line - 1);
    }
}

#[derive(Component)]
pub struct EnemyCar {
    pub current_line: i32,
}

impl EnemyCar {
    pub fn new(current_line: i32) -> Self {
        Self { current_line }
    }
}

#[derive(Resource)]
pub struct GameState {}

impl GameState {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl EnemySpawnTimer {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(SPAWN_TIMEOUT, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
pub struct GameLostEvent {}

impl GameLostEvent {
    pub fn new() -> Self {
        Self {}
    }
}
