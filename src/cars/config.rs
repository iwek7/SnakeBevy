use bevy::color::Color;
use bevy::prelude::{vec2, Vec2};

pub const NUMBER_OF_TRACKS: i32 = 3;
pub const TRACK_WIDTH: f32 = 200.;
pub const TRACK_SHOWN_LENGTH: f32 = 1000.;

pub const BACKGROUND_Z: f32 = 0.;
pub const BACKGROUND_COLOR: Color = Color::srgba(169. / 255., 169. / 255., 169. / 255., 1.0);

pub const CAR_Z: f32 = 1.;
// ratio should be kept as 2:1
pub const CAR_SIZE: Vec2 = vec2(400., 200.);

