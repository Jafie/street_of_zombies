//! Whether a game is running or over, and starting a new one.

use bevy::prelude::*;

use crate::game::GameSet;

/// Whether the player is still alive
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>().add_systems(
        Update,
        restart_on_r_system
            .in_set(GameSet::Input)
            .run_if(in_state(GameState::GameOver)),
    );
}

/// Marks an entity that belongs to a single game: it is despawned when a new game starts.
///
/// Ennemies, projectiles and bonus drops keep living on the game over screen, so they are
/// despawned when leaving it rather than when leaving `Playing`.
pub fn despawn_on_restart() -> DespawnOnExit<GameState> {
    DespawnOnExit(GameState::GameOver)
}

/// Start a new game when R is pressed on the game over screen
fn restart_on_r_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}
