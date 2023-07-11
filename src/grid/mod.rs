use bevy::prelude::*;

mod interactions;
use interactions::interact_with_screen_button;
pub mod layout;
use layout::spawn_main_menu;
mod styles;

pub const SIZE: (usize, usize) = (9, 9);

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_main_menu)
            .add_system(interact_with_screen_button);
    }
}
