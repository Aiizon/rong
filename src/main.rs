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

#[derive(Component)]
struct Paddle {
    move_up:   KeyCode,
    move_down: KeyCode
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}