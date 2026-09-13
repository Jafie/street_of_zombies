//! Whether the game is on its title screen, running or over, and starting a new one.

use bevy::prelude::*;

use crate::game::GameSet;

/// Whether a game has started and the player is still alive
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// The front page shown at launch. It is left for good when the first game starts.
    #[default]
    Title,
    Playing,
    GameOver,
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>().add_systems(
        Update,
        (
            start_on_space_system.run_if(in_state(GameState::Title)),
            restart_on_r_system.run_if(in_state(GameState::GameOver)),
        )
            .in_set(GameSet::Input),
    );
}

/// Marks an entity that belongs to a single game: it is despawned when a new game starts.
///
/// Ennemies, projectiles and bonus drops keep living on the game over screen, so they are
/// despawned when leaving it rather than when leaving `Playing`.
pub fn despawn_on_restart() -> DespawnOnExit<GameState> {
    DespawnOnExit(GameState::GameOver)
}

/// Start the first game when space is pressed on the title screen.
///
/// Holding space into the game fires nothing: the player's weapon starts empty and only reloads
/// once space is released.
fn start_on_space_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
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
