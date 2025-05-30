use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Copy, Clone)]
pub(crate) enum Player {
    Player1,
    Player2
}

impl Player {
    pub(crate) fn start_speed(&self) -> Velocity {
        match self {
            Player::Player1 => Velocity::linear(Vec2::new(250.0, 0.0)),
            Player::Player2 => Velocity::linear(Vec2::new(-250.0, 0.0))
        }
    }
}