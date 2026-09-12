use crate::game_entity::bonus::{Bonus, BonusKind};
use crate::game_entity::player::Player;
use crate::game_entity::MoveableSpriteTrait;
use crate::game_system::projectile_and_kill_gameplay::is_entities_collides;
use crate::game_system::*;
use rand::Rng;

use bevy::prelude::*;

static MAXIMUM_NUMBER_OF_BONUS_ON_MAP: usize = 1;
static BONUS_SPAWN_CHANCE_PER_SECOND: f32 = 0.05;

/// Game System: spawn of the bonus drops, pick-up by the player and duration of the active bonus. Managed by as a "Bevy System"
pub fn bonus_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
    mut bonus_query: Query<(&mut Bonus, Entity)>,
) {
    // No player means game over: nothing to update, pick up or spawn
    if let Ok(mut player) = player_query.single_mut() {
        player.update_bonus(time.delta_secs());

        pick_up_or_expire_bonuses(&mut commands, &time, &mut player, &mut bonus_query);

        let mut rng = rand::thread_rng();
        if is_bonus_spawn_triggered(
            rng.gen::<f32>(),
            time.delta_secs(),
            bonus_query.iter().count(),
        ) {
            generate_new_bonus(&mut commands, BonusKind::MachineGun);
        }
    }
}

fn pick_up_or_expire_bonuses(
    commands: &mut Commands,
    time: &Res<Time>,
    player: &mut Player,
    bonus_query: &mut Query<(&mut Bonus, Entity)>,
) {
    for (mut bonus, bonus_entity) in bonus_query.iter_mut() {
        if bonus.tick_on_ground(time.delta_secs()) {
            commands.entity(bonus_entity).despawn();
        } else if is_entities_collides(
            player.get_moveable_interface(),
            bonus.get_moveable_interface(),
        ) {
            player.apply_bonus(bonus.get_kind());
            commands.entity(bonus_entity).despawn();
        }
    }
}

fn generate_new_bonus(commands: &mut Commands, kind: BonusKind) {
    let bonus_position: (f32, f32) =
        math_and_generator::generate_random_position(GAME_AREA_LIMIT_X, GAME_AREA_LIMIT_Y);
    let bonus = Bonus::new(kind, bonus_position);
    let (hitbox_x, hitbox_y) = bonus.get_moveable_interface().get_hitbox_size();

    commands.spawn((
        Sprite {
            color: kind.get_color(),
            custom_size: Some(Vec2::new(hitbox_x, hitbox_y)),
            ..Default::default()
        },
        Transform::from_xyz(bonus_position.0, bonus_position.1, 0.0),
        bonus,
    ));
}

/// Return true if a new bonus drop shall be spawned during this frame
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

#[cfg(test)]
mod tests {
    use super::*;

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
