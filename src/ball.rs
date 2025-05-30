use bevy::prelude::*;
use crate::paddle;
use crate::paddle::Paddle;

pub const BALL_RADIUS: f32 = 10.0;

#[derive(Component)]
pub struct Ball {
    velocity: Vec2
}

pub(crate) fn spawn_ball(
    mut commands:  Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Ball {
            velocity: Vec2::new(-250.0, 0.0)
        }
    ));
}

pub(crate) fn move_ball(
    mut balls: Query<(&mut Transform, &Ball)>,
    time: Res<Time>
) {
    for (mut pos, ball) in &mut balls {
        pos.translation += ball.velocity.extend(0.0) * time.delta_secs();
    }
}

pub(crate) fn collide_ball(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles:   Query<&Transform, With<Paddle>>
) {
    for (ball, mut vel) in &mut balls {
        for paddle in paddles {
            if 
                ball.translation.x - BALL_RADIUS / 2.0 < paddle.translation.x + paddle::PADDLE_WIDTH / 4.0 &&
                ball.translation.y - BALL_RADIUS / 2.0 < paddle.translation.y + paddle::PADDLE_WIDTH / 4.0 &&
                ball.translation.x + BALL_RADIUS / 2.0 > paddle.translation.x - paddle::PADDLE_WIDTH / 4.0 &&
                ball.translation.y + BALL_RADIUS / 2.0 > paddle.translation.y - paddle::PADDLE_WIDTH / 4.0 
            { 
                vel.velocity *= -1.0;
            }
        }
    }
}