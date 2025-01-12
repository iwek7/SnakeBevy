use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::sneko::components::GameLostEvent;
use crate::sneko::systems::*;
use crate::systems::{quit_game, setup_camera};

mod sneko;
mod components;
mod config;
mod systems;

pub(crate) fn lounch_snake()  {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: bevy::window::WindowResolution::new(1920.0, 1080.0), // Set your desired resolution
            mode: WindowMode::Fullscreen(MonitorSelection::Primary), // Enable fullscreen mode
            ..Default::default()
        }),
        ..Default::default()
    }))
        .add_event::<GameLostEvent>()
        .add_systems(Startup, (setup_game, setup_camera, setup_snake, setup_background))
        .add_systems(
            Update,
            (
                // draw_gizmos,
                move_snecko,
                handle_turn_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                handle_turn_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
                handle_turn_left.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                handle_turn_right.run_if(input_just_pressed(KeyCode::ArrowRight)),
                quit_game.run_if(input_just_pressed(KeyCode::Escape)),
                handle_game_lost,
            ),
        ).run();
}

