//! Axis-aligned hitboxes and the contacts between them.

use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

/// The box, centered on the entity position, in which the entity can be touched.
/// Holds the full width and height.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Hitbox(pub Vec2);

impl Hitbox {
    /// True when this hitbox, at `position`, overlaps `other` at `other_position`
    pub fn collides_with(&self, position: Vec2, other: &Hitbox, other_position: Vec2) -> bool {
        self.bounds_at(position)
            .intersects(&other.bounds_at(other_position))
    }

    fn bounds_at(&self, position: Vec2) -> Aabb2d {
        // Hitboxes store full sizes; Aabb2d expects half sizes.
        Aabb2d::new(position, self.0 / 2.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENNEMY_HITBOX: Hitbox = Hitbox(Vec2::new(40., 50.));
    const PLAYER_HITBOX: Hitbox = Hitbox(Vec2::new(35., 40.));
    const PROJECTILE_HITBOX: Hitbox = Hitbox(Vec2::new(10., 10.));

    #[test]
    fn hitboxes_at_same_position_collide() {
        let position = Vec2::new(15., 20.);

        assert_eq!(
            ENNEMY_HITBOX.collides_with(position, &PLAYER_HITBOX, position),
            true
        );
    }

    #[test]
    fn projectile_just_outside_hitbox_not_collides() {
        // Ennemy is 40 wide and projectile 10 wide, so edges touch at an x gap of 25.
        assert_eq!(
            ENNEMY_HITBOX.collides_with(Vec2::ZERO, &PROJECTILE_HITBOX, Vec2::new(30., 0.)),
            false
        );
    }

    #[test]
    fn projectile_just_inside_hitbox_collides() {
        assert_eq!(
            ENNEMY_HITBOX.collides_with(Vec2::ZERO, &PROJECTILE_HITBOX, Vec2::new(20., 0.)),
            true
        );
    }

    #[test]
    fn distant_hitboxes_not_collide() {
        assert_eq!(
            ENNEMY_HITBOX.collides_with(
                Vec2::new(15., 20.),
                &PROJECTILE_HITBOX,
                Vec2::new(150., 2000.)
            ),
            false
        );
    }
}
