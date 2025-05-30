mod paddle;
mod ball;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;

pub const WINDOW_WIDTH:  f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            title: "Rong".into(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            ..default()
        }),
        ..default()
    }));
    
    app.add_plugins(RapierPhysicsPlugin::<()>::default());
    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());
    
    app.add_systems(Startup, (
        spawn_camera, 
        paddle::spawn_players,
        ball::spawn_ball
    ));
    
    app.add_systems(Update, (
        paddle::move_paddles,
        ball::move_ball,
        ball::collide_ball
    ));
    
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}