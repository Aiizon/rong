use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use crate::{GameEvents, GameState};

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Player {
    Player1,
    Player2
}

impl Player {
    pub(crate) fn start_speed(&self) -> Velocity {
        match self {
            Player::Player1 => Velocity::linear(Vec2::new(400.0, 0.0)),
            Player::Player2 => Velocity::linear(Vec2::new(-400.0, 0.0))
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct Score {
    scores: HashMap<Player, i32>
}

pub(crate) fn add_point(
    mut game_events:     EventReader<GameEvents>,
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

pub(crate) fn check_winner(
    score: Res<Score>,
    mut game_events: EventWriter<GameEvents>
) {
    for (player, &points) in score.scores.iter() {
        if points == 11 {
            game_events.write(GameEvents::PlayerWin(*player));
        }
    }
}

#[derive(Component)]
pub(crate) struct WinScreen;

pub(crate) fn handle_win(
    mut game_events: EventReader<GameEvents>,
    mut commands:    Commands,
    mut game_state:  ResMut<GameState>
) {
    for game_event in game_events.read() {
        match game_event {
            GameEvents::PlayerWin(player) => {
                game_state.running = false;

                let winner_text = match player {
                    Player::Player1 => "Player 1",
                    Player::Player2 => "Player 2"
                }.to_string() + " wins!\nPress any key to restart";
                
                commands.spawn((
                    WinScreen,
                    Transform::from_xyz(0.0, 0.0, 11.0),
                    TextColor(Color::WHITE),
                    Node {
                        position_type:   PositionType::Absolute,
                        display:         Display::Flex,
                        flex_direction:  FlexDirection::Row,
                        align_items:     AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        margin:          UiRect::horizontal(Val::Auto),
                        padding:         UiRect::all(Val::Px(40.0)),
                        width:           Val::Percent(50.0),
                        height:          Val::Percent(20.0),
                        top:             Val::Percent(40.0),
                        right:           Val::Percent(25.0),
                        ..default()
                    },
                    BackgroundColor::from(Srgba::new(0.1, 0.1, 0.1, 0.5))
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new(winner_text),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            },
            _ => {}
        }
    }
}

pub(crate) fn handle_restart(
    input:                Res<ButtonInput<KeyCode>>,
    win_screens:          Query<Entity, With<WinScreen>>,
    mut game_state:       ResMut<GameState>,
    mut score:            ResMut<Score>,
    mut scoreboard_texts: Query<(&mut Text, &Player)>,
    mut event_writer:     EventWriter<GameEvents>,
    mut commands:         Commands
) {
    if !input.get_just_pressed().next().is_some() {
        return;
    }

    game_state.running = true;

    score.scores.clear();

    for (mut text, _) in &mut scoreboard_texts {
        text.0 = "0".to_string();
    }

    for entity in &win_screens {
        commands.entity(entity).despawn();
    }

    let player = if rand::rng().random_bool(0.5) {
        Player::Player1
    } else {
        Player::Player2
    };

    event_writer.write(GameEvents::ResetBall(player));
}
