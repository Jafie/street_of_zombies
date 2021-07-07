use rand::Rng;

pub fn calculate_cartesian_distance(from_coord: (f32, f32), to_coord: (f32, f32)) -> f32 {
    let (from_x, from_y) = from_coord;
    let (to_x, to_y) = to_coord;

    let position_diff_x = (from_x - to_x) as i32;
    let position_diff_y = (from_y - to_y) as i32;
    let distance_walked_squared = (position_diff_x.pow(2) + position_diff_y.pow(2)) as f32;

    distance_walked_squared.sqrt()
}

pub fn generate_random_direction_factor() -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let random_direction_factor_x : f32= rng.gen_range(-1.0..1.0);
    let random_direction_factor_y : f32= rng.gen_range(-1.0..1.0);

    (random_direction_factor_x, random_direction_factor_y)
}

pub fn generate_random_direction_factor_strict() -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let direction_factor_x_abs: f32 = rng.gen_range(0.0..1.0);
    let direction_factor_y_abs: f32 = 1.0 - direction_factor_x_abs;

    let random_sign_generator: u32 = rng.gen_range(0..4);
    let direction_factor: (f32, f32);
    match random_sign_generator {
        0 => direction_factor = (direction_factor_x_abs, direction_factor_y_abs),
        1 => direction_factor = (direction_factor_x_abs, -direction_factor_y_abs),
        2 => direction_factor = (-direction_factor_x_abs, direction_factor_y_abs),
        3 => direction_factor = (-direction_factor_x_abs, -direction_factor_y_abs),
        _ => direction_factor = (direction_factor_x_abs, direction_factor_y_abs)
    }

    direction_factor
}

pub fn generate_random_position() -> (f32, f32) {
    let mut rng = rand::thread_rng();

    let generated_position: (f32, f32) = (rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
    
    generated_position
}
