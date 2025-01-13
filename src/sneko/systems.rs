use crate::sneko::components::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::sneko::components::{
    DespawnOnLoss, Direction, Food, FoodBundle, GameLostEvent, GlobalGameState, SnakeSegment,
    SnakeSegmentBundle,
};
use crate::sneko::config::*;
use bevy::asset::Assets;
use bevy::math::{vec2, Vec2};
use bevy::prelude::{Circle, ColorMaterial, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, KeyCode, Mesh, Mesh2d, MeshMaterial2d, Query, Rectangle, Res, ResMut, Time, Transform, With, Without};
use rand::Rng;
use std::ops::Div;
use bevy::input::ButtonInput;

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape_mesh = meshes.add(Rectangle::new(
        GRID_SIZE[0] as f32 * CELL_SIZE,
        GRID_SIZE[1] as f32 * CELL_SIZE,
    ));

    commands.spawn((
        Mesh2d(shape_mesh),
        MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
        Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
    ));
}

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
}

pub fn setup_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    actually_setup_snake(&mut commands, &mut meshes, &mut materials);
}

pub fn actually_setup_snake(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // spawning snake
    spawn_snake_segment(commands, meshes, materials, vec2(0.0, 0.0), 0);

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
    commands.insert_resource(GlobalGameState::new(RIGHT));
}

pub fn spawn_snake_segment(
    // todo meshes and materials need to go
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    index: i32,
) {
    let snake_mesh = Mesh2d(meshes.add(Rectangle::new(SNAKE_SIZE, SNAKE_SIZE)));
    let snake_material = MeshMaterial2d(materials.add(if index == 0 {
        SNAKE_HEAD_COLOR
    } else {
        SNAKE_COLOR
    }));

    let snake_full_position = if index == 0 {
        position.extend(SNAKE_HEAD_Z)
    } else {
        position.extend(SNAKE_Z)
    };

    let snake = SnakeSegmentBundle::new(
        snake_mesh,
        snake_material,
        snake_full_position,
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
                    (x as f32) * CELL_SIZE - (GRID_SIZE[0] as f32 * 0.5 * CELL_SIZE)
                        + (CELL_SIZE * 0.5),
                    (y as f32) * CELL_SIZE - GRID_SIZE[1] as f32 * 0.5 * CELL_SIZE
                        + (CELL_SIZE * 0.5),
                ));
            }
        }
    }

    // Return a random free position or None if there are no free cells.
    if free_positions.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let free_pos = free_positions[rng.gen_range(0..free_positions.len())];
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

fn calc_new_position(current_direction: &Direction, current_position: &Vec2) -> Vec2 {
    match &current_direction {
        UP => Vec2::new(current_position.x, current_position.y + CELL_SIZE),
        DOWN => Vec2::new(current_position.x, current_position.y - CELL_SIZE),
        LEFT => Vec2::new(current_position.x - CELL_SIZE, current_position.y),
        RIGHT => Vec2::new(current_position.x + CELL_SIZE, current_position.y),
    }
}

pub fn move_snecko(
    time: Res<Time>,
    mut snake_segments_q: Query<
        (&mut Transform, &mut SnakeSegment),
        (With<SnakeSegment>, Without<Food>),
    >,
    // we don't want mutable access to current direction but we get it anyway :(
    mut global_state: ResMut<GlobalGameState>,
    mut food_q: Query<&mut Transform, (With<Food>, Without<SnakeSegment>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_lost_ev_writer: EventWriter<GameLostEvent>,
) {
    global_state.move_timer.tick(time.delta());
    if global_state.move_timer.finished() { 
        let mut snake_segments_vec = snake_segments_q.iter_mut().collect::<Vec<_>>();
        // sorted from the back of the snecko
        snake_segments_vec.sort_by_key(|(_, segment)| -segment.index);

        let mut sneko_positions_for_collision_check = snake_segments_vec
            .iter()
            .map(|(t, _)| t.translation.truncate())
            .collect::<Vec<_>>();
        // so this is position of last segment before moving
        // so after whole snake moves it won't be occupied (unless by the head)
        sneko_positions_for_collision_check.remove(0);

        for i in 0..snake_segments_vec.len() {
            let (current_slice, next_slice) = snake_segments_vec.split_at_mut(i + 1);
            let (segment_transform, segment) = &mut current_slice[i];

            // move head
            if segment.index == 0 {
                let current_direction = &global_state.direction;
                let new_position =
                    calc_new_position(current_direction, &segment_transform.translation.truncate());
                if is_inside_map_bounds(new_position) {
                    segment_transform.translation.x = new_position.x;
                    segment_transform.translation.y = new_position.y;
                } else {
                    game_lost_ev_writer.send(GameLostEvent::new());
                    return;
                }

                for pos in sneko_positions_for_collision_check.iter() {
                    if pos == &new_position {
                        game_lost_ev_writer.send(GameLostEvent::new());
                        return;
                    }
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
                        snake_segments_vec
                            .iter()
                            .clone()
                            .map(|(t, _)| t.translation.truncate())
                            .collect::<Vec<_>>(),
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

pub fn handle_game_lost(
    mut commands: Commands,
    mut event_reader: EventReader<GameLostEvent>,
    query: Query<Entity, With<DespawnOnLoss>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in event_reader.read() {
        for e in query.iter() {
            commands.entity(e).despawn_recursive();
        }
        actually_setup_snake(&mut commands, &mut meshes, &mut materials);
    }
}

pub fn handle_turning(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    segments: Query<Entity, With<SnakeSegment>>,
    mut global_game_state: ResMut<GlobalGameState>
) {
    let num_segments = segments.iter().count();
    if keyboard_input.just_pressed(KeyCode::ArrowUp) && (num_segments == 1 || global_game_state.direction != DOWN) {
        global_game_state.direction = UP;
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) && (num_segments == 1 || global_game_state.direction != UP) {
        global_game_state.direction = DOWN;
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) && (num_segments == 1 || global_game_state.direction != RIGHT) {
        global_game_state.direction = LEFT;
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) && (num_segments == 1 || global_game_state.direction != LEFT) {
        global_game_state.direction = RIGHT;
    }
}
