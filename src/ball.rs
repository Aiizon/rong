use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use crate::border::Border;
use crate::GameEvents;
use crate::player::Player;

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
        CollidingEntities::default(),
        ActiveEvents::COLLISION_EVENTS,
        Collider::ball(BALL_RADIUS),
        Velocity::linear(Vec2::new(-250.0, 0.0)),
        Restitution {
            coefficient: 1.02,
            combine_rule: CoefficientCombineRule::Max
        }
    ));
}

pub(crate) fn detect_reset(
    input:           Res<ButtonInput<KeyCode>>,
    balls:           Query<&CollidingEntities, With<Ball>>,
    goals:           Query<&Player, With<Sensor>>,
    mut game_events: EventWriter<GameEvents>
) {
    if input.just_pressed(KeyCode::Space) {
        let player = if rand::rng().random_bool(0.5) {
            Player::Player1
        } else {
            Player::Player2
        };
        game_events.write(GameEvents::ResetBall(player));
    }

    for ball in &balls {
        for hit_entity in ball.iter() {
            if let Ok(player) = goals.get(hit_entity) {
                game_events.write(GameEvents::ResetBall(*player));
            }
        }
    }
}

pub(crate) fn reset_ball(
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut game_events: EventReader<GameEvents>
) {
    for game_event in game_events.read() {
        match game_event { 
            GameEvents::ResetBall(player) => {
                for (mut ball, mut vel) in &mut balls {
                    ball.translation = Vec3::new(0.0, 0.0, 1.0);
                    *vel = player.start_speed();
                }
            }
            _ => {}
        }
    }
}