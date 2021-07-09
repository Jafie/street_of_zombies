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

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(StreetOfZombiesEngine)
        .insert_resource(WindowDescriptor {
            title: "street_of_zombies".to_string(),
            width: GAME_RESOLUTION_WIDTH,
            height: GAME_RESOLUTION_HEIGHT,
            ..Default::default()
        })
        .run();
}
