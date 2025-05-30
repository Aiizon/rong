mod paddle;
mod ball;
mod border;
mod player;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;
use crate::player::Player;

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
    
    app.add_event::<GameEvents>();
    
    app.add_systems(Startup, (
        setup,
        paddle::spawn_players,
        ball::spawn_ball,
        border::spawn_borders
    ));
    
    app.add_systems(Update, (
        paddle::move_paddles,
        ball::detect_reset
    ));
    
    app.add_systems(PostUpdate, ball::reset_ball);
    
    app.run();
}

fn setup(
    mut commands:   Commands,
    rapier_configs: Query<&mut RapierConfiguration>
) {
    commands.spawn(Camera2d::default());
    
    for mut config in rapier_configs {
        config.gravity = Vec2::ZERO;
    }
}

#[derive(Event)]
enum GameEvents {
    ResetBall(Player)
}