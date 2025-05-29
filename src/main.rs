use bevy::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(10.0, 100.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera, 
            spawn_players
        ))
        .run()
    ;
}

#[derive(Component)]
struct Paddle {
    move_up:   KeyCode,
    move_down: KeyCode
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_players(mut commands: Commands, query: Query<&Window>) {
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
                move_up:   KeyCode::KeyZ,
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
                translation: Vec3::new((width - 15.0), 0.0, 0.0),
                ..default()
            },
            Paddle {
                move_up:   KeyCode::ArrowUp,
                move_down: KeyCode::ArrowDown,
            },
        ),
    ]);
}