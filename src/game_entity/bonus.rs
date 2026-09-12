use crate::game_entity::MoveableSprite;
use crate::game_entity::MoveableSpriteTrait;

use bevy::prelude::*;
use bevy::ecs::component::Component;

static MACHINE_GUN_BONUS_DURATION: f32 = 10.0;
static BONUS_TIME_ON_GROUND: f32 = 8.0;
static BONUS_HITBOX_SIZE: (f32, f32) = (20., 20.);

/// The kind of bonus given to the player when a bonus drop is picked up
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BonusKind {
    /// Higher fire rate and faster projectiles
    MachineGun,
}

impl BonusKind {
    /// Get the duration (in seconds) of the bonus once picked up
    pub fn get_duration(&self) -> f32 {
        match self {
            BonusKind::MachineGun => MACHINE_GUN_BONUS_DURATION,
        }
    }

    /// Get the name of the bonus, displayed in the scoreboard
    pub fn get_name(&self) -> &'static str {
        match self {
            BonusKind::MachineGun => "MACHINE GUN",
        }
    }

    /// Get the color of the bonus drop on the map
    pub fn get_color(&self) -> Color {
        match self {
            BonusKind::MachineGun => Color::srgb(1.0, 0.84, 0.0),
        }
    }
}

/// A bonus drop waiting on the map to be picked up by the player
#[derive(Component)]
pub struct Bonus {
    sprite_data: MoveableSprite,
    kind: BonusKind,
    remaining_time_on_ground: f32,
}

impl MoveableSpriteTrait for Bonus {
    fn get_moveable_interface(&self) -> &MoveableSprite {
        &self.sprite_data
    }

    fn get_moveable_interface_mut(&mut self) -> &mut MoveableSprite {
        &mut self.sprite_data
    }
}

impl Bonus {
    /// Returns a new Bonus drop, which does not move
    ///
    /// # Arguments
    ///
    /// * `kind` - The bonus given to the player on pick-up
    /// * `position` - The position of the drop on the game area
    /// # Examples
    ///
    /// ```
    ///     let bonus = Bonus::new(BonusKind::MachineGun, (15., 20.));
    /// ```
    pub fn new(kind: BonusKind, position: (f32, f32)) -> Self {
        Bonus {
            sprite_data: MoveableSprite::new(0.0, (0.0, 0.0), position, BONUS_HITBOX_SIZE),
            kind,
            remaining_time_on_ground: BONUS_TIME_ON_GROUND,
        }
    }

    /// Get the bonus given to the player on pick-up
    pub fn get_kind(&self) -> BonusKind {
        self.kind
    }

    /// Reduce the time left on the ground. Return true when the drop has expired
    ///
    /// # Arguments
    ///
    /// * `delta_secs` - The time elapsed since the last call
    pub fn tick_on_ground(&mut self, delta_secs: f32) -> bool {
        self.remaining_time_on_ground -= delta_secs;
        self.remaining_time_on_ground <= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_initial_position_and_hitbox() {
        let bonus = Bonus::new(BonusKind::MachineGun, (15., 20.));

        assert_eq!(bonus.get_moveable_interface().get_position(), (15., 20.));
        assert_eq!(bonus.get_moveable_interface().get_hitbox_size(), BONUS_HITBOX_SIZE);
        assert_eq!(bonus.get_kind(), BonusKind::MachineGun);
    }

    #[test]
    fn bonus_not_expired_before_time_on_ground() {
        let mut bonus = Bonus::new(BonusKind::MachineGun, (15., 20.));

        assert_eq!(bonus.tick_on_ground(BONUS_TIME_ON_GROUND - 0.1), false);
    }

    #[test]
    fn bonus_expired_after_time_on_ground() {
        let mut bonus = Bonus::new(BonusKind::MachineGun, (15., 20.));

        bonus.tick_on_ground(BONUS_TIME_ON_GROUND - 0.1);
        assert_eq!(bonus.tick_on_ground(0.2), true);
    }

    #[test]
    fn machine_gun_bonus_duration() {
        assert_eq!(BonusKind::MachineGun.get_duration(), MACHINE_GUN_BONUS_DURATION);
    }
}
