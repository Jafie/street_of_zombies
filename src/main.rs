mod game_entity;
mod game_system;
mod weapons;
mod sprite_manager_system;

use bevy::{
    prelude::*
};

use crate::game_system::*;

// Resolution
static GAME_RESOLUTION_WIDTH: f32 = 1024.0;
static GAME_RESOLUTION_HEIGHT: f32 = 720.0;


#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
        

    app.add_plugin(StreetOfZombiesEngine)
    .insert_resource(WindowDescriptor {
        title: "street_of_zombies".to_string(),
        width: GAME_RESOLUTION_WIDTH,
        height: GAME_RESOLUTION_HEIGHT,
        ..Default::default()
    })
    .run();
}
