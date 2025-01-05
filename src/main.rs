mod components;

use bevy::app::App;
use bevy::color::palettes::basic::BLACK;
use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};
use bevy::math::vec2;
use std::ops::Div;
use std::time::Duration;

use crate::components::{
    Direction, Food, FoodBundle, GlobalGameState, SnakeSegment, SnakeSegmentBundle,
};
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use rand::Rng;

// palette https://html-color.codes/grey
const GRID_SIZE: [i32; 2] = [5, 5];

const CELL_SIZE: f32 = 50.0;

const DOT_RADIUS: f32 = 1.5;
const DOTS_Z: f32 = 0.1;
const DOTS_COLOR: Color = Color::srgba(128. / 255., 128. / 255., 128. / 255., 0.5);

const BACKGROUND_Z: f32 = 0.0;
const BACKGROUND_COLOR: Color = Color::srgba(169. / 255., 169. / 255., 169. / 255., 1.0);

const SNAKE_Z: f32 = 1.0;
const SNAKE_COLOR: Color = Color::srgba(85. / 255., 85. / 255., 85. / 255., 1.0);
const SNAKE_SIZE: f32 = CELL_SIZE * 0.8;
pub const SNAKE_MOVE_TIMEOUT: Duration = Duration::from_millis(200);

const FOOD_COLOR: Color = Color::srgba(4. / 255., 12. / 255., 239. / 255., 1.0);
const FOOD_RADIUS: f32 = CELL_SIZE * 0.4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_snake, setup_camera))
        .add_systems(
            Update,
            (
                // draw_gizmos,
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

    let dot_mesh = meshes.add(Circle::new(DOT_RADIUS));
    // dots
    for row in 0..GRID_SIZE[0] {
        for col in 0..GRID_SIZE[1] {
            commands.spawn((
                Mesh2d(dot_mesh.clone()),
                MeshMaterial2d(materials.add(DOTS_COLOR)),
                Transform::from_xyz(
                    row as f32 * CELL_SIZE - (GRID_SIZE[0] as f32 / 2. * CELL_SIZE),
                    col as f32 * CELL_SIZE - (GRID_SIZE[1] as f32 / 2. * CELL_SIZE),
                    DOTS_Z,
                ),
            ));
        }
    }

    // spawning snake
    spawn_snake_segment(
        &mut commands,
        &mut meshes,
        &mut materials,
        vec2(0.0, 0.0),
        0,
    );

    // spawn some food
    let food_mesh = Mesh2d(meshes.add(Circle::new(FOOD_RADIUS)));
    let food_material = MeshMaterial2d(materials.add(FOOD_COLOR));

    let food = FoodBundle::new(
        food_mesh,
        food_material,
        get_new_food_position(vec![vec2(0.0, 0.0)], None).unwrap(),
    );
    commands.spawn(food);

    // initializing game state
    commands.spawn((GlobalGameState::new(Direction::RIGHT),));
}

fn spawn_snake_segment(
    // todo meshes and materials need to go
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    index: i32,
) {
    let snake_mesh = Mesh2d(meshes.add(Rectangle::new(SNAKE_SIZE, SNAKE_SIZE)));
    let snake_material = MeshMaterial2d(materials.add(SNAKE_COLOR));

    let snake = SnakeSegmentBundle::new(
        snake_mesh,
        snake_material,
        position,
        SnakeSegment::new(index, index),
    );

    commands.spawn(snake);
}

// when false is returned there is no more place to spawn food
fn get_new_food_position(
    snake_segments_positions: Vec<Vec2>,
    current_food_position: Option<Vec2>,
) -> Option<Vec2> {
    let mut grid = [false; (GRID_SIZE[0] * GRID_SIZE[1]) as usize];

    // Mark the snake segments as occupied.
    for segment in snake_segments_positions {
        let x = ((segment.x + (GRID_SIZE[0] as f32 * 0.5 * CELL_SIZE)) / CELL_SIZE) as i32;
        let y = ((segment.y + (GRID_SIZE[1] as f32 * 0.5 * CELL_SIZE)) / CELL_SIZE) as i32;
        let index = y * GRID_SIZE[0] + x;
        grid[index as usize] = true;
    }

    // todo: handle no space left for food here
    if let Some(food_pos) = current_food_position {
        // Mark the current food position as occupied.
        let food_x = ((food_pos.x + (GRID_SIZE[0] as f32 * 0.5 * CELL_SIZE)) / CELL_SIZE) as i32;
        let food_y = ((food_pos.y + (GRID_SIZE[1] as f32 * 0.5 * CELL_SIZE)) / CELL_SIZE) as i32;
        let food_index = food_y * GRID_SIZE[0] + food_x;
        grid[food_index as usize] = true;
    }

    // Collect all free positions.
    let mut free_positions = Vec::new();
    for y in 0..GRID_SIZE[1] {
        for x in 0..GRID_SIZE[0] {
            let index = y * GRID_SIZE[0] + x;
            if !grid[index as usize] {
                free_positions.push(Vec2::new(
                    (x as f32) * CELL_SIZE - (GRID_SIZE[0] as f32 * 0.5 * CELL_SIZE) + (CELL_SIZE * 0.5),
                    (y as f32) * CELL_SIZE - GRID_SIZE[1] as f32 * 0.5 * CELL_SIZE + (CELL_SIZE * 0.5)));
            }
        }
    }

    // Return a random free position or None if there are no free cells.
    if free_positions.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let free_pos = free_positions[rng.gen_range(0..free_positions.len())];
        println!("Selected free position: {:?}", free_pos);
        Some(free_pos)
    }
}
fn is_inside_map_bounds(position: Vec2) -> bool {
    let half_grid_size_x = GRID_SIZE[0] as f32 / 2.0 * CELL_SIZE;
    let half_grid_size_y = GRID_SIZE[1] as f32 / 2.0 * CELL_SIZE;
    position.x > -half_grid_size_x
        && position.x < half_grid_size_x
        && position.y > -half_grid_size_y
        && position.y < half_grid_size_y
}

