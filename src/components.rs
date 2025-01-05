use bevy::math::Vec2;
use bevy::prelude::*;
use crate::{SNAKE_MOVE_TIMEOUT, SNAKE_Z};

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Component)]
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
    pub(crate) index: i32,
    pub(crate) segment_in_front: Option<Entity>
}

impl SnakeSegment {
    pub fn new(index: i32, segment_in_front: Entity) -> Self {
        Self {
            index,
            segment_in_front: Some(segment_in_front)
        }
    }
    pub fn new_head() -> Self {
        Self {
            segment_in_front: None,
            index: 0,
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