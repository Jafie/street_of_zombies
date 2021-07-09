use crate::game_entity::MoveableSprite;
use crate::game_system::math_and_generator;


static DEFAULT_PROJECTILE_HITBOX: (f32, f32) = (10., 10.);

struct ProjectileInternalData {
    speed: f32,
    direction: (f32, f32),
    initial_position : (f32, f32),
    current_position : (f32, f32),
    projectile_limit_distance: u32,
    hitbox_size: (f32, f32),
    is_from_ennemy: bool
}

pub struct Projectile {
    internal_data: ProjectileInternalData
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
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), current_position_to_set: (f32, f32), limit_of_fire: u32, is_from_ennemy: bool) -> Self {
        Projectile { internal_data: ProjectileInternalData {
                speed: speed_to_set,
                direction: direction_to_set,
                initial_position: current_position_to_set,
                current_position: current_position_to_set,
                projectile_limit_distance: limit_of_fire,
                hitbox_size: DEFAULT_PROJECTILE_HITBOX,
                is_from_ennemy: is_from_ennemy
            }
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
        let distance_walked = math_and_generator::calculate_cartesian_distance(self.internal_data.initial_position, self.internal_data.current_position);

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

impl MoveableSprite for Projectile {
    fn get_speed(&self) -> f32 {
        self.internal_data.speed
    }
    fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.internal_data.direction = direction;
    }
    fn get_direction(&self) -> (f32, f32) {
        self.internal_data.direction
    }
    fn get_position(&self) -> (f32, f32) {
        self.internal_data.current_position
    }
    fn set_new_position(&mut self, position: (f32, f32)) {
        self.internal_data.current_position = position;
    }
    fn get_hitbox_size(&self) -> (f32, f32) {
        self.internal_data.hitbox_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projectile_initial_speed() {
        let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);

        assert_eq!(projectile.get_speed(), 500.0);
    }

    #[test]
    fn projectile_initial_direction() {
        let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);

        assert_eq!(projectile.get_direction(), (5., 10.));
    }

    #[test]
    fn projectile_initial_position() {
        let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);

        assert_eq!(projectile.get_position(), (15., 20.));
    }

    #[test]
    fn projectile_set_direction() {
        let mut projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
        projectile.set_new_direction((100., 45.));
        assert_eq!(projectile.get_direction(), (100., 45.));
    }

    #[test]
    fn projectile_set_position() {
        let mut projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
        projectile.set_new_position((60., 40.));
        assert_eq!(projectile.get_position(), (60., 40.));
    }
    
    #[test]
    fn projectile_get_hitbox_size() {
        let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
        let hitbox_size = projectile.get_hitbox_size();
        assert_eq!(hitbox_size, DEFAULT_PROJECTILE_HITBOX);
    }

    #[test]
    fn projectile_out_of_fire_position() {
        let mut projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
        projectile.set_new_position((600., 600.));
        assert_eq!(projectile.is_out_of_distance(), true);
    }

    #[test]
    fn projectile_still_in_fire() {
        let mut projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
        projectile.set_new_position((50., 20.));
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