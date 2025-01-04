mod components;

use std::time::Duration;
use bevy::app::App;
use bevy::color::palettes::basic::{BLACK, FUCHSIA, RED};
use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};
use bevy::math::vec2;

use bevy::input::common_conditions::*;
use bevy::prelude::*;
use crate::components::{GlobalGameState, Direction, SegmentBundle, SnakeSegment};

const GRID_SIZE: [i32; 2] = [21, 21];
const CELL_SIZE: f32 = 50.0;

const BACKGROUND_Z: f32 = 0.0;
const SNAKE_Z: f32 = 1.0;

const SNAKE_SRGBA: Srgba = FUCHSIA;
const SNAKE_COLOR: Color = Color::srgba(FUCHSIA.red, FUCHSIA.green, FUCHSIA.blue, FUCHSIA.alpha);
const BACKGROUND_COLOR: Color = Color::srgba(220. / 255., 220. / 255., 220. / 255., 1.0);

pub const MOVE_TIMEOUT: Duration = Duration::from_millis(500);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_snake, setup_camera))
        .add_systems(
            Update,
            (
                draw_gizmos,
                move_snecko,
                handle_turn_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                handle_turn_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
                handle_turn_left.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                handle_turn_right.run_if(input_just_pressed(KeyCode::ArrowRight)),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawning background
    let shape_mesh = meshes.add(Rectangle::new(
        GRID_SIZE[0] as f32 * CELL_SIZE,
        GRID_SIZE[1] as f32 * CELL_SIZE,
    ));

    commands.spawn((
        Mesh2d(shape_mesh),
        MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
        Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
    ));

    // spawning snake
    let shape_mesh = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));

    let snake = SegmentBundle::from_single_segment(
        Mesh2d(shape_mesh),
        MeshMaterial2d(materials.add(SNAKE_COLOR)),
        vec2(0.0, 0.0),
    );

    commands.spawn(snake);
    
    // spawning current direction
    commands.spawn((GlobalGameState::new(Direction::RIGHT),));
}


fn move_snecko(
    time: Res<Time>,
    mut snake_segments_q: Query<&mut Transform, With<SnakeSegment>>,
    // we don't want mutable access to current direction but we get it anyway :(
    mut global_game_state_q: Query<&mut GlobalGameState, With<GlobalGameState>>,
) {
    let mut global_state = global_game_state_q.get_single_mut().unwrap();

    global_state.move_timer.tick(time.delta());
    if(global_state.move_timer.finished()) {
        let mut segment = snake_segments_q.get_single_mut().unwrap();
        let current_direction = &global_state.direction;
        
        let new_position  = match &current_direction {
            Direction::UP => Vec2::new(segment.translation.x, segment.translation.y + CELL_SIZE),
            Direction::DOWN => Vec2::new(segment.translation.x, segment.translation.y - CELL_SIZE),
            Direction::LEFT => Vec2::new(segment.translation.x - CELL_SIZE, segment.translation.y),
            Direction::RIGHT => Vec2::new(segment.translation.x + CELL_SIZE, segment.translation.y),
        };
        // check if inside the map
        let half_grid_size_x = GRID_SIZE[0] as f32 / 2.0 * CELL_SIZE;
        let half_grid_size_y = GRID_SIZE[1] as f32 / 2.0 * CELL_SIZE;
        if(new_position.x > -half_grid_size_x  && new_position.x < half_grid_size_x && new_position.y > -half_grid_size_y && new_position.y <half_grid_size_y) {
            segment.translation.x = new_position.x;
            segment.translation.y = new_position.y;
        }
        global_state.move_timer.reset();
    }
}


// todo: those separate methods are overkill
fn handle_turn_up(mut global_game_state_q: Query<&mut GlobalGameState, With<GlobalGameState>>) {
    global_game_state_q.get_single_mut().unwrap().direction = Direction::UP;
}

fn handle_turn_down(mut global_game_state_q: Query<&mut GlobalGameState, With<GlobalGameState>>) {
    global_game_state_q.get_single_mut().unwrap().direction = Direction::DOWN;

}

fn handle_turn_left(mut global_game_state_q: Query<&mut GlobalGameState, With<GlobalGameState>>) {
    global_game_state_q.get_single_mut().unwrap().direction = Direction::LEFT;

}

fn handle_turn_right(mut global_game_state_q: Query<&mut GlobalGameState, With<GlobalGameState>>) {
    global_game_state_q.get_single_mut().unwrap().direction = Direction::RIGHT;
}

fn draw_gizmos(mut gizmos: Gizmos) {
    let grid_root = [
        -(GRID_SIZE[0] as f32 / 2.0) * CELL_SIZE,
        -(GRID_SIZE[1] as f32 / 2.0) * CELL_SIZE,
    ];

    let grid_end = [-grid_root[0], -grid_root[1]];

    for row in 0..GRID_SIZE[0] {
        let row_y = grid_root[0] + row as f32 * CELL_SIZE;
        gizmos.line_2d(vec2(grid_root[0], row_y), vec2(grid_end[0], row_y), BLACK);
        for col in 0..GRID_SIZE[1] {
            let col_x = grid_root[1] + col as f32 * CELL_SIZE;
            gizmos.line_2d(vec2(col_x, grid_root[1]), vec2(col_x, grid_end[1]), BLACK);
        }
    }
}
