//! Geometry helpers shared by several features.

use bevy::math::Vec2;

/// Calculate the distance from a first point to a second point.
///
/// The coordinate differences are truncated to whole units before squaring, so the result is
/// only accurate to whole units.
pub fn calculate_cartesian_distance(from: Vec2, to: Vec2) -> f32 {
    let position_diff_x = (from.x - to.x) as i32;
    let position_diff_y = (from.y - to.y) as i32;
    let distance_squared = (position_diff_x.pow(2) + position_diff_y.pow(2)) as f32;

    distance_squared.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_cartesian_distance_positive() {
        let calculated_distance =
            calculate_cartesian_distance(Vec2::new(50., 50.), Vec2::new(10., 10.));
        assert_eq!(calculated_distance, 56.568542);
    }

    #[test]
    fn calculate_cartesian_distance_0() {
        let calculated_distance = calculate_cartesian_distance(Vec2::ZERO, Vec2::ZERO);
        assert_eq!(calculated_distance, 0.);
    }

    #[test]
    fn calculate_cartesian_distance_negative() {
        let calculated_distance =
            calculate_cartesian_distance(Vec2::new(-10., -10.), Vec2::new(-50., -50.));
        assert_eq!(calculated_distance, 56.568542);
    }
}
