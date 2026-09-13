//! Random appearance of new ennemies, more frequent as the difficulty rises.

use bevy::prelude::*;
use rand::Rng;

use crate::ennemy::{spawn_ennemy, Ennemy};
use crate::game::{random_position_in_arena, GameAssets, GameSet, GameState};
use crate::progression::Difficulty;

const MAXIMUM_NUMBER_OF_ENNEMIES: usize = 40;
/// Upper bound of the spawn roll at the lowest difficulty
const SPAWN_ROLL_RANGE: u32 = 1100;
/// How much each difficulty level lowers the spawn roll range
const SPAWN_ROLL_RANGE_REDUCTION_PER_LEVEL: u32 = 200;
/// A roll at or below this spawns an ennemy
const SPAWN_ROLL_THRESHOLD: u32 = 2;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        ennemy_spawn_system
            .in_set(GameSet::Spawn)
            // Ennemies are despawned when leaving the game over screen, not the title screen:
            // one spawned behind the title would still be there when the first game starts
            .run_if(not(in_state(GameState::Title))),
    );
}

/// Upper bound of the spawn roll for a difficulty level: the lower, the more frequent the spawns
fn spawn_roll_range(difficulty_level: u32) -> u32 {
    SPAWN_ROLL_RANGE
        .saturating_sub(SPAWN_ROLL_RANGE_REDUCTION_PER_LEVEL * difficulty_level)
        .max(1)
}

/// True when a new ennemy shall spawn this frame
fn is_spawn_triggered(roll: u32, ennemies_on_map: usize) -> bool {
    ennemies_on_map < MAXIMUM_NUMBER_OF_ENNEMIES && roll <= SPAWN_ROLL_THRESHOLD
}

fn ennemy_spawn_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    difficulty: Res<Difficulty>,
    ennemies: Query<(), With<Ennemy>>,
) {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..spawn_roll_range(difficulty.level()));

    if is_spawn_triggered(roll, ennemies.iter().count()) {
        let direction = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        spawn_ennemy(&mut commands, &assets, random_position_in_arena(), direction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_roll_range_shrinks_with_difficulty() {
        assert_eq!(spawn_roll_range(0), 1100);
        assert_eq!(spawn_roll_range(1), 900);
        assert_eq!(spawn_roll_range(5), 100);
    }

    #[test]
    fn spawn_roll_range_never_empty() {
        assert_eq!(spawn_roll_range(100), 1);
    }

    #[test]
    fn low_roll_spawns_ennemy() {
        assert_eq!(is_spawn_triggered(SPAWN_ROLL_THRESHOLD, 0), true);
    }

    #[test]
    fn high_roll_does_not_spawn_ennemy() {
        assert_eq!(is_spawn_triggered(SPAWN_ROLL_THRESHOLD + 1, 0), false);
    }

    #[test]
    fn spawn_blocked_when_map_is_full() {
        assert_eq!(is_spawn_triggered(0, MAXIMUM_NUMBER_OF_ENNEMIES), false);
    }
}
