use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::time::Time;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const PADDLE_WIDTH: f32 = 10.0;
pub const PADDLE_HEIGHT: f32 = 100.0;
pub const PADDLE_SIZE: Vec2 = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);

#[derive(Component)]
pub struct Paddle {
    move_up:   KeyCode,
    move_down: KeyCode
}


pub(crate) fn spawn_players(
    mut commands: Commands
) {
    let width = WINDOW_WIDTH / 2.0;

    commands.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        Transform::default(),
    ));

    commands.spawn_batch([
        (
            Sprite {
                color: Color::WHITE,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            Transform {
                translation: Vec3::new(-(width - 15.0), 0.0, 0.0),
                ..default()
            },
            Paddle {
                move_up:   KeyCode::KeyW,
                move_down: KeyCode::KeyS,
            },
        ),
        (
            Sprite {
                color: Color::WHITE,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            Transform {
                translation: Vec3::new(width - 15.0, 0.0, 0.0),
                ..default()
            },
            Paddle {
                move_up:   KeyCode::ArrowUp,
                move_down: KeyCode::ArrowDown,
            },
        ),
    ]);
}

pub(crate) fn move_paddles(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input:       Res<ButtonInput<KeyCode>>,
    time:        Res<Time>
) {
    let height = WINDOW_HEIGHT / 2.0 - PADDLE_SIZE.y / 2.0;

    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) { 
            pos.translation.y += 200.0 * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(-height, height);
        }
        if input.pressed(settings.move_down) {
            pos.translation.y -= 200.0 * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(-height, height);
        }
    }
}