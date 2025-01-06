use bevy::math::Vec2;
use bevy::prelude::*;
use crate::config::*;

#[derive(PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Resource)]
pub struct GlobalGameState {
    pub direction: Direction,
    pub move_timer: Timer
}

impl GlobalGameState {
    pub fn new(direction: Direction) -> Self {
        Self { 
            direction,
            move_timer: Timer::new(SNAKE_MOVE_TIMEOUT, TimerMode::Repeating)
        }
    }
}

#[derive(Component)]
pub struct SnakeSegment {
    pub index: i32,
    // this flag is used to delay movement of segment by n move ticks
    // so that spawn happens at the end of the snake
    pub move_delay: i32
}

impl SnakeSegment {
    pub fn new(index: i32, move_delay: i32) -> Self {
        Self {
            index,
            move_delay
        }
    }
}
#[derive(Bundle)]
pub struct SnakeSegmentBundle {
    snake_segment: SnakeSegment,
    mesh: Mesh2d,
    transform: Transform,
    material: MeshMaterial2d<ColorMaterial>,

}

impl SnakeSegmentBundle {
    pub fn new(
        mesh: Mesh2d,
        material: MeshMaterial2d<ColorMaterial>,
        segment_pos: Vec2,
        snake_segment: SnakeSegment
    ) -> SnakeSegmentBundle {
        SnakeSegmentBundle {
            snake_segment,
            mesh,
            material,
            transform: Transform::from_xyz(segment_pos.x, segment_pos.y, SNAKE_Z),
        }
    }
}

#[derive(Component)]
pub struct Food {}

impl Food {
    fn new() -> Self {
        Self {}
    }
}
#[derive(Bundle)]
pub struct FoodBundle {
    food: Food,
    mesh: Mesh2d,
    transform: Transform,
    material: MeshMaterial2d<ColorMaterial>,
}

impl FoodBundle {
    pub fn new(mesh: Mesh2d, material: MeshMaterial2d<ColorMaterial>, position: Vec2) -> Self {
        Self {
            food: Food::new(),
            mesh,
            material,
            transform: Transform::from_xyz(position.x, position.y, SNAKE_Z),
        }
    }
}