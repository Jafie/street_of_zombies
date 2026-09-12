//! Health points of anything that can be killed.

use bevy::prelude::*;

/// Health points of an entity. It is dead at zero.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Health {
    current: u32,
    max: u32,
}

impl Health {
    pub fn new(points: u32) -> Self {
        Health {
            current: points,
            max: points,
        }
    }

    /// Health points left
    pub fn current(&self) -> u32 {
        self.current
    }

    /// Health points the entity started with
    pub fn max(&self) -> u32 {
        self.max
    }

    /// Remove health points, stopping at zero
    pub fn take_damage(&mut self, points: u32) {
        self.current = self.current.saturating_sub(points);
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_remove_test() {
        let mut health = Health::new(5);
        health.take_damage(1);

        assert_eq!(health.current(), 4);
        assert_eq!(health.is_dead(), false);
    }

    #[test]
    fn max_health_kept_after_damage_test() {
        let mut health = Health::new(5);
        health.take_damage(2);

        assert_eq!(health.max(), 5);
    }

    #[test]
    fn health_underflow_test() {
        let mut health = Health::new(5);
        health.take_damage(5000);

        assert_eq!(health.current(), 0);
    }

    #[test]
    fn death_after_all_health_removed_test() {
        let mut health = Health::new(3);

        for _ in 0..3 {
            health.take_damage(1);
        }

        assert_eq!(health.is_dead(), true);
    }

    #[test]
    fn alive_with_health_left_test() {
        assert_eq!(Health::new(1).is_dead(), false);
    }
}
