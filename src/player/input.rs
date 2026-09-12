//! Keyboard control of the player: arrow keys to move, space to fire.

use bevy::prelude::*;

use crate::combat::{fire_weapon, Faction, Weapon};
use crate::game::{GameSet, GameState};
use crate::physics::Movement;
use crate::player::Player;

/// Speed factor applied to each axis when several arrow keys are pressed
const DIAGONAL_FACTOR: f32 = 0.67;
const ARROW_KEYS: [KeyCode; 4] = [
    KeyCode::ArrowLeft,
    KeyCode::ArrowRight,
    KeyCode::ArrowUp,
    KeyCode::ArrowDown,
];

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        // Fire first: a shot leaves in the direction the player faced before this frame's input
        (fire_input_system, movement_input_system)
            .chain()
            .in_set(GameSet::Input)
            .run_if(in_state(GameState::Playing)),
    );
}

/// Fire while space is held, reload while it is released.
///
/// Weapons start empty, so reloading on release is what makes the first shot possible.
fn fire_input_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&Transform, &Movement, &Faction, &mut Weapon), With<Player>>,
) {
    let Ok((transform, movement, faction, mut weapon)) = player.single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) {
        fire_weapon(
            &mut commands,
            &mut weapon,
            time.delta_secs(),
            *faction,
            transform.translation.truncate(),
            movement.direction(),
        );
    } else {
        weapon.reload();
    }
}

/// Move the player along the arrow keys, and stop when none is pressed
fn movement_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Movement, With<Player>>,
) {
    let Ok(mut movement) = player.single_mut() else {
        return;
    };

    match direction_from_arrow_keys(&keyboard_input) {
        Some(direction) => movement.start_moving(direction),
        None => movement.stop(),
    }
}

/// The direction requested by the arrow keys, or `None` when no arrow key is pressed.
///
/// Right wins over left and down wins over up. When several arrow keys are pressed, each axis is
/// slowed down by `DIAGONAL_FACTOR`.
fn direction_from_arrow_keys(keyboard_input: &ButtonInput<KeyCode>) -> Option<Vec2> {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x = -1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x = 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y = 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y = -1.0;
    }

    let pressed_arrow_keys = ARROW_KEYS
        .into_iter()
        .filter(|key| keyboard_input.pressed(*key))
        .count();

    match pressed_arrow_keys {
        0 => None,
        1 => Some(direction),
        _ => Some(direction * DIAGONAL_FACTOR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys_pressed(keys: &[KeyCode]) -> ButtonInput<KeyCode> {
        let mut keyboard_input = ButtonInput::default();
        for key in keys {
            keyboard_input.press(*key);
        }
        keyboard_input
    }

    #[test]
    fn no_arrow_key_gives_no_direction() {
        assert_eq!(direction_from_arrow_keys(&keys_pressed(&[])), None);
        assert_eq!(
            direction_from_arrow_keys(&keys_pressed(&[KeyCode::Space])),
            None
        );
    }

    #[test]
    fn single_arrow_key_gives_full_direction() {
        assert_eq!(
            direction_from_arrow_keys(&keys_pressed(&[KeyCode::ArrowLeft])),
            Some(Vec2::new(-1.0, 0.0))
        );
        assert_eq!(
            direction_from_arrow_keys(&keys_pressed(&[KeyCode::ArrowDown])),
            Some(Vec2::new(0.0, -1.0))
        );
    }

    #[test]
    fn two_arrow_keys_give_slowed_diagonal() {
        assert_eq!(
            direction_from_arrow_keys(&keys_pressed(&[KeyCode::ArrowUp, KeyCode::ArrowRight])),
            Some(Vec2::new(DIAGONAL_FACTOR, DIAGONAL_FACTOR))
        );
    }

    #[test]
    fn opposite_arrow_keys_right_wins() {
        assert_eq!(
            direction_from_arrow_keys(&keys_pressed(&[KeyCode::ArrowLeft, KeyCode::ArrowRight])),
            Some(Vec2::new(DIAGONAL_FACTOR, 0.0))
        );
    }
}
