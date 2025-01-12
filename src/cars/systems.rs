use crate::cars::components::{GameState, PlayerCar};
use crate::cars::config::*;
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::*;
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

pub fn setup_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let total_height = (TRACK_WIDTH * NUMBER_OF_TRACKS as f32)
        + ((NUMBER_OF_TRACKS as f32 - 1.) * STRIPE_SIZE.y);
    let road_shape = meshes.add(Rectangle::new(
        TRACK_SHOWN_LENGTH,
        total_height,
    ));

    commands.spawn((
        Mesh2d(road_shape),
        MeshMaterial2d(materials.add(ROAD_COLOR)),
        Transform::from_xyz(0.0, 0.0, ROAD_Z),
    ));

    let mut already_drawn_stripe_lines = 0;
    for stripe_line_idx in 0..NUMBER_OF_TRACKS - 1 {
        let mut current_x = -TRACK_SHOWN_LENGTH / 2.;
        let line_under_stripe_offset = -total_height / 2. + stripe_line_idx as f32 * TRACK_WIDTH + already_drawn_stripe_lines as f32 * STRIPE_SIZE.y + TRACK_WIDTH;
        println!("{}", line_under_stripe_offset);
        let stripe_y = line_under_stripe_offset;
        while current_x < TRACK_SHOWN_LENGTH / 2. {
            // todo: cache it
            let stripe_shape = meshes.add(Rectangle::from_size(STRIPE_SIZE));
            let stripe_transform = Transform::from_xyz(current_x, stripe_y, STRIPE_Z);
            commands.spawn((
                Mesh2d(stripe_shape),
                MeshMaterial2d(materials.add(STRIPE_COLOR)),
                stripe_transform,
            ));
            current_x += STRIPE_SIZE.x + STRIPE_GAP;
        }
        already_drawn_stripe_lines += 1;
    }
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

pub fn update_player_car_position(mut query: Query<(&PlayerCar, &mut Transform)>) {
    for (player_car, mut transform) in query.iter_mut() {
        let mid_line = calculate_midline();
        let line_offset_from_middle = player_car.current_line - mid_line;
        transform.translation.y =  line_offset_from_middle as f32 * (TRACK_WIDTH + STRIPE_SIZE.y);
    }
}

pub fn calculate_midline() -> i32 {
    NUMBER_OF_TRACKS / 2
}
