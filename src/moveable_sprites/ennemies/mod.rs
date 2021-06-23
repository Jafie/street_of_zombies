use crate::moveable_sprites::MoveableSprite;
use crate::weapons::Weapon;
use crate::weapons::Pistol;

/// The Main Character. Controllable by the player.
pub struct Ennemy {
    speed: f32,
    current_position : (f32, f32),
    direction: (f32, f32),
    _current_weapon: Box<dyn Weapon + Send + Sync>
}

impl MoveableSprite for Ennemy {
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

impl Ennemy {
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), initial_pos: (f32, f32)) -> Self {
        Ennemy {
             speed: speed_to_set,
             current_position: initial_pos,
             direction: direction_to_set,
             _current_weapon: Box::new(Pistol::new())}
    }
}
