mod paddle;
mod ball;
mod border;
mod player;
mod scoreboard;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;
use crate::player::{Player, Score};

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
    
    app.init_resource::<Score>();
    app.insert_resource(GameState {running: true});
    
    app.add_event::<GameEvents>();
    
    app.add_systems(Startup, (
        setup,
        scoreboard::spawn_scoreboard,
        paddle::spawn_players,
        ball::spawn_ball,
        border::spawn_borders
    ));
    
    app.add_systems(Update, (
        paddle::move_paddles,
        ball::detect_reset,
        player::handle_win
    ).run_if(check_game_running));
    
    app.add_systems(Update, (player::handle_restart).run_if(check_game_stopped));
    
    app.add_systems(PostUpdate, (
        ball::reset_ball,
        (
            player::add_point,
            player::check_winner
        ).chain()
    ).run_if(check_game_running));
    
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

#[derive(Resource, Default)]
pub(crate) struct GameState {
    running: bool
}

pub(crate) fn check_game_running(
    game_state: Res<GameState>
) -> bool {
    game_state.running
}

pub(crate) fn check_game_stopped(
    game_state: Res<GameState>
) -> bool {
    !game_state.running
}

#[derive(Event)]
enum GameEvents {
    ResetBall(Player),
    AddPoint(Player),
    PlayerWin(Player)
}