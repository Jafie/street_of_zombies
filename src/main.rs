mod animation;
mod bonus;
mod combat;
mod ennemy;
mod game;
mod hud;
mod math;
mod physics;
mod player;
mod progression;

use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::WindowPlugin;

// Resolution
const GAME_RESOLUTION_WIDTH: u32 = 1024;
const GAME_RESOLUTION_HEIGHT: u32 = 720;

// Vulkan on desktop; the browser only has WebGL2 (Bevy's default `webgl2` feature).
#[cfg(not(target_arch = "wasm32"))]
const RENDER_BACKENDS: Backends = Backends::VULKAN;
#[cfg(target_arch = "wasm32")]
const RENDER_BACKENDS: Backends = Backends::GL;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Street of Zombies".to_string(),
                        resolution: (GAME_RESOLUTION_WIDTH, GAME_RESOLUTION_HEIGHT).into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                        backends: Some(RENDER_BACKENDS),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
        )
        .add_plugins(game::plugin)
        .run();
}
