//! The kinds of bonus, and what each of them gives.

use bevy::prelude::*;

use crate::combat::WeaponStats;

const MACHINE_GUN_BONUS_DURATION: f32 = 10.0;

/// The kind of bonus given when a bonus drop is picked up
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BonusKind {
    /// Higher fire rate and faster projectiles
    MachineGun,
}

impl BonusKind {
    /// How long the bonus lasts once picked up, in seconds
    pub fn duration(self) -> f32 {
        match self {
            BonusKind::MachineGun => MACHINE_GUN_BONUS_DURATION,
        }
    }

    /// The name of the bonus, displayed in the HUD
    pub fn name(self) -> &'static str {
        match self {
            BonusKind::MachineGun => "MACHINE GUN",
        }
    }

    /// The color of the bonus drop on the map
    pub fn color(self) -> Color {
        match self {
            BonusKind::MachineGun => Color::srgb(1.0, 0.84, 0.0),
        }
    }

    /// The weapon held while the bonus lasts
    pub fn weapon(self) -> WeaponStats {
        match self {
            BonusKind::MachineGun => WeaponStats::MACHINE_GUN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_gun_bonus_duration() {
        assert_eq!(BonusKind::MachineGun.duration(), MACHINE_GUN_BONUS_DURATION);
    }

    #[test]
    fn machine_gun_bonus_gives_machine_gun() {
        assert_eq!(BonusKind::MachineGun.weapon(), WeaponStats::MACHINE_GUN);
    }
}
