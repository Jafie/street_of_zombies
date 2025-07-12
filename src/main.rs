mod game_entity;
mod game_system;
mod sprite_manager_system;
mod weapons;

use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::render::settings::{WgpuSettings, Backends, RenderCreation};
use bevy::render::RenderPlugin;

use crate::game_system::*;

// Resolution
static GAME_RESOLUTION_WIDTH: f32 = 1024.0;
static GAME_RESOLUTION_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "street_of_zombies".to_string(),
                        resolution: (GAME_RESOLUTION_WIDTH, GAME_RESOLUTION_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
        )
        .add_plugins(StreetOfZombiesEngine)
        .run();
}
