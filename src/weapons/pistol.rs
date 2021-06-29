use crate::moveable_sprites::projectiles::Projectile;
use crate::weapons::Weapon;
use crate::weapons::Pistol;

// Pistol data
static PROJECTILE_SPEED: f32 = 500.0;
static AMO_IN_WEAPON: u32 = 10;
static LIMIT_OF_FIRE: u32 = 500;
static FIRE_RATE: f32 = 0.2;


impl Weapon for Pistol {
    fn new() -> Self {
        Pistol {speed: PROJECTILE_SPEED,
                amo: AMO_IN_WEAPON,
                limit_of_fire: LIMIT_OF_FIRE,
                initial_fire_rate: FIRE_RATE,
                current_fire_rate_timer: 0.0}
    }

    fn reload(&mut self) {
        self.amo = AMO_IN_WEAPON;
    }

    fn get_amo(&self) -> u32 {
        self.amo
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
        self.amo -= 1;
    }

    fn create_projectile(&self, direction_to_set: (f32, f32), initial_position_to_set: (f32, f32)) -> Projectile {
        Projectile::new(self.speed, direction_to_set, initial_position_to_set, self.limit_of_fire)
    }
}