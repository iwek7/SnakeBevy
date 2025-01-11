use bevy::prelude::*;
use std::time::Duration;

// palette https://html-color.codes/grey
pub const GRID_SIZE: [i32; 2] = [15, 15];

pub const CELL_SIZE: f32 = 50.0;

pub const DOT_RADIUS: f32 = 1.5;
pub const DOTS_Z: f32 = 0.1;
pub const DOTS_COLOR: Color = Color::srgba(128. / 255., 128. / 255., 128. / 255., 0.5);

pub const BACKGROUND_Z: f32 = 0.0;
pub const BACKGROUND_COLOR: Color = Color::srgba(169. / 255., 169. / 255., 169. / 255., 1.0);

pub const SNAKE_Z: f32 = 1.0;
pub const SNAKE_COLOR: Color = Color::srgba(85. / 255., 85. / 255., 85. / 255., 1.0);
pub const SNAKE_HEAD_COLOR: Color =  Color::srgba(65. / 255., 205. / 255., 225. / 255., 1.0);
pub const SNAKE_SIZE: f32 = CELL_SIZE * 0.8;
pub const SNAKE_MOVE_TIMEOUT: Duration = Duration::from_millis(200);

pub const FOOD_COLOR: Color = Color::srgba(4. / 255., 12. / 255., 239. / 255., 1.0);
pub const FOOD_RADIUS: f32 = CELL_SIZE * 0.4;
pub const FOOD_Z: f32 = 0.2;