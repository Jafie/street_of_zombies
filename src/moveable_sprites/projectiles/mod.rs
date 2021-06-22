use crate::moveable_sprites::MoveableSprite;

pub struct Projectile {
    speed: f32,
    direction: (f32, f32),
    initial_position : (f32, f32),
    current_position : (f32, f32),
    projectile_limit_distance: u32,
}

impl Projectile {
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), current_position_to_set: (f32, f32), limit_of_fire: u32) -> Self {
        Projectile {
            speed: speed_to_set,
            direction: direction_to_set,
            initial_position: current_position_to_set,
            current_position: current_position_to_set,
            projectile_limit_distance: limit_of_fire
        }
    }

    /// Return true if the Projectile browsed more than the "projectile_limit_distance"
    pub fn is_out_of_distance(&self) -> bool {
        let (initial_pos_x, initial_pos_y) = self.initial_position;
        let (current_pos_x, current_pos_y) = self.current_position;

        let position_diff_x = (current_pos_x - initial_pos_x) as i32;
        let position_diff_y = (current_pos_y - initial_pos_y) as i32;
        let distance_walked_squared = (position_diff_x.pow(2) + position_diff_y.pow(2)) as u32;

        let result = distance_walked_squared > self.projectile_limit_distance.pow(2);
        result
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
    fn get_sprite_name() -> String {
        String::from("Amo")
    }
}
