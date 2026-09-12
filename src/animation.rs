//! Walking animation of the characters drawn from a sprite sheet.

use std::time::Duration;

use bevy::prelude::*;

use crate::game::{GameSet, CHARACTER_SHEET_COLUMNS};
use crate::physics::Movement;

/// Time between two frames of the walking animation
const FRAME_DURATION_SECS: f32 = 0.1;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        animate_sprite_system.in_set(GameSet::Presentation),
    );
}

/// Plays the walking animation of a character while it moves
#[derive(Component)]
pub struct WalkAnimation {
    timer: Timer,
    /// Position at the last frame change, to know whether the character moved since
    last_position: Vec2,
}

impl WalkAnimation {
    /// A new animation, for a character spawned at `position`
    pub fn new(position: Vec2) -> Self {
        WalkAnimation {
            timer: Timer::from_seconds(FRAME_DURATION_SECS, TimerMode::Repeating),
            last_position: position,
        }
    }

    /// Let time pass. True when the next frame shall be shown: the frame duration elapsed and
    /// the character moved since the last frame change.
    fn tick(&mut self, delta: Duration, position: Vec2) -> bool {
        self.timer.tick(delta);

        if !self.timer.is_finished() || position == self.last_position {
            return false;
        }

        self.last_position = position;
        true
    }
}

/// Row of the sprite sheet for each direction. Rows are ordered DOWN, LEFT, RIGHT, UP.
#[derive(PartialEq, Debug, Clone, Copy)]
enum SheetRow {
    Down = 0,
    Left = 1,
    Right = 2,
    Up = 3,
}

impl SheetRow {
    /// The row showing the character facing a direction
    fn from_direction(direction: Vec2) -> Self {
        if direction.x.abs() == direction.y.abs() {
            // Exact diagonal: use the vertical direction
            return if direction.y > 0. {
                SheetRow::Up
            } else {
                SheetRow::Down
            };
        }

        let is_horizontal = direction.x.abs() > direction.y.abs();
        let is_right_or_down = (direction.x - direction.y) >= 0.;

        match (is_horizontal, is_right_or_down) {
            (true, true) => SheetRow::Right,
            (true, false) => SheetRow::Left,
            (false, true) => SheetRow::Down,
            (false, false) => SheetRow::Up,
        }
    }
}

/// Index of the next frame: the next column, on the row of the direction
fn next_frame_index(current_index: usize, direction: Vec2) -> usize {
    let columns = CHARACTER_SHEET_COLUMNS as usize;

    (current_index + 1) % columns + SheetRow::from_direction(direction) as usize * columns
}

/// Advance the walking animation of every moving character
fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&Movement, &Transform, &mut WalkAnimation, &mut Sprite)>,
) {
    for (movement, transform, mut animation, mut sprite) in query.iter_mut() {
        if !animation.tick(time.delta(), transform.translation.truncate()) {
            continue;
        }

        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = next_frame_index(atlas.index, movement.direction());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LONGER_THAN_A_FRAME: Duration = Duration::from_millis(150);

    #[test]
    fn direction_to_row_up() {
        assert_eq!(SheetRow::from_direction(Vec2::new(0.0, 1.0)), SheetRow::Up);
    }

    #[test]
    fn direction_to_row_down() {
        assert_eq!(SheetRow::from_direction(Vec2::new(0.0, -1.0)), SheetRow::Down);
    }

    #[test]
    fn direction_to_row_left() {
        assert_eq!(SheetRow::from_direction(Vec2::new(-1.0, 0.0)), SheetRow::Left);
    }

    #[test]
    fn direction_to_row_right() {
        assert_eq!(SheetRow::from_direction(Vec2::new(1.0, 0.0)), SheetRow::Right);
    }

    #[test]
    fn direction_to_row_priority_up() {
        assert_eq!(SheetRow::from_direction(Vec2::new(1.0, 1.0)), SheetRow::Up);
    }

    #[test]
    fn direction_to_row_priority_down() {
        assert_eq!(SheetRow::from_direction(Vec2::new(1.0, -1.0)), SheetRow::Down);
    }

    #[test]
    fn next_frame_wraps_on_the_row_of_the_direction() {
        // Last column of the DOWN row, now heading RIGHT: first column of the RIGHT row
        assert_eq!(next_frame_index(7, Vec2::X), 16);
        // Second column of the RIGHT row, still heading RIGHT
        assert_eq!(next_frame_index(17, Vec2::X), 18);
    }

    #[test]
    fn animation_advances_when_moved_after_a_frame() {
        let mut animation = WalkAnimation::new(Vec2::new(15., 20.));

        assert_eq!(animation.tick(LONGER_THAN_A_FRAME, Vec2::new(20., 50.)), true);
    }

    #[test]
    fn animation_does_not_advance_when_not_moved() {
        let mut animation = WalkAnimation::new(Vec2::new(15., 20.));

        assert_eq!(animation.tick(LONGER_THAN_A_FRAME, Vec2::new(15., 20.)), false);
    }

    #[test]
    fn animation_does_not_advance_before_a_frame() {
        let mut animation = WalkAnimation::new(Vec2::new(15., 20.));

        assert_eq!(
            animation.tick(Duration::from_millis(10), Vec2::new(20., 50.)),
            false
        );
    }

    #[test]
    fn animation_compares_with_the_position_at_the_last_frame_change() {
        let mut animation = WalkAnimation::new(Vec2::ZERO);
        animation.tick(LONGER_THAN_A_FRAME, Vec2::new(20., 50.));

        assert_eq!(animation.tick(LONGER_THAN_A_FRAME, Vec2::new(20., 50.)), false);
    }
}
