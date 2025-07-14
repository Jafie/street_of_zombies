use rand::Rng;

/// Calculate the distance between two objects in a "cartesian referencial".
/// Calculate from the "first point" to the "second point"
///
/// # Arguments
///
/// * `from_coord` - The coordinate of the first point (x, y)
/// * `to_coord` - The coordinate of the second point (x, y)
///
/// # Examples
///
/// ```
///     let from_point: (f32, f32) = (50., 50.);
///     let to_point: (f32, f32) = (10., 10.);
///     let calculated_distance = calculate_cartesian_distance(from_point, to_point);
///     assert_eq!(calculated_distance, 80.);
/// ```
pub fn calculate_cartesian_distance(from_coord: (f32, f32), to_coord: (f32, f32)) -> f32 {
    let (from_x, from_y) = from_coord;
    let (to_x, to_y) = to_coord;

    let position_diff_x = (from_x - to_x) as i32;
    let position_diff_y = (from_y - to_y) as i32;
    let distance_walked_squared = (position_diff_x.pow(2) + position_diff_y.pow(2)) as f32;

    distance_walked_squared.sqrt()
}

/// Generate a random direction factor in a tuple (x, y) (without x.abs() + y.abs() = 1)
///
/// # Examples
///
/// ```
///     let my_direction_factor = generate_random_direction_factor();
/// ```
pub fn generate_random_direction_factor() -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let random_direction_factor_x: f32 = rng.gen_range(-1.0..1.0);
    let random_direction_factor_y: f32 = rng.gen_range(-1.0..1.0);

    (random_direction_factor_x, random_direction_factor_y)
}

/// Generate a random position limited to the entry value (in a square)
///
/// # Arguments
///
/// * `absolute_max_x` - The maximal coordinate in X axe from -absolute_max_x to absolute_max_x
/// * `absolute_max_y` - The maximal coordinate in Y axe from -absolute_max_y to absolute_max_y
///
/// # Examples
///
/// ```
///     // Generate a random number in the range (-500..500, -200..200)
///     let my_direction_factor = generate_random_position(500., 200.);
/// ```
pub fn generate_random_position(absolute_max_x: f32, absolute_max_y: f32) -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let generated_position: (f32, f32) = (
        rng.gen_range(-absolute_max_x..absolute_max_x),
        rng.gen_range(-absolute_max_y..absolute_max_y),
    );

    generated_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_cartesian_distance_positive() {
        let from_point: (f32, f32) = (50., 50.);
        let to_point: (f32, f32) = (10., 10.);
        let calculated_distance = calculate_cartesian_distance(from_point, to_point);
        assert_eq!(calculated_distance, 56.568542);
    }

    #[test]
    fn calculate_cartesian_distance_0() {
        let from_point: (f32, f32) = (0., 0.);
        let to_point: (f32, f32) = (0., 0.);
        let calculated_distance = calculate_cartesian_distance(from_point, to_point);
        assert_eq!(calculated_distance, 0.);
    }

    #[test]
    fn calculate_cartesian_distance_negative() {
        let from_point: (f32, f32) = (-10., -10.);
        let to_point: (f32, f32) = (-50., -50.);
        let calculated_distance = calculate_cartesian_distance(from_point, to_point);
        assert_eq!(calculated_distance, 56.568542);
    }

}
