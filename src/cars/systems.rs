use crate::cars::config::*;
use crate::cars::PlayerCar;
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::*;

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
