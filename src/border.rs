use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::Player;

pub enum BorderLocation {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
pub struct Border;

impl Border {
    pub fn new_horizontal(loc: BorderLocation) -> (Sprite, Transform, Border, RigidBody, Collider) {
        (
            Sprite::from_color(Color::BLACK, Vec2::ONE),
            Transform {
                translation: match loc {
                    BorderLocation::Top => {
                        Vec3::new(0.0, WINDOW_HEIGHT / 2.0, 0.0)
                    }
                    BorderLocation::Bottom => {
                        Vec3::new(0.0, -(WINDOW_HEIGHT / 2.0), 0.0)
                    }
                    _ => Vec3::ZERO
                },
                scale: match loc {
                    BorderLocation::Top => {
                        Vec3::new(WINDOW_WIDTH / 2.0, 0.0, 0.0)
                    }
                    BorderLocation::Bottom => {
                        Vec3::new(-(WINDOW_WIDTH / 2.0), 0.0, 0.0)
                    }
                    _ => Vec3::ZERO
                },
                ..default()
            },
            Border,
            RigidBody::Fixed,
            Collider::cuboid(WINDOW_WIDTH / 2.0, 3.0)
        )
    }

    pub fn new_vertical(loc: BorderLocation) -> (Sprite, Transform, Border, RigidBody, Collider, Player, Sensor) {
        (
            Sprite::from_color(Color::BLACK, Vec2::ONE),
            Transform {
                translation: match loc {
                    BorderLocation::Left => {
                        Vec3::new(-(WINDOW_WIDTH / 2.0), 0.0, 0.0)
                    }
                    BorderLocation::Right => {
                        Vec3::new(WINDOW_WIDTH / 2.0, 0.0, 0.0)
                    }
                    _ => Vec3::ZERO
                },
                scale: match loc {
                    BorderLocation::Left => {
                        Vec3::new(0.0, -(WINDOW_WIDTH / 2.0), 0.0)
                    }
                    BorderLocation::Right => {
                        Vec3::new(0.0, WINDOW_WIDTH / 2.0, 0.0)
                    }
                    _ => Vec3::ZERO
                },
                ..default()
            },
            Border,
            RigidBody::Fixed,
            Collider::cuboid(3.0, WINDOW_HEIGHT / 2.0),
            match loc { 
                BorderLocation::Left  => Player::Player2,
                BorderLocation::Right => Player::Player1,
                _ => Player::Player1
            },
            Sensor
        )
    }
}

pub(crate) fn spawn_borders(mut commands: Commands) {
    commands.spawn_batch([
        Border::new_horizontal(BorderLocation::Top),
        Border::new_horizontal(BorderLocation::Bottom)
    ]);
    
    commands.spawn_batch([
        Border::new_vertical(BorderLocation::Left),
        Border::new_vertical(BorderLocation::Right),
    ])
}