//! Weapons: what they fire, how often, and how many shots they hold.

use bevy::prelude::*;

/// What a weapon fires and how often. The constants are the weapons of the game.
///
/// To add a weapon, add a constant here and give it to an entity with `Weapon::new`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeaponStats {
    /// Speed of the projectiles
    pub projectile_speed: f32,
    /// Minimum time between two shots, in seconds
    pub fire_rate: f32,
    /// Shots available after a reload
    pub max_amo: u32,
    /// Distance travelled by a projectile before it disappears
    pub range: f32,
}

impl WeaponStats {
    /// Single fire. The player's weapon when no bonus is active.
    pub const PISTOL: Self = WeaponStats {
        projectile_speed: 700.0,
        fire_rate: 0.18,
        max_amo: 800,
        range: 700.0,
    };

    /// Rapid fire, given by the machine gun bonus
    pub const MACHINE_GUN: Self = WeaponStats {
        projectile_speed: 1200.0,
        fire_rate: 0.06,
        max_amo: 800,
        range: 700.0,
    };

    /// Slow projectiles and a few shots per reload. Held by the ennemies.
    pub const ZOMBIE_PISTOL: Self = WeaponStats {
        projectile_speed: 300.0,
        fire_rate: 0.5,
        max_amo: 3,
        range: 500.0,
    };
}

/// A weapon held by an entity
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Weapon {
    stats: WeaponStats,
    amo: u32,
    /// Time left before the next shot is allowed
    cooldown: f32,
}

impl Weapon {
    /// A new weapon. It starts empty: reload it before firing.
    pub fn new(stats: WeaponStats) -> Self {
        Weapon {
            stats,
            amo: 0,
            cooldown: 0.0,
        }
    }

    /// A new weapon, reloaded and ready to fire
    pub fn loaded(stats: WeaponStats) -> Self {
        let mut weapon = Weapon::new(stats);
        weapon.reload();
        weapon
    }

    pub fn stats(&self) -> WeaponStats {
        self.stats
    }

    /// Refill the weapon
    pub fn reload(&mut self) {
        self.amo = self.stats.max_amo;
    }

    /// Pull the trigger for a frame lasting `delta_secs`. True when a shot is fired: the
    /// weapon has amo left and its fire rate allows a new shot. Firing consumes one amo.
    ///
    /// The fire rate cooldown only runs down while the trigger is pulled with amo left.
    pub fn try_fire(&mut self, delta_secs: f32) -> bool {
        if self.amo == 0 || !self.is_ready_to_fire(delta_secs) {
            return false;
        }

        self.amo -= 1;
        true
    }

    fn is_ready_to_fire(&mut self, delta_secs: f32) -> bool {
        self.cooldown -= delta_secs;

        if self.cooldown < 0.0 {
            self.cooldown = self.stats.fire_rate;
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_weapon_is_empty_test() {
        let mut weapon = Weapon::new(WeaponStats::MACHINE_GUN);

        assert_eq!(weapon.amo, 0);
        assert_eq!(weapon.try_fire(1.0), false);
    }

    #[test]
    fn weapon_reload_test() {
        let mut weapon = Weapon::new(WeaponStats::MACHINE_GUN);
        weapon.reload();

        assert_eq!(weapon.amo, WeaponStats::MACHINE_GUN.max_amo);
    }

    #[test]
    fn loaded_weapon_is_full_test() {
        assert_eq!(
            Weapon::loaded(WeaponStats::PISTOL).amo,
            WeaponStats::PISTOL.max_amo
        );
    }

    #[test]
    fn fire_consumes_amo_test() {
        let mut weapon = Weapon::loaded(WeaponStats::ZOMBIE_PISTOL);

        assert_eq!(weapon.try_fire(0.01), true);
        assert_eq!(weapon.amo, WeaponStats::ZOMBIE_PISTOL.max_amo - 1);
    }

    #[test]
    fn weapon_empty_after_all_shots_test() {
        let mut weapon = Weapon::loaded(WeaponStats::ZOMBIE_PISTOL);

        for _ in 0..WeaponStats::ZOMBIE_PISTOL.max_amo {
            assert_eq!(weapon.try_fire(1.0), true);
        }

        assert_eq!(weapon.try_fire(1.0), false);
    }

    #[test]
    fn machine_gun_fire_rate_test() {
        let mut weapon = Weapon::loaded(WeaponStats::MACHINE_GUN);

        // The cooldown starts at 0: the first shot is immediate
        assert_eq!(weapon.try_fire(0.01), true);
        // 0.03s elapsed of the 0.06s fire rate
        assert_eq!(weapon.try_fire(0.03), false);
        // 0.07s elapsed
        assert_eq!(weapon.try_fire(0.04), true);
    }

    #[test]
    fn pistol_fires_slower_than_machine_gun_test() {
        let mut pistol = Weapon::loaded(WeaponStats::PISTOL);
        let mut machine_gun = Weapon::loaded(WeaponStats::MACHINE_GUN);
        pistol.try_fire(0.01);
        machine_gun.try_fire(0.01);

        // 0.1s is longer than the machine gun fire rate, shorter than the pistol one
        assert_eq!(machine_gun.try_fire(0.1), true);
        assert_eq!(pistol.try_fire(0.1), false);
    }
}
