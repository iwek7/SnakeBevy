use std::time::Duration;
use bevy::color::Color;
use bevy::prelude::{vec2, Vec2};


pub const DRAW_GIZMOS: bool = true;

pub const NUMBER_OF_LINES: i32 = 3;
// todo: refactor to vec2
pub const LINE_WIDTH: f32 = 1080. / 2. / NUMBER_OF_LINES as f32;
pub const LINE_LENGTH: f32 = 1980.;

pub const ROAD_Z: f32 = 0.;
pub const ROAD_COLOR: Color = Color::srgba(169. / 255., 169. / 255., 169. / 255., 1.0);

pub const STRIPE_Z: f32 = 0.1;
pub const STRIPE_SIZE: Vec2 = vec2(50., 20.);
pub const STRIPE_GAP: f32 = 30.;
pub const STRIPE_COLOR: Color = Color::srgba(1., 1., 1., 1.0);

pub const CAR_Z: f32 = 1.;
// ratio should be kept as 2:1
pub const CAR_SIZE: Vec2 = vec2(400., 200.);


pub const ENEMY_CAR_SPEED: f32 = 4.;

pub const SPAWN_TIMEOUT: Duration = Duration::from_millis(3000);