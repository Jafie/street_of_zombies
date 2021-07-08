use crate::game_entity::MoveableSprite;
use crate::game_system::math_cartesian;


struct ProjectileInternalData {
    speed: f32,
    direction: (f32, f32),
    initial_position : (f32, f32),
    current_position : (f32, f32),
    projectile_limit_distance: u32,
    is_from_ennemy: bool
}

pub struct Projectile {
    internal_data: ProjectileInternalData
}

impl Projectile {
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), current_position_to_set: (f32, f32), limit_of_fire: u32, is_from_ennemy: bool) -> Self {
        Projectile { internal_data: ProjectileInternalData {
                speed: speed_to_set,
                direction: direction_to_set,
                initial_position: current_position_to_set,
                current_position: current_position_to_set,
                projectile_limit_distance: limit_of_fire,
                is_from_ennemy: is_from_ennemy
            }
        }
    }

    /// Return true if the Projectile browsed more than the "projectile_limit_distance"
    pub fn is_out_of_distance(&self) -> bool {
        let distance_walked = math_cartesian::calculate_cartesian_distance(self.internal_data.initial_position, self.internal_data.current_position);

        let result = distance_walked > (self.internal_data.projectile_limit_distance as f32);
        result
    }

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
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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
}