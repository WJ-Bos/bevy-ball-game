use ::bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SIZE:f32 = 64.0;
pub const PLAYER_SPEED:f32 = 750.0;
pub const NUM_ENEMIES:usize = 6;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera,spawn_player))
        .add_systems(Update,(move_player,confine_player_bounds))
        .run();
}

#[derive(Component, Debug)]
pub struct Player {}
#[derive(Component,Debug)]
pub struct Enemy{}

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
        
    }
}

pub fn setup_camera(mut commands: Commands, window_q: Query<&Window, With<PrimaryWindow>>) {
    let window = window_q.single().unwrap();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>, //Keyboard Resource
    mut player_q : Query<&mut Transform,With<Player>>, //Players With Transform Comp
    time: Res<Time> //Time Resource
){
    //Check to see if Player Exists
    if let Ok(mut transform) = player_q.single_mut(){
        //3D vector for Marking Player Position
        let mut direction = Vec3::ZERO;

        //Translate Player based on key press
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA){
            direction += Vec3::new(-1.0,0.0,0.0);
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD){
            direction += Vec3::new(1.0,0.0,0.0);
        }
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW){
            direction += Vec3::new(0.0,1.0,0.0);
        }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS){
            direction += Vec3::new(0.0,-1.0,0.0);
        }

        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_bounds(
    mut player_q: Query<&mut Transform, With<Player>>,
    window: Query<&Window,With<PrimaryWindow>>
){
    if let Ok(mut transform) = player_q.single_mut(){
        let window = window.single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = transform.translation;
        if translation.x < x_min{
            translation.x = x_min
        }else if translation.x > x_max{
            translation.x = x_max;
        }

        if translation.y < y_min{
            translation.y = y_min
        }else if translation.y > y_max{
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}