//! Ennemy behavior: wander around the spawn position and shoot all the time.

use bevy::prelude::*;

use crate::combat::{fire_weapon, Faction, Weapon};
use crate::game::{is_outside_arena, GameSet};
use crate::math::calculate_cartesian_distance;
use crate::physics::Movement;

/// Distance from its spawn position at which an ennemy turns back
const MAXIMUM_WANDER_DISTANCE: f32 = 300.0;
/// Time between two reloads of an ennemy weapon, in seconds
const RELOAD_COOLDOWN_SECS: f32 = 2.5;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        // Turn back first: the attack is aimed in the direction the ennemy now heads
        (turn_back_system, attack_system)
            .chain()
            .in_set(GameSet::Behavior),
    );
}

/// Keeps an ennemy close to where it spawned
#[derive(Component)]
pub(super) struct Wander {
    origin: Vec2,
}

impl Wander {
    pub(super) fn new(origin: Vec2) -> Self {
        Wander { origin }
    }

    /// True when the ennemy shall head back: it went too far from its origin, or is about to
    /// leave the play area
    fn should_turn_back(&self, position: Vec2, direction: Vec2) -> bool {
        calculate_cartesian_distance(self.origin, position) > MAXIMUM_WANDER_DISTANCE
            || is_outside_arena(position + direction)
    }
}

/// Reloads an ennemy weapon at a fixed interval, whether it fired or not
#[derive(Component, Default)]
pub(super) struct ReloadCooldown {
    elapsed: f32,
}

impl ReloadCooldown {
    /// Let time pass. True when the weapon shall be reloaded.
    fn tick(&mut self, delta_secs: f32) -> bool {
        self.elapsed += delta_secs;

        if self.elapsed > RELOAD_COOLDOWN_SECS {
            self.elapsed = 0.;
            return true;
        }

        false
    }
}

fn turn_back_system(mut query: Query<(&Wander, &Transform, &mut Movement)>) {
    for (wander, transform, mut movement) in query.iter_mut() {
        if wander.should_turn_back(transform.translation.truncate(), movement.direction()) {
            movement.reverse();
        }
    }
}

fn attack_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &Movement, &Faction, &mut Weapon, &mut ReloadCooldown)>,
) {
    for (transform, movement, faction, mut weapon, mut reload_cooldown) in query.iter_mut() {
        fire_weapon(
            &mut commands,
            &mut weapon,
            time.delta_secs(),
            *faction,
            transform.translation.truncate(),
            movement.direction(),
        );

        if reload_cooldown.tick(time.delta_secs()) {
            weapon.reload();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ennemy_near_origin_keeps_heading() {
        let wander = Wander::new(Vec2::new(15., 20.));

        assert_eq!(wander.should_turn_back(Vec2::new(60., 40.), Vec2::X), false);
    }

    #[test]
    fn ennemy_far_from_origin_turns_back() {
        let wander = Wander::new(Vec2::new(-150., 0.));

        assert_eq!(wander.should_turn_back(Vec2::new(200., 0.), Vec2::X), true);
    }

    #[test]
    fn ennemy_leaving_arena_turns_back() {
        let arena_edge = crate::game::clamp_to_arena(Vec2::new(0., 5000.));
        let wander = Wander::new(arena_edge);

        assert_eq!(wander.should_turn_back(arena_edge, Vec2::Y), true);
    }

    #[test]
    fn reload_waits_for_cooldown() {
        let mut reload_cooldown = ReloadCooldown::default();

        assert_eq!(reload_cooldown.tick(RELOAD_COOLDOWN_SECS - 0.1), false);
        assert_eq!(reload_cooldown.tick(0.2), true);
    }

    #[test]
    fn reload_cooldown_restarts_after_reload() {
        let mut reload_cooldown = ReloadCooldown::default();
        reload_cooldown.tick(RELOAD_COOLDOWN_SECS + 0.1);

        assert_eq!(reload_cooldown.tick(0.2), false);
    }
}
