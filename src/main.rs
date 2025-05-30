mod paddle;
mod ball;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera, 
            paddle::spawn_players,
            ball::spawn_ball
        ))
        .add_systems(Update, (
            paddle::move_paddles,
            ball::move_ball
        ))
        .run()
    ;
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}