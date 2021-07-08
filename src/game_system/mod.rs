pub mod math_and_generator;
pub mod ennemy_spawn_ai_gameplay;
pub mod projectile_and_kill_gameplay;

mod scoreboard;

use crate::game_entity::*;
use crate::sprite_manager_system::*;

use bevy::{
    prelude::*
};

// Game area limit
pub static GAME_AREA_LIMIT_X: f32 = 500.0;
pub static GAME_AREA_LIMIT_Y: f32 = 300.0;

static MAXIMUM_ENNEMY_DISTANCE: f32 = 300.;
static INITIAL_ENNEMY_SPEED: f32 = 200.0;

// Main character initialization
static INITIAL_PLAYER_POSITION_X: f32 = 0.0;
static INITIAL_PLAYER_POSITION_Y: f32 = -215.0;
static INITIAL_PLAYER_SPEED: f32 = 350.0;
static INITIAL_PLAYER_DIRECTION: (f32, f32) = (0.0, 1.0);

/// Initial setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());


    // Main character
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: generate_texture(&asset_server, &mut texture_atlases, TextureToGenerate::PLAYER),
            transform: Transform::from_xyz(INITIAL_PLAYER_POSITION_X, INITIAL_PLAYER_POSITION_Y, 0.0),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .insert(player::Player::new(INITIAL_PLAYER_SPEED, 
            INITIAL_PLAYER_DIRECTION,
            (INITIAL_PLAYER_POSITION_X, INITIAL_PLAYER_POSITION_Y)))
        .insert(Timer::from_seconds(0.1, true));

    // Scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "health".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 1.0, 0.5),
                    },
                },
                TextSection {
                    value: "Difficulty".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(scoreboard::ScoreAndInfo::new());
}

/// Capture the keyboard entry to move or fire with the player entity. Managed by as a "Bevy System"
pub fn keyboard_capture(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut player::Player, &mut Transform)>) {
    
    if let Ok((mut player, mut transform)) = query.single_mut() {
        let mut direction : (f32, f32) = (0.0, 0.0);
        let mut number_of_valid_pressure : u8 = 0;

        // Fire capture
        if keyboard_input.pressed(KeyCode::Space) {
            player.fire(&mut commands, &mut materials, &time);
        }
        else {
            player.reload_weapon();
        }

        // Movement        
        if keyboard_input.pressed(KeyCode::Left) {
            direction.0 = -1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.0 = 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.1 = 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.1 = -1.0;
            number_of_valid_pressure += 1;
        }

        match number_of_valid_pressure {
            0 => return,
            1 => (),
            _ => { 
                direction.0 = (direction.0.abs() - 0.33) * direction.0;
                direction.1 = (direction.1.abs() - 0.33) * direction.1;
            }
        }

        
        player.move_sprite(&time, &direction, &mut transform.translation);
    }
}


/// Check if the next movement will be out of the game area.
///
/// # Arguments
///
/// * `position` - The current coordinate of the entity in a cartesian graph (x, y)
/// * `direction_factor` - The direction factor of the entity in a cartesian graph (x, y)
///
/// ```
pub fn is_next_movement_out_of_game_area(position: (f32, f32), direction_factor: (f32, f32)) -> bool {
    let next_movement_coord : (f32, f32) = ((position.0 + direction_factor.0).abs(), (position.1 + direction_factor.1).abs());
    
    next_movement_coord.0 > GAME_AREA_LIMIT_X || next_movement_coord.1 > GAME_AREA_LIMIT_Y
}