fn move_snecko(
    time: Res<Time>,
    mut snake_segments_q: Query<
        (&mut Transform, &mut SnakeSegment),
        (With<SnakeSegment>, Without<Food>),
    >,
    // we don't want mutable access to current direction but we get it anyway :(
    mut global_game_state_q: Query<&mut GlobalGameState, With<GlobalGameState>>,
    mut food_q: Query<&mut Transform, (With<Food>, Without<SnakeSegment>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut global_state = global_game_state_q.get_single_mut().unwrap();

    global_state.move_timer.tick(time.delta());
    if global_state.move_timer.finished() {
        let mut snake_segments_vec = snake_segments_q.iter_mut().collect::<Vec<_>>();
        // sorted from the back of the snecko
        snake_segments_vec.sort_by_key(|(_, segment)| -segment.index);

        for i in 0..snake_segments_vec.len() {
            let (current_slice, next_slice) = snake_segments_vec.split_at_mut(i + 1);
            let (segment_transform, segment) = &mut current_slice[i];

            // move head
            if segment.index == 0 {
                let current_direction = &global_state.direction;

                let new_position = match &current_direction {
                    Direction::UP => Vec2::new(
                        segment_transform.translation.x,
                        segment_transform.translation.y + CELL_SIZE,
                    ),
                    Direction::DOWN => Vec2::new(
                        segment_transform.translation.x,
                        segment_transform.translation.y - CELL_SIZE,
                    ),
                    Direction::LEFT => Vec2::new(
                        segment_transform.translation.x - CELL_SIZE,
                        segment_transform.translation.y,
                    ),
                    Direction::RIGHT => Vec2::new(
                        segment_transform.translation.x + CELL_SIZE,
                        segment_transform.translation.y,
                    ),
                };

                if is_inside_map_bounds(new_position) {
                    segment_transform.translation.x = new_position.x;
                    segment_transform.translation.y = new_position.y;
                }

                let mut food_transform = food_q.get_single_mut().unwrap();
                let food_grid_cell = food_transform.translation.div(CELL_SIZE).floor().truncate();
                let snake_head_grid_cell = segment_transform
                    .translation
                    .div(CELL_SIZE)
                    .floor()
                    .truncate();
                if food_grid_cell == snake_head_grid_cell {
                    spawn_snake_segment(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        segment_transform.translation.truncate(),
                        snake_segments_vec.len() as i32,
                    );
                    let new_food_position = get_new_food_position(
                        snake_segments_vec.iter().clone().map(|(t, _)| t.translation.truncate()).collect::<Vec<_>>(),
                        Some(food_transform.translation.truncate()),
                    )
                        .unwrap();
                    food_transform.translation.x = new_food_position.x;
                    food_transform.translation.y = new_food_position.y;
                }

                // check if food is consumed
                global_state.move_timer.reset();
            } else {
                if segment.move_delay > 0 {
                    segment.move_delay -= 1;
                } else if let Some((next_transform, _)) = next_slice.first() {
                    segment_transform.translation.x = next_transform.translation.x;
                    segment_transform.translation.y = next_transform.translation.y;
                }
            }
        }
    }
}

// todo: those separate methods are overkill
// todo: prevent turning around in opposite direction
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
