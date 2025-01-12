use bevy::app::{App, Startup, Update};
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::{default, Commands, Component, IntoSystemConfigs, KeyCode, MonitorSelection, PluginGroup, Res, Sprite, Transform, Window, WindowPlugin};
use bevy::window::WindowMode;
use crate::systems::{quit_game, setup_camera};

pub mod cars;

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
        .add_systems(Startup, (setup_camera, setup))
        .add_systems(
            Update,
            (quit_game.run_if(input_just_pressed(KeyCode::Escape))),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_left = asset_server.load(
        "blueCar/blueCar.png",
    );
    commands.spawn((
        PlayerCar::new(),
        Transform::from_xyz(100.0, 0.0, 0.0),
        Sprite {
            image: texture_left,
            flip_x: true,
            ..default()
        }
    ));
}

#[derive(Component)]
struct PlayerCar {}

impl PlayerCar {
    fn new() -> Self {
        Self {}
    }
}