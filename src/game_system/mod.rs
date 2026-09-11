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
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    // cameras
    commands.spawn(Camera2d);

    // Background image
    let background_image: Handle<Image> =
        asset_server.load("images/background_street_of_zombies.png");
    commands.spawn(Sprite::from_image(background_image));

    spawn_player_and_score(commands, asset_server, texture_atlas_layouts);
}

fn spawn_player_and_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>)
{
    // Main character
    commands.spawn((
        generate_texture(
            &asset_server,
            &mut texture_atlas_layouts,
            TextureToGenerate::PLAYER,
        ),
        Transform::from_xyz(
            INITIAL_PLAYER_POSITION_X,
            INITIAL_PLAYER_POSITION_Y,
            0.0,
        ),
        player::Player::new(
            INITIAL_PLAYER_SPEED,
            INITIAL_PLAYER_DIRECTION,
            (INITIAL_PLAYER_POSITION_X, INITIAL_PLAYER_POSITION_Y),
        ),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    // Scoreboard
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.into(),
        font_size: FontSize::Px(40.0),
        ..Default::default()
    };

    commands
        .spawn((
            Text::new("Score"),
            text_font.clone(),
            TextColor(Color::srgb(0.5, 0.5, 1.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            scoreboard::ScoreAndInfo::new(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("health"),
                text_font.clone(),
                TextColor(Color::srgb(0.5, 1.0, 0.5)),
            ));
            parent.spawn((
                TextSpan::new("Difficulty"),
                text_font,
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });

    // Hidden ennemy (quick texture load)
    // This is a "pre-load" of the zombie texture.
    // Avoid to show a "Zombies" with a player Sprite for few milliseconds.
    commands.spawn((
        generate_texture(
            &asset_server,
            &mut texture_atlas_layouts,
            TextureToGenerate::ZOMBIE,
        ),
        Transform::from_xyz(GAME_AREA_LIMIT_X + 50., GAME_AREA_LIMIT_Y + 50., 0.0),
        Visibility::Hidden,
    ));
}

/// Capture the keyboard entry to move or fire with the player entity. Managed by as a "Bevy System"
pub fn keyboard_capture(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut player::Player, &mut Transform)>,
) {
    if let Ok((mut player, mut transform)) = query.single_mut() {
        let mut direction: (f32, f32) = (0.0, 0.0);
        let mut number_of_valid_pressure: u8 = 0;

        // Fire capture
        if keyboard_input.pressed(KeyCode::Space) {
            player.fire(&mut commands, &time);
        } else {
            player.reload_weapon();
        }

        // Movement
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.0 = -1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.0 = 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.1 = 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
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
    if let Ok(mut window) = windows.single_mut() {
        window.title = "Street of Zombies".to_string();
        window.resizable = false;
    }
}

/// System to restart the game when R is pressed after game over
fn restart_on_r_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    ennemy_query: Query<Entity, With<Ennemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    scoreboard_entity_query: Query<Entity, With<ScoreAndInfo>>,
    scoreboard_state_query: Query<&ScoreAndInfo>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Only allow restart if game is over
    let is_gameover = scoreboard_state_query.iter().any(|scoreboard| scoreboard.is_gameover());

    if is_gameover && keyboard_input.just_pressed(KeyCode::KeyR) {
        // Despawn all relevant entities (despawn also removes children, e.g. scoreboard text spans)
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in ennemy_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in projectile_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in scoreboard_entity_query.iter() {
            commands.entity(entity).despawn();
        }
        // Optionally, despawn other entities (background, etc.) if needed

        // Re-run setup to reset the game
        spawn_player_and_score(commands, asset_server, texture_atlas_layouts);
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
