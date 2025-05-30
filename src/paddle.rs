use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::time::Time;
use bevy_rapier2d::prelude::*;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const PADDLE_WIDTH: f32 = 10.0;
pub const PADDLE_HEIGHT: f32 = 100.0;
pub const PADDLE_SIZE: Vec2 = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);

pub enum PaddleLocation {
    Left,
    Right
}

#[derive(Component)]
pub struct Paddle {
    move_up:   KeyCode,
    move_down: KeyCode
}

impl Paddle {
    pub fn new(loc: PaddleLocation, move_up: KeyCode, move_down: KeyCode) -> (Sprite, Transform, Paddle, RigidBody, Collider) {
        (
            Sprite {
                color: Color::WHITE,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            Transform {
                translation: match loc {
                    PaddleLocation::Left => {
                        Vec3::new(-(WINDOW_WIDTH / 2.0 - 15.0), 0.0, 1.0)
                    }
                    PaddleLocation::Right => {
                        Vec3::new(WINDOW_WIDTH / 2.0 - 15.0, 0.0, 1.0)
                    }
                },
                ..default()
            },
            Paddle {
                move_up,
                move_down,
            },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0)
        )
    }
}


pub(crate) fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        Transform::default(),
    ));

    commands.spawn_batch([
        Paddle::new(PaddleLocation::Left, KeyCode::KeyW, KeyCode::KeyS),
        Paddle::new(PaddleLocation::Right, KeyCode::ArrowUp, KeyCode::ArrowDown),
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