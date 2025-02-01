mod game_entity;
mod game_system;
mod sprite_manager_system;
mod weapons;

use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::window::WindowResolution;
use crate::game_system::*;

// Resolution
static GAME_RESOLUTION_WIDTH: u32 = 1024;
static GAME_RESOLUTION_HEIGHT: u32 = 720;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "street_of_zombies".to_string(),
            resolution: WindowResolution {
                ..Default::default()
            },
            ..Default::default()
        }),
        close_when_requested: true,
        ..Default::default()
      })).run();
}
