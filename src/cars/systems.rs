use crate::cars::config::*;
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::*;
use crate::cars::components::{GameState, PlayerCar};
pub fn setup_game_state(mut commands: Commands) {
    commands.insert_resource(GameState::new());
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_left = asset_server.load("cars/blueCar/blueCar.png");
    commands.spawn((
        PlayerCar::new(),
        Transform::from_xyz(0.0, 0.0, CAR_Z),
        Sprite {
            image: texture_left,
            flip_x: true,
            custom_size: Some(CAR_SIZE),
            ..default()
        },
    ));
}

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape_mesh = meshes.add(Rectangle::new(
        TRACK_SHOWN_LENGTH,
        TRACK_WIDTH * NUMBER_OF_TRACKS as f32,
    ));

    commands.spawn((
        Mesh2d(shape_mesh),
        MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
        Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
    ));
}

pub fn move_player_car(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerCar>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        query.get_single_mut().unwrap().try_move_up();
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        query.get_single_mut().unwrap().try_move_down();
    }
}

pub fn update_player_car_position(
    mut query: Query<(&PlayerCar, &mut Transform)>,
) {
    for (player_car, mut transform) in query.iter_mut() {
        let mid_line = calculate_midline();
        let line_offset_from_middle = player_car.current_line - mid_line;
        transform.translation.y = line_offset_from_middle as f32 * TRACK_WIDTH;
    }
}

pub fn calculate_midline() -> i32 {
    NUMBER_OF_TRACKS / 2
}
