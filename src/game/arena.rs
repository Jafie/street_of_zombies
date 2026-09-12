//! The play area: its bounds, and the camera and background that show it.

use bevy::prelude::*;
use rand::Rng;

use crate::game::GameAssets;

/// Half the size of the play area: positions stay within +/-x and +/-y of the center
const ARENA_HALF_SIZE: Vec2 = Vec2::new(500.0, 300.0);
/// Below every sprite, which are all drawn at z = 0
const BACKGROUND_Z: f32 = -1.0;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera_and_background_system);
}

fn spawn_camera_and_background_system(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(assets.background.clone()),
        Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
    ));
}

/// Bring a position back inside the play area
pub fn clamp_to_arena(position: Vec2) -> Vec2 {
    position.clamp(-ARENA_HALF_SIZE, ARENA_HALF_SIZE)
}

/// True when a position is outside the play area
pub fn is_outside_arena(position: Vec2) -> bool {
    position.x.abs() > ARENA_HALF_SIZE.x || position.y.abs() > ARENA_HALF_SIZE.y
}

/// A random position inside the play area
pub fn random_position_in_arena() -> Vec2 {
    let mut rng = rand::thread_rng();

    Vec2::new(
        rng.gen_range(-ARENA_HALF_SIZE.x..ARENA_HALF_SIZE.x),
        rng.gen_range(-ARENA_HALF_SIZE.y..ARENA_HALF_SIZE.y),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_inside_arena_is_not_clamped() {
        assert_eq!(clamp_to_arena(Vec2::ZERO), Vec2::ZERO);
    }

    #[test]
    fn position_outside_arena_is_clamped_to_the_limit() {
        let outside = ARENA_HALF_SIZE + Vec2::new(50.0, 50.0);

        assert_eq!(clamp_to_arena(outside), ARENA_HALF_SIZE);
        assert_eq!(clamp_to_arena(-outside), -ARENA_HALF_SIZE);
    }

    #[test]
    fn position_past_the_limit_is_outside_arena() {
        assert_eq!(is_outside_arena(ARENA_HALF_SIZE + Vec2::new(1.0, 1.0)), true);
    }

    #[test]
    fn position_on_the_limit_is_inside_arena() {
        assert_eq!(is_outside_arena(ARENA_HALF_SIZE), false);
        assert_eq!(is_outside_arena(Vec2::ZERO), false);
    }

    #[test]
    fn random_position_is_inside_arena() {
        for _ in 0..100 {
            assert_eq!(is_outside_arena(random_position_in_arena()), false);
        }
    }
}
