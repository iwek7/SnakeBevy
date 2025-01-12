use bevy::asset::{AssetServer, Assets};
use bevy::prelude::{default, ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, Rectangle, Res, ResMut, Sprite, Transform};
use crate::cars::config::{BACKGROUND_COLOR, BACKGROUND_Z, CAR_Z, NUMBER_OF_TRACKS, TRACK_SHOWN_LENGTH, TRACK_WIDTH};
use crate::cars::PlayerCar;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_left = asset_server.load(
        "blueCar/blueCar.png",
    );
    commands.spawn((
        PlayerCar::new(),
        Transform::from_xyz(0.0, 0.0, CAR_Z),
        Sprite {
            image: texture_left,
            flip_x: true,
            ..default()
        }
    ));
}

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape_mesh = meshes.add(Rectangle::new(
        TRACK_WIDTH * NUMBER_OF_TRACKS as f32,
        TRACK_SHOWN_LENGTH
    ));

    commands.spawn((
        Mesh2d(shape_mesh),
        MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
        Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
    ));
}