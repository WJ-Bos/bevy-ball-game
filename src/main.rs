use ::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUM_ENEMIES: usize = 5;
pub const ENEMY_SPEED: f32 = 250.0;
pub const ENEMY_SIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_player, spawn_enemies))
        .add_systems(Update, (player_movement, confine_player_bounds,enemy_movement,update_enemy_movement))
        .run();
}

#[derive(Component, Debug)]
pub struct Player {}
#[derive(Component, Debug)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    for _ in 0..NUM_ENEMIES {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(rand_x, rand_y, 0.0),
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
            },
        ));
    }
}

pub fn setup_camera(mut commands: Commands, window_q: Query<&Window, With<PrimaryWindow>>) {
    let window = window_q.single().unwrap();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>, //Keyboard Resource
    mut player_q: Query<&mut Transform, With<Player>>, //Players With Transform Comp
    time: Res<Time>, //Time Resource
) {
    //Check to see if Player Exists
    if let Ok(mut transform) = player_q.single_mut() {
        //3D vector for Marking Player Position
        let mut direction = Vec3::ZERO;

        //Translate Player based on key press
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn enemy_movement(
    mut enemy_q : Query<(&mut Transform,&Enemy)>,
    time: Res<Time>
){
    for(mut transform, enemy) in enemy_q.iter_mut(){
        let direction = Vec3::new(enemy.direction.x,enemy.direction.y,0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn confine_player_bounds(
    mut player_q: Query<&mut Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_q.single_mut() {
        let window = window.single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = transform.translation;
        if translation.x < x_min {
            translation.x = x_min
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn update_enemy_movement(
    mut enemy_q : Query<(&Transform, &mut Enemy)>,
    window: Query<&Window, With<PrimaryWindow>>
){
    let window = window.single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for(transform, mut enemy) in enemy_q.iter_mut(){
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max{
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max{
            enemy.direction.y *= -1.0;
        }
    }
}