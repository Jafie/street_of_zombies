//! The difficulty level, rising with the time spent playing.

use bevy::prelude::*;

use crate::game::{GameSet, GameState};

/// Name of each difficulty level. The last one is the maximum level.
const LEVEL_NAMES: [&str; 6] = [
    "EASY",
    "NORMAL",
    "HARD",
    "EXTREME",
    "STILL OK?",
    "!YOU'LL DIE!",
];
const MAX_DIFFICULTY_LEVEL: u32 = LEVEL_NAMES.len() as u32 - 1;
const SECONDS_BEFORE_NEXT_LEVEL: u32 = 30;

pub fn plugin(app: &mut App) {
    app.init_resource::<Difficulty>()
        .add_systems(OnEnter(GameState::Playing), reset_difficulty_system)
        .add_systems(
            Update,
            difficulty_progression_system
                .in_set(GameSet::Behavior)
                .run_if(in_state(GameState::Playing)),
        );
}

/// The difficulty of the current game
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct Difficulty {
    level: u32,
    /// Time spent on the current level, in seconds
    time_on_level: f32,
}

impl Difficulty {
    /// From 0 (easiest) to the maximum level
    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn name(&self) -> &'static str {
        LEVEL_NAMES[self.level as usize]
    }

    /// Progress towards the next level, in percent of whole seconds.
    /// `None` at the maximum level.
    pub fn percent_until_next_level(&self) -> Option<u32> {
        (!self.is_max_level())
            .then(|| self.time_on_level as u32 * 100 / SECONDS_BEFORE_NEXT_LEVEL)
    }

    /// Let time pass: the next level is reached after 30 seconds on the current one
    fn advance(&mut self, delta_secs: f32) {
        if self.is_max_level() {
            return;
        }

        self.time_on_level += delta_secs;

        if self.time_on_level as u32 >= SECONDS_BEFORE_NEXT_LEVEL {
            self.level += 1;
            self.time_on_level = 0.0;
        }
    }

    fn is_max_level(&self) -> bool {
        self.level >= MAX_DIFFICULTY_LEVEL
    }
}

fn difficulty_progression_system(time: Res<Time>, mut difficulty: ResMut<Difficulty>) {
    difficulty.advance(time.delta_secs());
}

fn reset_difficulty_system(mut difficulty: ResMut<Difficulty>) {
    *difficulty = Difficulty::default();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reach a level, one level at a time
    fn difficulty_at_level(level: u32) -> Difficulty {
        let mut difficulty = Difficulty::default();
        for _ in 0..level {
            difficulty.advance(SECONDS_BEFORE_NEXT_LEVEL as f32);
        }
        difficulty
    }

    #[test]
    fn initial_difficulty_test() {
        let difficulty = Difficulty::default();

        assert_eq!(difficulty.level(), 0);
        assert_eq!(difficulty.name(), "EASY");
        assert_eq!(difficulty.percent_until_next_level(), Some(0));
    }

    #[test]
    fn percent_counts_whole_seconds_test() {
        let mut difficulty = Difficulty::default();
        difficulty.advance(1.5);

        assert_eq!(
            difficulty.percent_until_next_level(),
            Some(100 / SECONDS_BEFORE_NEXT_LEVEL)
        );
    }

    #[test]
    fn increase_difficulty_level_test() {
        let mut difficulty = Difficulty::default();
        difficulty.advance(SECONDS_BEFORE_NEXT_LEVEL as f32 - 0.5);
        assert_eq!(difficulty.level(), 0);

        difficulty.advance(0.5);
        assert_eq!(difficulty.level(), 1);
        assert_eq!(difficulty.percent_until_next_level(), Some(0));
    }

    #[test]
    fn increase_difficulty_level_to_max_test() {
        let difficulty = difficulty_at_level(MAX_DIFFICULTY_LEVEL + 5);

        assert_eq!(difficulty.level(), MAX_DIFFICULTY_LEVEL);
        assert_eq!(difficulty.name(), "!YOU'LL DIE!");
        assert_eq!(difficulty.percent_until_next_level(), None);
    }
}
