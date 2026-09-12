//! How far the player got: the score and the difficulty.

mod difficulty;
mod score;

pub use difficulty::Difficulty;
pub use score::{PointValue, Score};

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((score::plugin, difficulty::plugin));
}
