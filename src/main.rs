use std::{
    thread,
    time::{self, Duration},
};

use bevy::{
    app::StartupSet::PostStartup,
    prelude::*,
    window::{PresentMode, PrimaryWindow, WindowMode, WindowResized},
};

mod grid;
use grid::MainMenuPlugin;

mod maze_gen;
use maze_gen::Maze;

const BACKGROUND_SIZE: (f32, f32) = (1024., 800.);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Into The Magical Forest".to_string(),
                resolution: (1200., 1000.).into(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_maze)
        // .add_startup_system(spawn_backgrounds)
        // // make sure that the background is initialized after all of the backgrounds have spawned
        // .add_startup_system(init_background.in_base_set(PostStartup))
        // .add_system(maintain_background)
        .run();
}

#[derive(Component)]
pub struct MazeComponent {
    pub maze: Maze,
}

#[derive(Clone, Component)]
struct Background {
    pub image_path: String,
    pub location: (usize, usize),
}

// there should only ever be one ActiveBackground instace, the contence of the ActiveBackground
// will change as the game changes.
#[derive(Component)]
struct ActiveBackground {
    pub background: Background,
}

fn spawn_maze(mut commands: Commands) {
    let maze = Maze::gen(grid::SIZE.0, grid::SIZE.1);
    commands.spawn(MazeComponent { maze: maze.clone() });

    maze.display_maze();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..Default::default()
    });
}

fn spawn_backgrounds(mut commands: Commands) {
    commands.spawn(Background {
        image_path: String::from("sprites/backgrounds/grid/9-5.png"),
        location: (9, 5),
    });
    commands.spawn(Background {
        image_path: String::from("sprites/backgrounds/grid/7-6.png"),
        location: (7, 6),
    });
}

fn init_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    background_query: Query<&Background>,
) {
    let window = window_query.get_single().unwrap();
    let active_background = ActiveBackground {
        background: find_background(background_query, (9, 5)),
    };

    let scale_height = window.height() / BACKGROUND_SIZE.1;
    let scale_width = window.width() / BACKGROUND_SIZE.0;

    // find the smallest of the scales
    let scale = if scale_width > scale_height {
        scale_height
    } else {
        scale_width
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.)
                .with_scale(Vec3::splat(scale)),
            texture: asset_server.load(&active_background.background.image_path),
            ..Default::default()
        },
        active_background,
    ));
}

// the size of the background should change as the window is resized to fit the greatest size it
// can on the screen while still maintaining the original aspect ratio.
fn maintain_background(
    resize_event: Res<Events<WindowResized>>,
    mut active_background_query: Query<&mut Transform, With<ActiveBackground>>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        let scale_height = e.height / BACKGROUND_SIZE.1;
        let scale_width = e.width / BACKGROUND_SIZE.0;

        // find the smallest of the scales
        let scale = if scale_width > scale_height {
            scale_height
        } else {
            scale_width
        };

        let mut transform = active_background_query.get_single_mut().unwrap();

        *transform = transform.with_scale(Vec3::splat(scale));
    }
}

fn find_background(background_query: Query<&Background>, location: (usize, usize)) -> Background {
    background_query
        .iter()
        .find(|b| b.location == location)
        .expect("Could not find background that matched location")
        .clone()
}

fn change_background(
    asset_server: Res<AssetServer>,
    mut active_background_query: Query<&mut Handle<Image>, With<ActiveBackground>>,
    new_background_query: Query<&Background>,
    new_background: (usize, usize),
) {
    // this should be safe to unwrap because there is only one ActiveBackground instance in the
    // program
    let mut background = active_background_query.get_single_mut().unwrap();

    *background =
        asset_server.load(&find_background(new_background_query, new_background).image_path);
}
