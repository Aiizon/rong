use bevy::prelude::*;

const BALL_RADIUS: f32 = 10.0;

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