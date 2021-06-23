
pub fn calculate_cartesian_distance(from_coord: (f32, f32), to_coord: (f32, f32)) -> u32 {
    let (from_x, from_y) = from_coord;
    let (to_x, to_y) = to_coord;

    let position_diff_x = (from_x - to_x) as i32;
    let position_diff_y = (from_y - to_y) as i32;
    let distance_walked_squared = (position_diff_x.pow(2) + position_diff_y.pow(2)) as u32;

    distance_walked_squared
}
