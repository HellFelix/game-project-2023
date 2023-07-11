use super::styles::{
    get_button_text_style, get_title_text_style, BUTTON_STYLE, MAIN_MENU_STYLE,
    NORMAL_BUTTON_COLOR, TITLE_STYLE,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Page {
    pub location: (usize, usize),
}

#[derive(Component)]
pub struct DirectionButton {
    pub direction: Direction,
}

pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub static mut CURRENT_LOCATION: (usize, usize) = (1, 1);

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    unsafe {
        build_screen(&mut commands, &asset_server, CURRENT_LOCATION);
    }
}

pub fn build_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    location: (usize, usize),
) {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..Default::default()
            },
            Page { location },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                location.0.to_string() + " - " + location.1.to_string().as_str(),
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    DirectionButton {
                        direction: Direction::Up,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Up",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    DirectionButton {
                        direction: Direction::Right,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Right",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    DirectionButton {
                        direction: Direction::Left,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Left",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    DirectionButton {
                        direction: Direction::Down,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Down",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}
