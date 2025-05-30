mod paddle;
mod ball;

use bevy::prelude::*;
use bevy::window::WindowResolution;

const WINDOW_WIDTH:  f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                title: "Rong".into(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (
            spawn_camera, 
            paddle::spawn_players,
            ball::spawn_ball
        ))
        .add_systems(Update, (
            paddle::move_paddles,
            ball::move_ball,
            ball::collide_ball
        ))
        .run()
    ;
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}