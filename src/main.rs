mod moveable_sprites;
mod weapons;

use bevy::{
    prelude::*,
};

use crate::moveable_sprites::*;


// Game area limit
static GAME_AREA_LIMIT_X: f32 = 500.0;
static GAME_AREA_LIMIT_Y: f32 = 300.0;


// Main character initialization
static INITIAL_POST_MAIN_CHAR_X: f32 = 0.0;
static INITIAL_POST_MAIN_CHAR_Y: f32 = -215.0;
static INITIAL_SPEED_MAIN_CHAR: f32 = 200.0;
static INITIAL_DIRECTION_MAIN_CHAR: (f32, f32) = (0.0, 1.0);



fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "street_of_zombies".to_string(),
            width: GAME_AREA_LIMIT_X,
            height: GAME_AREA_LIMIT_Y,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_system(set_window_title.system())
        .add_system(keyboard_capture.system())
        .add_system(projectile_movement_system.system())
        .add_system(fire_capture.system())
        .run();
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Main character
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.0, 1.0).into()),
            transform: Transform::from_xyz(INITIAL_POST_MAIN_CHAR_X, INITIAL_POST_MAIN_CHAR_Y, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(moveable_sprites::main_character::MainCharacter::new(INITIAL_SPEED_MAIN_CHAR, INITIAL_DIRECTION_MAIN_CHAR, (INITIAL_POST_MAIN_CHAR_X, INITIAL_POST_MAIN_CHAR_Y)));
}

/// This system will then change the title during execution
fn set_window_title(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(String::from("Street of Zombies"));
}


fn keyboard_capture(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut moveable_sprites::main_character::MainCharacter, &mut Transform)>,
) {
    if let Ok((mut main_character, mut transform)) = query.single_mut() {
        let mut direction : (f32, f32) = (0.0, 0.0);
        let mut number_of_valid_pressure : u8 = 0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction.0 -= 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.0 += 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.1 += 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.1 -= 1.0;
            number_of_valid_pressure += 1;
        }


        match number_of_valid_pressure {
            0 => return,
            1 => (),
            _ => { 
                direction.0 = direction.0 / 1.5;
                direction.1 = direction.1 / 1.5;
            }
        }

        main_character.move_sprite(&time, &direction, &mut transform.translation);
    }
}

fn fire_capture(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut moveable_sprites::main_character::MainCharacter, &mut Transform)>,
) {
    if let Ok((mut main_character, _)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            main_character.fire(&mut commands, &mut materials, time);
        }
        else {
            main_character.reload_weapon();
        }
    }
}

// Fire projectiles
fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(&mut moveable_sprites::projectiles::Projectile,
    &mut Transform, Entity)>) {

    for projectile_single_query in projectile_query.iter_mut() {
        let (mut projectile, mut transform, projectile_entity) = projectile_single_query;
        let direction_of_fire = projectile.get_direction();
        projectile.move_sprite(&time, &direction_of_fire, &mut transform.translation);

        // If outside of game area, delete
        if projectile.is_out_of_distance() {
            commands.entity(projectile_entity).despawn();
        }
    }
}
