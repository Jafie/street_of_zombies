//! Moving entities at a constant speed, inside the play area.

use bevy::prelude::*;

use crate::game::clamp_to_arena;

/// Moves an entity at a constant speed along a direction
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Movement {
    speed: f32,
    /// Where the entity heads. Kept while it stands still, so it keeps facing that way.
    direction: Vec2,
    is_moving: bool,
}

impl Movement {
    /// A movement that starts right away
    pub fn new(speed: f32, direction: Vec2) -> Self {
        Movement {
            speed,
            direction,
            is_moving: true,
        }
    }

    /// A movement that stands still, facing a direction
    pub fn standing(speed: f32, direction: Vec2) -> Self {
        Movement {
            is_moving: false,
            ..Movement::new(speed, direction)
        }
    }

    /// Where the entity heads, or faces while it stands still
    pub fn direction(&self) -> Vec2 {
        self.direction
    }

    /// Start moving along a direction
    pub fn start_moving(&mut self, direction: Vec2) {
        self.direction = direction;
        self.is_moving = true;
    }

    /// Stand still, keeping the current direction
    pub fn stop(&mut self) {
        self.is_moving = false;
    }

    /// Head the opposite way
    pub fn reverse(&mut self) {
        self.direction = -self.direction;
    }

    /// The position after moving for `delta_secs`, kept inside the play area.
    /// `None` while standing still.
    pub fn next_position(&self, position: Vec2, delta_secs: f32) -> Option<Vec2> {
        self.is_moving
            .then(|| clamp_to_arena(position + self.direction * self.speed * delta_secs))
    }
}

/// Move every entity along its direction
pub fn movement_system(time: Res<Time>, mut query: Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in query.iter_mut() {
        let position = transform.translation.truncate();

        // Standing entities are left untouched, so their `Transform` is not marked as changed
        if let Some(next_position) = movement.next_position(position, time.delta_secs()) {
            transform.translation = next_position.extend(transform.translation.z);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_advances_along_direction() {
        let movement = Movement::new(500.0, Vec2::new(0.5, -0.5));

        assert_eq!(
            movement.next_position(Vec2::new(15., 20.), 0.1),
            Some(Vec2::new(40., -5.))
        );
    }

    #[test]
    fn movement_is_clamped_to_the_arena() {
        let movement = Movement::new(500.0, Vec2::X);

        assert_eq!(
            movement.next_position(Vec2::ZERO, 10.0),
            Some(clamp_to_arena(Vec2::new(5000., 0.)))
        );
    }

    #[test]
    fn standing_movement_does_not_move() {
        let movement = Movement::standing(500.0, Vec2::Y);

        assert_eq!(movement.next_position(Vec2::new(15., 20.), 0.1), None);
        assert_eq!(movement.direction(), Vec2::Y);
    }

    #[test]
    fn start_moving_sets_direction() {
        let mut movement = Movement::standing(500.0, Vec2::Y);
        movement.start_moving(Vec2::new(100., 45.));

        assert_eq!(movement.direction(), Vec2::new(100., 45.));
        assert!(movement.next_position(Vec2::ZERO, 0.001).is_some());
    }

    #[test]
    fn stop_keeps_direction() {
        let mut movement = Movement::new(500.0, Vec2::new(5., 10.));
        movement.stop();

        assert_eq!(movement.direction(), Vec2::new(5., 10.));
        assert_eq!(movement.next_position(Vec2::ZERO, 0.1), None);
    }

    #[test]
    fn reverse_flips_direction() {
        let mut movement = Movement::new(500.0, Vec2::new(5., -10.));
        movement.reverse();

        assert_eq!(movement.direction(), Vec2::new(-5., 10.));
    }
}
