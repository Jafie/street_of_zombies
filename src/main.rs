mod moveable_sprites;
use crate::moveable_sprites::MoveableSprite;
use crate::moveable_sprites::ArmedSprite;

use bevy::{
    prelude::*,
};


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "street_of_zombies".to_string(),
            width: 500.,
            height: 300.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_system(set_window_title.system())
        .add_system(keyboard_capture.system())
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
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(moveable_sprites::MainCharacter { speed: 500.0, direction: (1.0, 0.0) });
}

/// This system will then change the title during execution
fn set_window_title(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(String::from("Street of Zombies"));
}


fn keyboard_capture(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&moveable_sprites::MainCharacter, &mut Transform)>,
) {
    if let Ok((main_character, mut transform)) = query.single_mut() {
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
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&moveable_sprites::MainCharacter, &mut Transform)>,
) {
    if let Ok((main_character, _)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            main_character.fire_projectile(&mut commands, &mut materials, main_character.direction);
        }
    }
}
