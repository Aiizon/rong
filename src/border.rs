use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

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
            Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::ONE),
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
}

pub(crate) fn spawn_borders(mut commands: Commands) {
    commands.spawn_batch([
        Border::new_horizontal(BorderLocation::Top),
        Border::new_horizontal(BorderLocation::Bottom)
    ]);
}