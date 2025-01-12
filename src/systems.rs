use bevy::app::AppExit;
use bevy::prelude::{Camera2d, Commands, CursorMoved, EventReader, EventWriter};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn quit_game(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit::Success);
}

pub fn print_mouse_position(mut cursor_moved_events: EventReader<CursorMoved>) {
    // 2) Print out the mouse position each time it moves
    for event in cursor_moved_events.read() {
        println!("Mouse position: {:?}", event.position);
    }
}