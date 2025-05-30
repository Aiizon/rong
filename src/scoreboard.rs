use crate::Srgba;
use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub(crate) struct Scoreboard; 

pub(crate) fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 10.0),
        Scoreboard,
        TextColor(Color::WHITE),
        Node {
            position_type:   PositionType::Absolute,
            display:         Display::Flex,
            flex_direction:  FlexDirection::Row,
            align_items:     AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            margin:          UiRect::horizontal(Val::Auto),
            padding:         UiRect::all(Val::Px(20.0)),
            top:             Val::ZERO,
            width:           Val::Percent(30.0),
            height:          Val::Percent(15.0),
            ..default()
        },
        BackgroundColor::from(Srgba::new(0.2, 0.2, 0.2, 0.5))
    )).with_children(|parent| {
        parent.spawn((
            Text::new("0"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Player::Player1
        ));
        
        parent.spawn((
            Text::new("|"),
            TextFont {
                font_size: 42.0,
                ..default()
            },
            TextColor(Color::WHITE)
        ));

        parent.spawn((
            Text::new("0"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Player::Player2
        ));
    });
}