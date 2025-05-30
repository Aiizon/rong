use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::GameEvents;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Resource, Default)]
pub(crate) struct Score {
    scores: HashMap<Player, i32>
}

pub(crate) fn add_point(
    mut game_events:      EventReader<GameEvents>,
    mut scoreboard_texts: Query<(&mut Text, &Player)>,
    mut score:            ResMut<Score>
) {
    for game_event in game_events.read() {
        match game_event {
            GameEvents::AddPoint(player) => {
                *score.scores.entry(*player).or_default() += 1;
                let score = score.scores.get(player).cloned().unwrap_or_default();

                for (mut text, owner) in &mut scoreboard_texts {
                    if owner != player { 
                        continue;
                    }
                    
                    text.0 = score.to_string();
                    break;
                }
            }
            _ => {}
        }
    }
}