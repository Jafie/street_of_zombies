mod game_entity;
mod game_system;
mod weapons;
mod sprite_manager_system;

use bevy::{
    prelude::*
};

use crate::game_system::*;

static GAME_RESOLUTION_WIDTH: f32 = 1024.0;
static GAME_RESOLUTION_HEIGHT: f32 = 720.0;


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "street_of_zombies".to_string(),
            width: GAME_RESOLUTION_WIDTH,
            height: GAME_RESOLUTION_HEIGHT,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_system(set_window_parameters.system())
        .add_system(keyboard_capture.system())
        .add_system(projectile_and_kill_gameplay::projectile_movement_system.system())
        .add_system(projectile_and_kill_gameplay::projectile_collision_and_score_system.system())
        .add_system(ennemy_spawn_ai_gameplay::ennemy_ai_system.system())
        .add_system(sprite_manager_system::animate_sprite_system.system())
        .run();
}

/// This "Startup-Item" modify the Window parameter (title and no-resize)
fn set_window_parameters(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(String::from("Street of Zombies"));
    window.set_resizable(false);
}
