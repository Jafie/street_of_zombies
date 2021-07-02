use crate::moveable_sprites::MoveableSprite;
use crate::math_cartesian;


pub struct Projectile {
    speed: f32,
    direction: (f32, f32),
    initial_position : (f32, f32),
    current_position : (f32, f32),
    projectile_limit_distance: u32,
    is_from_ennemy: bool
}

impl Projectile {
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), current_position_to_set: (f32, f32), limit_of_fire: u32, is_from_ennemy: bool) -> Self {
        Projectile {
            speed: speed_to_set,
            direction: direction_to_set,
            initial_position: current_position_to_set,
            current_position: current_position_to_set,
            projectile_limit_distance: limit_of_fire,
            is_from_ennemy: is_from_ennemy
        }
    }

    /// Return true if the Projectile browsed more than the "projectile_limit_distance"
    pub fn is_out_of_distance(&self) -> bool {
        let distance_walked = math_cartesian::calculate_cartesian_distance(self.initial_position, self.current_position);

        let result = distance_walked > (self.projectile_limit_distance as f32);
        result
    }

    pub fn is_coming_from_ennemy(&self) -> bool {
        self.is_from_ennemy
    }
}

impl MoveableSprite for Projectile {
    fn get_speed(&self) -> f32 {
        self.speed
    }
    fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.direction = direction;
    }
    fn get_direction(&self) -> (f32, f32) {
        self.direction
    }
    fn get_position(&self) -> (f32, f32) {
        self.current_position
    }
    fn set_new_position(&mut self, position: (f32, f32)) {
        self.current_position = position;
    }
}
