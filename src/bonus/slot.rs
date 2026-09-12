//! The timed bonus an entity benefits from, and giving back what it replaced when it ends.

use bevy::prelude::*;

use crate::bonus::BonusKind;
use crate::combat::{Weapon, WeaponStats};

/// The bonus an entity currently benefits from, if any.
/// Only entities with a slot can pick up bonus drops.
#[derive(Component, Debug, Default)]
pub struct BonusSlot {
    active: Option<ActiveBonus>,
}

#[derive(Debug)]
struct ActiveBonus {
    kind: BonusKind,
    remaining_time: f32,
    /// The weapon held before the bonus, given back when it ends
    replaced_weapon: WeaponStats,
}

impl BonusSlot {
    /// Start a bonus. Picking up a bonus while one is active restarts the timer.
    pub fn apply(&mut self, kind: BonusKind, weapon: &mut Weapon) {
        // When a bonus is already active, the weapon held is the bonus one: keep the original
        let replaced_weapon = match &self.active {
            Some(active_bonus) => active_bonus.replaced_weapon,
            None => weapon.stats(),
        };

        // Loaded: weapons start empty, and the player only reloads while not firing
        *weapon = Weapon::loaded(kind.weapon());

        self.active = Some(ActiveBonus {
            kind,
            remaining_time: kind.duration(),
            replaced_weapon,
        });
    }

    /// Reduce the time left of the active bonus. When it ends, the replaced weapon is given
    /// back, loaded.
    pub fn update(&mut self, delta_secs: f32, weapon: &mut Weapon) {
        let Some(active_bonus) = &mut self.active else {
            return;
        };

        active_bonus.remaining_time -= delta_secs;

        if active_bonus.remaining_time <= 0.0 {
            *weapon = Weapon::loaded(active_bonus.replaced_weapon);
            self.active = None;
        }
    }

    /// The name of the active bonus and its remaining time, in seconds rounded up
    pub fn status(&self) -> Option<(&'static str, u32)> {
        self.active.as_ref().map(|active_bonus| {
            (
                active_bonus.kind.name(),
                active_bonus.remaining_time.ceil() as u32,
            )
        })
    }
}

/// Count down every active bonus
pub(super) fn bonus_countdown_system(
    time: Res<Time>,
    mut query: Query<(&mut BonusSlot, &mut Weapon)>,
) {
    for (mut bonus_slot, mut weapon) in query.iter_mut() {
        bonus_slot.update(time.delta_secs(), &mut weapon);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pistol() -> Weapon {
        Weapon::new(WeaponStats::PISTOL)
    }

    #[test]
    fn slot_without_bonus_test() {
        assert_eq!(BonusSlot::default().status(), None);
    }

    #[test]
    fn bonus_applied_test() {
        let mut bonus_slot = BonusSlot::default();
        bonus_slot.apply(BonusKind::MachineGun, &mut pistol());

        assert_eq!(bonus_slot.status(), Some(("MACHINE GUN", 10)));
    }

    #[test]
    fn bonus_countdown_test() {
        let mut bonus_slot = BonusSlot::default();
        let mut weapon = pistol();
        bonus_slot.apply(BonusKind::MachineGun, &mut weapon);
        bonus_slot.update(4.0, &mut weapon);

        assert_eq!(bonus_slot.status(), Some(("MACHINE GUN", 6)));
    }

    #[test]
    fn bonus_ended_test() {
        let mut bonus_slot = BonusSlot::default();
        let mut weapon = pistol();
        bonus_slot.apply(BonusKind::MachineGun, &mut weapon);
        bonus_slot.update(BonusKind::MachineGun.duration() + 0.5, &mut weapon);

        assert_eq!(bonus_slot.status(), None);
    }

    #[test]
    fn bonus_pickup_restarts_timer_test() {
        let mut bonus_slot = BonusSlot::default();
        let mut weapon = pistol();
        bonus_slot.apply(BonusKind::MachineGun, &mut weapon);
        bonus_slot.update(7.0, &mut weapon);
        bonus_slot.apply(BonusKind::MachineGun, &mut weapon);

        assert_eq!(bonus_slot.status(), Some(("MACHINE GUN", 10)));
    }

    #[test]
    fn bonus_weapon_loaded_on_pickup_test() {
        let mut bonus_slot = BonusSlot::default();
        let mut weapon = pistol();
        bonus_slot.apply(BonusKind::MachineGun, &mut weapon);

        assert_eq!(weapon, Weapon::loaded(WeaponStats::MACHINE_GUN));
    }

    #[test]
    fn replaced_weapon_restored_loaded_after_bonus_test() {
        let mut bonus_slot = BonusSlot::default();
        let mut weapon = pistol();
        bonus_slot.apply(BonusKind::MachineGun, &mut weapon);
        bonus_slot.update(BonusKind::MachineGun.duration() + 0.5, &mut weapon);

        assert_eq!(weapon, Weapon::loaded(WeaponStats::PISTOL));
    }

    #[test]
    fn original_weapon_restored_after_repeated_pickups_test() {
        let mut bonus_slot = BonusSlot::default();
        let mut weapon = pistol();
        bonus_slot.apply(BonusKind::MachineGun, &mut weapon);
        bonus_slot.apply(BonusKind::MachineGun, &mut weapon);
        bonus_slot.update(BonusKind::MachineGun.duration() + 0.5, &mut weapon);

        assert_eq!(weapon, Weapon::loaded(WeaponStats::PISTOL));
    }
}
