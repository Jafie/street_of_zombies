//! Moving entities and detecting contacts between them.

mod collision;
mod movement;

pub use collision::Hitbox;
pub use movement::Movement;

use bevy::prelude::*;

use crate::game::GameSet;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        movement::movement_system.in_set(GameSet::Movement),
    );
}
