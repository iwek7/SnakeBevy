use crate::cars::systems::{move_player_car, setup_road, setup_game_state, setup_player, update_player_car_position, spawn_enemy_car, move_enemy_cars};
use crate::systems::{print_mouse_position, quit_game, setup_camera};
use bevy::app::{App, Startup, Update};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::DefaultPlugins;
pub mod cars;
mod components;
mod config;
mod systems;

pub fn launch_cars() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::new(1920.0, 1080.0), // Set your desired resolution
                mode: WindowMode::Fullscreen(MonitorSelection::Primary), // Enable fullscreen mode
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_player,
                setup_road,
                setup_game_state,
                spawn_enemy_car
            ),
        )
        .add_systems(
            Update,
            (
                quit_game.run_if(input_just_pressed(KeyCode::Escape)),
                move_player_car,
                update_player_car_position.after(move_player_car),
                move_enemy_cars
                // print_mouse_position
            ),
        )
        .run();
}
