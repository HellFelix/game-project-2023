use crate::MazeComponent;

use super::layout::{build_screen, Direction::*, DirectionButton, Page, CURRENT_LOCATION};
use super::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use super::SIZE;
use bevy::prelude::*;

pub fn interact_with_screen_button(
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &DirectionButton),
        Changed<Interaction>,
    >,
    page_entity: Query<Entity, With<Page>>,
    asset_server: Res<AssetServer>,
    maze_query: Query<&MazeComponent>,
) {
    let maze = maze_query.get_single().unwrap().maze.clone();
    if let Ok((interaction, mut background_color, button)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                if let Ok(page_entity) = page_entity.get_single() {
                    commands.entity(page_entity).despawn_recursive();

                    let direction = &button.direction;

                    unsafe {
                        match direction {
                            Right => {
                                if CURRENT_LOCATION.0 < SIZE.0
                                    && !maze.walls.0[SIZE.1 - CURRENT_LOCATION.1]
                                        [CURRENT_LOCATION.0 - 1]
                                {
                                    CURRENT_LOCATION.0 += 1;
                                }
                            }
                            Left => {
                                if CURRENT_LOCATION.0 > 1
                                    && !maze.walls.0[SIZE.1 - CURRENT_LOCATION.1]
                                        [CURRENT_LOCATION.0 - 2]
                                {
                                    CURRENT_LOCATION.0 -= 1;
                                }
                            }
                            Up => {
                                if CURRENT_LOCATION.1 < SIZE.1
                                    && !maze.walls.1[SIZE.1 - CURRENT_LOCATION.1 - 1]
                                        [CURRENT_LOCATION.0 - 1]
                                {
                                    CURRENT_LOCATION.1 += 1;
                                }
                            }
                            Down => {
                                if CURRENT_LOCATION.1 > 1
                                    && !maze.walls.1[SIZE.1 - CURRENT_LOCATION.1]
                                        [CURRENT_LOCATION.0 - 1]
                                {
                                    CURRENT_LOCATION.1 -= 1;
                                }
                            }
                        }
                        build_screen(&mut commands, &asset_server, CURRENT_LOCATION)
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
