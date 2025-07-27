pub mod ennemy_spawn_ai_gameplay;
pub mod math_and_generator;
pub mod projectile_and_kill_gameplay;

mod scoreboard;

use crate::game_entity::*;
use crate::game_entity::ennemies::Ennemy;
use crate::game_entity::player::Player;
use crate::game_entity::projectiles::Projectile;
use crate::game_system::scoreboard::ScoreAndInfo;
use crate::sprite_manager_system::*;

use bevy::prelude::*;

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

pub struct StreetOfZombiesEngine;

impl Plugin for StreetOfZombiesEngine {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, set_window_parameters))
            .add_systems(Update, (
                keyboard_capture,
                projectile_and_kill_gameplay::projectile_movement_system,
                projectile_and_kill_gameplay::projectile_collision_and_score_system,
                ennemy_spawn_ai_gameplay::ennemy_ai_system,
                restart_on_r_system,
            ))
            .add_systems(Update, animate_sprite_system.after(keyboard_capture));
    }
}

/// Initial setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    // cameras
    commands.spawn(Camera2dBundle::default());

    // Background image
    let background_image: Handle<Image> =
        asset_server.load("images/background_street_of_zombies.png");
    commands.spawn(SpriteBundle {
        texture: background_image,
        ..Default::default()
    });

    spawn_player_and_score(commands, asset_server, texture_atlases);
}

fn spawn_player_and_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) 
{
    // Main character
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: generate_texture(
                &asset_server,
                &mut texture_atlases,
                TextureToGenerate::PLAYER,
            ),
            transform: Transform::from_xyz(
                INITIAL_PLAYER_POSITION_X,
                INITIAL_PLAYER_POSITION_Y,
                0.0,
            ),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .insert(player::Player::new(
            INITIAL_PLAYER_SPEED,
            INITIAL_PLAYER_DIRECTION,
            (INITIAL_PLAYER_POSITION_X, INITIAL_PLAYER_POSITION_Y),
        ))
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

    // Scoreboard
    commands
        .spawn(TextBundle::from_sections([
            TextSection::new(
                "Score",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                },
            ),
            TextSection::new(
                "health",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 1.0, 0.5),
                },
            ),
            TextSection::new(
                "Difficulty",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..Default::default()
        }))
        .insert(scoreboard::ScoreAndInfo::new());

    // Hidden ennemy (quick texture load)
    // This is a "pre-load" of the zombie texture.
    // Avoid to show a "Zombies" with a player Sprite for few milliseconds.
    commands.spawn(SpriteSheetBundle {
        texture_atlas: generate_texture(
            &asset_server,
            &mut texture_atlases,
            TextureToGenerate::ZOMBIE,
        ),
        transform: Transform::from_xyz(GAME_AREA_LIMIT_X + 50., GAME_AREA_LIMIT_Y + 50., 0.0),
        sprite: TextureAtlasSprite::new(1),
        visibility: Visibility::Hidden,
        ..Default::default()
    });
}

/// Capture the keyboard entry to move or fire with the player entity. Managed by as a "Bevy System"
pub fn keyboard_capture(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut player::Player, &mut Transform)>,
) {
    if let Ok((mut player, mut transform)) = query.get_single_mut() {
        let mut direction: (f32, f32) = (0.0, 0.0);
        let mut number_of_valid_pressure: u8 = 0;

        // Fire capture
        if keyboard_input.pressed(KeyCode::Space) {
            player.fire(&mut commands, &time);
        } else {
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

        player.get_moveable_interface_mut().move_sprite(
            &time,
            &direction,
            &mut transform.translation,
        );
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
pub fn is_next_movement_out_of_game_area(
    position: (f32, f32),
    direction_factor: (f32, f32),
) -> bool {
    let next_movement_coord: (f32, f32) = (
        (position.0 + direction_factor.0).abs(),
        (position.1 + direction_factor.1).abs(),
    );

    next_movement_coord.0 > GAME_AREA_LIMIT_X || next_movement_coord.1 > GAME_AREA_LIMIT_Y
}

/// This "Startup-Item" modify the Window parameter (title and no-resize)
fn set_window_parameters(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.title = "Street of Zombies".to_string();
        window.resizable = false;
    }
}

/// System to restart the game when R is pressed after game over
fn restart_on_r_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    ennemy_query: Query<Entity, With<Ennemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    scoreboard_entity_query: Query<Entity, With<ScoreAndInfo>>,
    scoreboard_state_query: Query<&ScoreAndInfo>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Only allow restart if game is over
    let is_gameover = scoreboard_state_query.iter().any(|scoreboard| scoreboard.is_gameover());

    if is_gameover && keyboard_input.just_pressed(KeyCode::R) {
        // Despawn all relevant entities
        for entity in player_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in ennemy_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in projectile_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in scoreboard_entity_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // Optionally, despawn other entities (background, etc.) if needed

        // Re-run setup to reset the game
        spawn_player_and_score(commands, asset_server, texture_atlases);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_movement_out_of_game_area() {
        assert_eq!(
            is_next_movement_out_of_game_area((GAME_AREA_LIMIT_X, GAME_AREA_LIMIT_Y), (1., 1.)),
            true
        );
    }

    #[test]
    fn next_movement_inside_of_game_area() {
        assert_eq!(is_next_movement_out_of_game_area((0., 0.), (0., 0.)), false);
    }
}
