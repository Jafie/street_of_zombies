//! Wiring shared by every feature: the order of the systems in a frame, the game state, the
//! loaded assets and the play area.

mod arena;
mod assets;
mod schedule;
mod state;

pub use arena::{clamp_to_arena, is_outside_arena, random_position_in_arena};
pub use assets::{CharacterSprite, GameAssets, CHARACTER_SHEET_COLUMNS};
pub use schedule::GameSet;
pub use state::{despawn_on_restart, GameState};

use bevy::prelude::*;

use crate::{animation, bonus, combat, ennemy, hud, physics, player, progression};

/// Register the whole game: the shared wiring first, then every feature.
pub fn plugin(app: &mut App) {
    app.add_plugins((schedule::plugin, state::plugin, assets::plugin, arena::plugin))
        .add_plugins((
            physics::plugin,
            animation::plugin,
            combat::plugin,
            progression::plugin,
            player::plugin,
            ennemy::plugin,
            bonus::plugin,
            hud::plugin,
        ));
}
