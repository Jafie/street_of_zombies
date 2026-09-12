//! The score, earned by hitting and killing ennemies.

use bevy::prelude::*;

use crate::combat::Hit;
use crate::game::{GameSet, GameState};

/// The points earned by a kill are the points per hit times this
const DEATH_POINTS_MULTIPLIER: u32 = 4;

pub fn plugin(app: &mut App) {
    app.init_resource::<Score>()
        .add_systems(OnEnter(GameState::Playing), reset_score_system)
        .add_systems(Update, award_points_system.in_set(GameSet::Resolution));
}

/// The score of the current game
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Score(u32);

impl Score {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn add(&mut self, points: u32) {
        self.0 += points;
    }
}

/// Points given for hitting and killing an entity
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointValue {
    per_hit: u32,
}

impl PointValue {
    pub fn new(per_hit: u32) -> Self {
        PointValue { per_hit }
    }

    /// Points earned by a hit, with the kill bonus when the hit is fatal
    fn for_hit(&self, is_fatal: bool) -> u32 {
        let kill_bonus = if is_fatal {
            self.per_hit * DEATH_POINTS_MULTIPLIER
        } else {
            0
        };

        self.per_hit + kill_bonus
    }
}

/// Add the points of every entity hit this frame
fn award_points_system(
    mut hits: MessageReader<Hit>,
    point_values: Query<&PointValue>,
    mut score: ResMut<Score>,
) {
    for hit in hits.read() {
        if let Ok(point_value) = point_values.get(hit.target) {
            score.add(point_value.for_hit(hit.is_fatal));
        }
    }
}

fn reset_score_system(mut score: ResMut<Score>) {
    *score = Score::default();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_add_test() {
        let mut score = Score::default();
        score.add(5000);
        score.add(50);

        assert_eq!(score.value(), 5050);
    }

    #[test]
    fn points_for_a_hit_test() {
        assert_eq!(PointValue::new(50).for_hit(false), 50);
    }

    #[test]
    fn points_for_a_kill_include_kill_bonus_test() {
        assert_eq!(
            PointValue::new(50).for_hit(true),
            50 + 50 * DEATH_POINTS_MULTIPLIER
        );
    }
}
