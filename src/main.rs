mod paddle;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera, 
            paddle::spawn_players
        ))
        .add_systems(Update, paddle::move_paddles)
        .run()
    ;
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}