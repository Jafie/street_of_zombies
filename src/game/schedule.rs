//! The order in which the gameplay systems run during a frame.

use bevy::prelude::*;

/// The stages of a frame, run in declaration order.
///
/// A system joins a stage with `.in_set(GameSet::...)`, so it is ordered against the other
/// features without naming any of their systems. Commands issued in a stage are applied before
/// the next one starts: an entity despawned in `Behavior` is already gone in `Collision`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSet {
    /// Read the keyboard: player movement, fire and restart
    Input,
    /// Move every entity that has a `Movement`
    Movement,
    /// React to the new positions and to the elapsed time: ennemies turn back and attack,
    /// projectiles and bonus drops expire, active bonuses and difficulty progress
    Behavior,
    /// Detect contacts: projectile hits and bonus pick-ups
    Collision,
    /// Apply the consequences of the contacts: score, hit flash and game over
    Resolution,
    /// Despawn what died this frame
    Cleanup,
    /// Spawn new ennemies and bonus drops
    Spawn,
    /// Update what is shown: sprite animation, hit flash tint and HUD
    Presentation,
}

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            GameSet::Input,
            GameSet::Movement,
            GameSet::Behavior,
            GameSet::Collision,
            GameSet::Resolution,
            GameSet::Cleanup,
            GameSet::Spawn,
            GameSet::Presentation,
        )
            .chain(),
    );
}
