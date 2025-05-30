use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::time::Time;

const PADDLE_SIZE: Vec2 = Vec2::new(10.0, 100.0);

#[derive(Component)]
pub struct Paddle {
    move_up:   KeyCode,
    move_down: KeyCode
}


pub(crate) fn spawn_players(
    mut commands: Commands, 
    query:        Query<&Window>
) {
    let window = query.single().expect("No primary window found");
    let width = window.width() / 2.0;

    commands.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(width * 2.0, window.height())),
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
    windows:     Query<&Window>,
    time:        Res<Time>
) {
    let window = windows.single().expect("No primary window found");
    let height = window.height() / 2.0 - PADDLE_SIZE.y / 2.0;

    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) { 
            pos.translation.y += 160.0 * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(-height, height);
        }
        if input.pressed(settings.move_down) {
            pos.translation.y -= 160.0 * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(-height, height);
        }
    }
}