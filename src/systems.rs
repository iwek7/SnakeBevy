use bevy::app::AppExit;
use bevy::prelude::{Camera2d, Commands, EventWriter};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn quit_game(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit::Success);
}
