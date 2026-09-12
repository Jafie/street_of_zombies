//! Bonuses: drops appear on the map, and picking one up gives a timed bonus.
//!
//! To add a bonus, add a `BonusKind` variant and its match arms in `kind.rs`, then choose where
//! `spawn_bonus_drops_system` spawns it. A bonus that is not a weapon also needs its effect
//! applied and reverted in `BonusSlot`.

mod drop;
mod kind;
mod slot;

pub use kind::BonusKind;
pub use slot::BonusSlot;

use bevy::prelude::*;

use crate::game::{GameSet, GameState};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            (slot::bonus_countdown_system, drop::expire_bonus_drops_system)
                .in_set(GameSet::Behavior)
                .run_if(in_state(GameState::Playing)),
            drop::pick_up_bonus_drops_system
                .in_set(GameSet::Collision)
                .run_if(in_state(GameState::Playing)),
            drop::spawn_bonus_drops_system
                .in_set(GameSet::Spawn)
                .run_if(in_state(GameState::Playing)),
        ),
    );
}
