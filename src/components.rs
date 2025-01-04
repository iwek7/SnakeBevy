use bevy::math::Vec2;
use bevy::prelude::*;
use crate::{MOVE_TIMEOUT, SNAKE_Z};

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
            move_timer: Timer::new(MOVE_TIMEOUT, TimerMode::Repeating)
        }
    }

}

#[derive(Component)]
pub struct SnakeSegment {}

impl SnakeSegment {
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Bundle)]
pub struct SegmentBundle {
    marker: SnakeSegment,
    mesh: Mesh2d,
    transform: Transform,
    material: MeshMaterial2d<ColorMaterial>,
}

impl SegmentBundle {
    pub fn from_single_segment(
        mesh: Mesh2d,
        material: MeshMaterial2d<ColorMaterial>,
        segment_pos: Vec2,
    ) -> SegmentBundle {
        SegmentBundle {
            marker: SnakeSegment::new(),
            mesh,
            material,
            transform: Transform::from_xyz(segment_pos.x, segment_pos.y, SNAKE_Z),
        }
    }
}
