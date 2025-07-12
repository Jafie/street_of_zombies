use crate::game_entity::MoveableSprite;
use crate::game_entity::MoveableSpriteTrait;

use crate::game_system::math_and_generator;

use bevy::ecs::component::Component;

static DEFAULT_PROJECTILE_HITBOX: (f32, f32) = (10., 10.);

struct ProjectileInternalData {
    projectile_limit_distance: u32,
    is_from_ennemy: bool,
}

#[derive(Component)]
pub struct Projectile {
    sprite_data: MoveableSprite,
    internal_data: ProjectileInternalData,
}

impl MoveableSpriteTrait for Projectile {
    fn get_moveable_interface(&self) -> &MoveableSprite {
        &self.sprite_data
    }

    fn get_moveable_interface_mut(&mut self) -> &mut MoveableSprite {
        &mut self.sprite_data
    }
}

impl Projectile {
    /// Returns a new Projectile object - A projectile object contains all the information dedicated to a single projectile
    ///
    /// # Arguments
    ///
    /// * `speed_to_set` - The speed of the projectile
    /// * `direction_to_set` - The direction of the projectile
    /// * `current_position_to_set` - The initial position of the projectile
    /// * `limit_of_fire` - The distance limit of the projectile. Will diseapear when this distance is reached.
    /// * `is_from_ennemy` - True if the projectile comes from an ennemy. Else, it comes from the player
    /// # Examples
    ///
    /// ```
    ///     let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
    /// ```
    pub fn new(
        speed_to_set: f32,
        direction_to_set: (f32, f32),
        current_position_to_set: (f32, f32),
        limit_of_fire: u32,
        is_from_ennemy: bool,
    ) -> Self {
        Projectile {
            internal_data: ProjectileInternalData {
                projectile_limit_distance: limit_of_fire,
                is_from_ennemy: is_from_ennemy,
            },
            sprite_data: MoveableSprite::new(
                speed_to_set,
                direction_to_set,
                current_position_to_set,
                DEFAULT_PROJECTILE_HITBOX,
            ),
        }
    }

    /// Check if the projectile reached its "limit_of_fire"
    ///
    /// # Examples
    ///
    /// ```
    ///    let mut projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
    ///    projectile.set_new_position((600., 600.));
    ///    assert_eq!(projectile.is_out_of_distance(), true);
    /// ```
    pub fn is_out_of_distance(&self) -> bool {
        let distance_walked = math_and_generator::calculate_cartesian_distance(
            self.sprite_data.internal_data.initial_position,
            self.sprite_data.internal_data.current_position,
        );

        let result = distance_walked > (self.internal_data.projectile_limit_distance as f32);
        result
    }

    /// True if the projectile comes from an ennemy. Else, it comes from the player
    ///
    /// # Examples
    ///
    /// ```
    ///    let mut projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
    ///    assert_eq!(projectile.is_coming_from_ennemy(), false);
    /// ```
    pub fn is_coming_from_ennemy(&self) -> bool {
        self.internal_data.is_from_ennemy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn projectile_out_of_fire_position() {
        let mut projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
        projectile.sprite_data.set_new_position((600., 600.));
        assert_eq!(projectile.is_out_of_distance(), true);
    }

    #[test]
    fn projectile_still_in_fire() {
        let mut projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
        projectile.sprite_data.set_new_position((50., 20.));
        assert_eq!(projectile.is_out_of_distance(), false);
    }

    #[test]
    fn projectile_is_launched_by_ennemy() {
        let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, true);
        assert_eq!(projectile.is_coming_from_ennemy(), true);
    }

    #[test]
    fn projectile_is_launched_by_player() {
        let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
        assert_eq!(projectile.is_coming_from_ennemy(), false);
    }
}
