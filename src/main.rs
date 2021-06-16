mod player;

use bevy::{
    prelude::*,
};

/// An implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "street_of_zombies".to_string(),
            width: 500.,
            height: 300.,
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_system(set_window_title.system())
        .add_system(keyboard_capture.system())
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
        .insert(player::MainCharacter { speed: 500.0 });
}

/// This system will then change the title during execution
fn set_window_title(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(String::from("Street of Zombies"));
}

pub fn keyboard_capture(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&player::MainCharacter, &mut Transform)>,
) {
    if let Ok((main_character, mut transform)) = query.single_mut() {
        let mut direction : f32 = 0.0;
        let translation = &mut transform.translation;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
            main_character.move_sprite(&time, &direction, &mut translation.x)
        }
        else if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
            main_character.move_sprite(&time, &direction, &mut translation.x)
        }
        else if keyboard_input.pressed(KeyCode::Up) {
            direction += 1.0;
            main_character.move_sprite(&time, &direction, &mut translation.y)
        }
        else if keyboard_input.pressed(KeyCode::Down) {
            direction -= 1.0;
            main_character.move_sprite(&time, &direction, &mut translation.y)
        }
        else if keyboard_input.pressed(KeyCode::Space) {
            direction -= 1.0;
            main_character.move_sprite(&time, &direction, &mut translation.y)
        }
    }
}
