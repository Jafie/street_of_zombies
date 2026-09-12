use crate::game_entity::projectiles::Projectile;
use crate::weapons::MachineGun;
use crate::weapons::Weapon;

impl Weapon for MachineGun {
    fn new(projectile_speed: f32, fire_rate: f32, amo: u32, fire_distance: u32) -> Self {
        MachineGun {
            speed: projectile_speed,
            current_amo: 0,
            max_amo: amo,
            limit_of_fire: fire_distance,
            initial_fire_rate: fire_rate,
            current_fire_rate_timer: 0.0,
        }
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

    fn create_projectile(
        &self,
        direction_to_set: (f32, f32),
        initial_position_to_set: (f32, f32),
        is_ennemy: bool,
    ) -> Projectile {
        Projectile::new(
            self.speed,
            direction_to_set,
            initial_position_to_set,
            self.limit_of_fire,
            is_ennemy,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_gun_reload_test() {
        let mut machine_gun = MachineGun::new(1200.0, 0.06, 800, 700);
        assert_eq!(machine_gun.get_amo(), 0);

        machine_gun.reload();
        assert_eq!(machine_gun.get_amo(), 800);
    }

    #[test]
    fn machine_gun_fire_rate_test() {
        let mut machine_gun = MachineGun::new(1200.0, 0.06, 800, 700);

        // The timer starts at 0: the first shot is immediate
        assert_eq!(machine_gun.is_ready_to_fire(0.01), true);
        // 0.03s elapsed of the 0.06s fire rate
        assert_eq!(machine_gun.is_ready_to_fire(0.03), false);
        // 0.07s elapsed
        assert_eq!(machine_gun.is_ready_to_fire(0.04), true);
    }
}
