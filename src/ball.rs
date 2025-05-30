use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const BALL_RADIUS: f32 = 10.0;

#[derive(Component)]
pub struct Ball;

pub(crate) fn spawn_ball(
    mut commands:  Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Ball,
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity::linear(Vec2::new(-250.0, 0.0)),
        Restitution {
            coefficient: 1.02,
            combine_rule: CoefficientCombineRule::Max
        }
    ));
}