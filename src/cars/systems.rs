use crate::cars::components::{EnemyCar, EnemySpawnTimer, GameLostEvent, GameState, PlayerCar};
use crate::cars::config::*;
use bevy::asset::{AssetServer, Assets};
use bevy::color::palettes::css::GREEN;
use bevy::prelude::*;
use rand::Rng;

pub fn setup_game_state(mut commands: Commands) {
    commands.insert_resource(GameState::new());
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tx = asset_server.load("cars/blueCar/blueCar.png");
    commands.spawn((
        PlayerCar::new(),
        Transform::from_xyz(-800.0, 0.0, CAR_Z),
        Sprite {
            image: tx,
            flip_x: true,
            custom_size: Some(CAR_SIZE),
            ..default()
        },
    ));
}

pub fn setup_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(EnemySpawnTimer::new());
    spawn_enemy_car(&mut commands, asset_server);
}

fn spawn_enemy_car(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let spawn_line = rand::thread_rng().gen_range(0..NUMBER_OF_LINES);
    let y = car_at_line_y_position(spawn_line);
    let texture_left = asset_server.load("cars/redCar/redCar.png");
    commands.spawn((
        EnemyCar::new(spawn_line),
        Transform::from_xyz(400.0, y, CAR_Z),
        Sprite {
            image: texture_left,
            flip_x: true,
            custom_size: Some(CAR_SIZE),
            ..default()
        },
    ));
}

pub fn setup_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let total_height =
        (LINE_WIDTH * NUMBER_OF_LINES as f32) + ((NUMBER_OF_LINES as f32 - 1.) * STRIPE_SIZE.y);
    let road_shape = meshes.add(Rectangle::new(LINE_LENGTH, total_height));

    commands.spawn((
        Mesh2d(road_shape),
        MeshMaterial2d(materials.add(ROAD_COLOR)),
        Transform::from_xyz(0.0, 0.0, ROAD_Z),
    ));

    let mut already_drawn_stripe_lines = 0;
    for stripe_line_idx in 0..NUMBER_OF_LINES - 1 {
        let mut current_x = -LINE_LENGTH / 2. + STRIPE_SIZE.x / 2. + STRIPE_GAP;
        let line_under_stripe_offset = -total_height / 2.
            + stripe_line_idx as f32 * LINE_WIDTH
            + already_drawn_stripe_lines as f32 * STRIPE_SIZE.y
            + LINE_WIDTH;
        let stripe_y = line_under_stripe_offset;
        while current_x < LINE_LENGTH / 2. {
            // todo: cache it
            let stripe_shape = meshes.add(Rectangle::from_size(STRIPE_SIZE));
            let stripe_transform = Transform::from_xyz(current_x, stripe_y, STRIPE_Z);
            commands.spawn((
                Mesh2d(stripe_shape),
                MeshMaterial2d(materials.add(STRIPE_COLOR)),
                stripe_transform,
            ));
            current_x += STRIPE_SIZE.x + STRIPE_GAP;
        }
        already_drawn_stripe_lines += 1;
    }
}

pub fn try_spawning_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        spawn_enemy_car(&mut commands, asset_server);
        timer.timer.reset()
    }
}

pub fn move_player_car(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerCar>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        query.get_single_mut().unwrap().try_move_up();
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        query.get_single_mut().unwrap().try_move_down();
    }
}

pub fn update_player_car_position(mut query: Query<(&PlayerCar, &mut Transform)>) {
    for (player_car, mut transform) in query.iter_mut() {
        transform.translation.y = car_at_line_y_position(player_car.current_line);
    }
}

fn car_at_line_y_position(line_idx: i32) -> f32 {
    let mid_line = calculate_midline();
    let line_offset_from_middle = line_idx - mid_line;
    line_offset_from_middle as f32 * (LINE_WIDTH + STRIPE_SIZE.y)
}

pub fn move_enemy_cars(mut query: Query<&mut Transform, With<EnemyCar>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = transform.translation.x - ENEMY_CAR_SPEED;
    }
}

pub fn despawn_enemy_cars(
    query: Query<(Entity, &Transform), With<EnemyCar>>,
    mut commands: Commands,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x + CAR_SIZE.x / 2. < -LINE_LENGTH / 2. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn calculate_midline() -> i32 {
    NUMBER_OF_LINES / 2
}

pub fn draw_gizmos(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<PlayerCar>>,
    enemies_query: Query<&Transform, With<EnemyCar>>,
) {
    if !DRAW_GIZMOS {
        return;
    }

    for player_transform in player_query.iter() {
        gizmos.rect_2d(
            Isometry2d::from_translation(player_transform.translation.truncate()),
            CAR_SIZE,
            GREEN,
        );
    }

    for enemy_transform in enemies_query.iter() {
        gizmos.rect_2d(
            Isometry2d::from_translation(enemy_transform.translation.truncate()),
            CAR_SIZE,
            GREEN,
        );
    }
}

pub fn check_player_collission(
    player_query: Query<(&PlayerCar, &Transform), With<PlayerCar>>,
    enemies_query: Query<(&EnemyCar, &Transform), With<EnemyCar>>,
    mut lost_game_ew: EventWriter<GameLostEvent>,
) {
    let (player_car, player_transform) = player_query.single();
    // obviously we can do it simply with comparing only transform and without line, but w/e for now
    for (enemy_car, enemy_transform) in enemies_query.iter() {
        if player_car.current_line == enemy_car.current_line {
            if player_transform.translation.x < enemy_transform.translation.x + CAR_SIZE.x
                && player_transform.translation.x + CAR_SIZE.x > enemy_transform.translation.x
            {
               lost_game_ew.send(GameLostEvent::new());
                return;
            }
        }
    }
}

pub fn handle_game_lost_event(mut game_lost_er: EventReader<GameLostEvent>) {
    for _ in game_lost_er.read() {
      println!("LOSING!!!! {}", rand::random::<i32>());   
    }
}
