use crate::moveable_sprites::projectiles::Projectile;
use crate::weapons::Weapon;
use crate::weapons::Pistol;


impl Weapon for Pistol {
    fn new(projectile_speed: f32, fire_rate: f32, amo: u32, fire_distance: u32) -> Self {
        Pistol {speed: projectile_speed,
                current_amo: amo,
                max_amo: amo,
                limit_of_fire: fire_distance,
                initial_fire_rate: fire_rate,
                current_fire_rate_timer: 0.0}
    }

    fn reload(&mut self) {
        self.current_amo = self.max_amo;
    }

    fn get_amo(&self) -> u32 {
        self.current_amo
    }

    fn is_ready_to_fire(&mut self, time_elapsed_since_last_update: f32) -> bool {
        self.current_fire_rate_timer -= time_elapsed_since_last_update;

        if self.current_fire_rate_timer < 0.0 {
            self.current_fire_rate_timer = self.initial_fire_rate;
            return true;
        } 

        false
    }

    fn reduce_amo(&mut self) {
        self.current_amo -= 1;
    }

    fn create_projectile(&self, direction_to_set: (f32, f32), initial_position_to_set: (f32, f32), is_ennemy: bool) -> Projectile {
        Projectile::new(self.speed, direction_to_set, initial_position_to_set, self.limit_of_fire, is_ennemy)
    }
}
