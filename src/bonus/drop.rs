//! Bonus drops: they appear at random, wait on the ground, and are picked up on contact.

use bevy::prelude::*;
use rand::Rng;

use crate::bonus::{BonusKind, BonusSlot};
use crate::combat::Weapon;
use crate::game::{despawn_on_restart, random_position_in_arena};
use crate::physics::Hitbox;

const BONUS_TIME_ON_GROUND: f32 = 8.0;
/// Drops are drawn as a plain square filling their hitbox
const BONUS_DROP_SIZE: Vec2 = Vec2::new(20., 20.);
const MAXIMUM_NUMBER_OF_BONUS_ON_MAP: usize = 1;
const BONUS_SPAWN_CHANCE_PER_SECOND: f32 = 0.05;

/// A bonus drop waiting on the map to be picked up
#[derive(Component, Debug)]
pub(super) struct BonusDrop {
    kind: BonusKind,
    remaining_time_on_ground: f32,
}

impl BonusDrop {
    fn new(kind: BonusKind) -> Self {
        BonusDrop {
            kind,
            remaining_time_on_ground: BONUS_TIME_ON_GROUND,
        }
    }

    /// Reduce the time left on the ground. True when the drop has expired.
    fn tick_on_ground(&mut self, delta_secs: f32) -> bool {
        self.remaining_time_on_ground -= delta_secs;
        self.remaining_time_on_ground <= 0.0
    }
}

/// True when a new bonus drop shall be spawned this frame
///
/// # Arguments
///
/// * `roll` - A random number in the range [0, 1)
/// * `delta_secs` - The time elapsed since the last frame, so the spawn chance does not depend on the frame rate
/// * `bonus_on_map` - The number of bonus drops currently on the map
fn is_bonus_spawn_triggered(roll: f32, delta_secs: f32, bonus_on_map: usize) -> bool {
    bonus_on_map < MAXIMUM_NUMBER_OF_BONUS_ON_MAP
        && roll < BONUS_SPAWN_CHANCE_PER_SECOND * delta_secs
}

/// Despawn the drops left on the ground for too long
pub(super) fn expire_bonus_drops_system(
    mut commands: Commands,
    time: Res<Time>,
    mut drops: Query<(Entity, &mut BonusDrop)>,
) {
    for (entity, mut drop) in drops.iter_mut() {
        if drop.tick_on_ground(time.delta_secs()) {
            commands.entity(entity).despawn();
        }
    }
}

/// Give the bonus of each drop to the first entity with a bonus slot touching it
pub(super) fn pick_up_bonus_drops_system(
    mut commands: Commands,
    drops: Query<(Entity, &BonusDrop, &Transform, &Hitbox)>,
    mut receivers: Query<(&Transform, &Hitbox, &mut BonusSlot, &mut Weapon)>,
) {
    for (drop_entity, drop, drop_transform, drop_hitbox) in drops.iter() {
        let drop_position = drop_transform.translation.truncate();

        let receiver = receivers
            .iter_mut()
            .find(|(receiver_transform, receiver_hitbox, _, _)| {
                receiver_hitbox.collides_with(
                    receiver_transform.translation.truncate(),
                    drop_hitbox,
                    drop_position,
                )
            });

        if let Some((_, _, mut bonus_slot, mut weapon)) = receiver {
            bonus_slot.apply(drop.kind, &mut weapon);
            commands.entity(drop_entity).despawn();
        }
    }
}

/// Randomly drop a new bonus on the map
pub(super) fn spawn_bonus_drops_system(
    mut commands: Commands,
    time: Res<Time>,
    drops: Query<(), With<BonusDrop>>,
) {
    let mut rng = rand::thread_rng();

    if is_bonus_spawn_triggered(rng.gen::<f32>(), time.delta_secs(), drops.iter().count()) {
        spawn_bonus_drop(&mut commands, BonusKind::MachineGun, random_position_in_arena());
    }
}

fn spawn_bonus_drop(commands: &mut Commands, kind: BonusKind, position: Vec2) {
    commands.spawn((
        Sprite {
            color: kind.color(),
            custom_size: Some(BONUS_DROP_SIZE),
            ..Default::default()
        },
        Transform::from_translation(position.extend(0.0)),
        BonusDrop::new(kind),
        Hitbox(BONUS_DROP_SIZE),
        despawn_on_restart(),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_drop_keeps_its_kind() {
        assert_eq!(BonusDrop::new(BonusKind::MachineGun).kind, BonusKind::MachineGun);
    }

    #[test]
    fn bonus_not_expired_before_time_on_ground() {
        let mut drop = BonusDrop::new(BonusKind::MachineGun);

        assert_eq!(drop.tick_on_ground(BONUS_TIME_ON_GROUND - 0.1), false);
    }

    #[test]
    fn bonus_expired_after_time_on_ground() {
        let mut drop = BonusDrop::new(BonusKind::MachineGun);

        drop.tick_on_ground(BONUS_TIME_ON_GROUND - 0.1);
        assert_eq!(drop.tick_on_ground(0.2), true);
    }

    #[test]
    fn bonus_spawn_low_roll_triggers() {
        assert_eq!(is_bonus_spawn_triggered(0.0, 0.016, 0), true);
    }

    #[test]
    fn bonus_spawn_high_roll_not_triggers() {
        assert_eq!(is_bonus_spawn_triggered(0.5, 0.016, 0), false);
    }

    #[test]
    fn bonus_spawn_blocked_when_map_is_full() {
        assert_eq!(
            is_bonus_spawn_triggered(0.0, 0.016, MAXIMUM_NUMBER_OF_BONUS_ON_MAP),
            false
        );
    }
}
