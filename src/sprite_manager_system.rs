use crate::game_entity::*;
use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use bevy::sprite::TextureAtlasSprite;
use bevy::time::Timer;

/// Path to the "ZOMBIE" sprite
static ZOMBIE_ASSET_PATH: &'static str = "sprites/zombie.png";
/// Path to the "PLAYER" sprite
static PLAYER_ASSET_PATH: &'static str = "sprites/woman.png";

/// Custom component that wraps Timer for sprite animation
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

/// Current direction of the entity (targeting up, left, right or down)
#[derive(PartialEq, Debug)]
enum TexturePositionEnum {
    DOWN,
    LEFT,
    RIGHT,
    UP,
}

/// Number of columns per sprite templates
static COLS_PER_SPRITES: usize = 8;

/// This is a function called by "Bevy" system.
/// This function will animate each ennemy and player sprites.
pub fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(
        Option<&mut ennemies::Ennemy>,
        Option<&mut player::Player>,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (enemy, player, mut timer, mut sprite) in query.iter_mut() {
        if let Some(mut enemy) = enemy {
            animate_sprite(
                &mut enemy.get_moveable_interface_mut(),
                &time,
                &mut timer.0,
                &mut sprite,
            );
        } else if let Some(mut player) = player {
            animate_sprite(
                &mut player.get_moveable_interface_mut(),
                &time,
                &mut timer.0,
                &mut sprite,
            );
        }
    }
}

/// Animate a "Moveable Sprite"
///
/// The animated sprite will change if the "Moveable Sprite" moved since last function call.
fn animate_sprite(
    entity: &mut MoveableSprite,
    time: &Res<Time>,
    timer: &mut Timer,
    sprite: &mut Mut<TextureAtlasSprite>,
) {
    timer.tick(time.delta());
    if timer.finished() && entity.is_sprite_moved_after_last_call() {
        let coef_val: usize;

        match generate_texture_position_from_coeff_factor(entity.get_direction()) {
            TexturePositionEnum::DOWN => coef_val = 0 * COLS_PER_SPRITES,
            TexturePositionEnum::LEFT => coef_val = 1 * COLS_PER_SPRITES,
            TexturePositionEnum::RIGHT => coef_val = 2 * COLS_PER_SPRITES,
            TexturePositionEnum::UP => coef_val = 3 * COLS_PER_SPRITES,
        }

        let calculated_index = ((sprite.index + 1) % COLS_PER_SPRITES) + coef_val;
        sprite.index = calculated_index;
    }
}

/// This function converts the direction of the "MoveableSprite" to a TexturePositionEnum (UP, DOWN, LEFT, RIGHT)
///
/// The animated sprite will change if the "Moveable Sprite" moved since last function call.
/// # Arguments
///
/// * `coeff_factor` - The direction factor as a tuple (x, y)
/// # Examples
///
/// ```
///     let generated_direction = generate_texture_position_from_coeff_factor((0.0, 1.0));
///     assert_eq!(generated_direction, TexturePositionEnum::UP);
/// ```
fn generate_texture_position_from_coeff_factor(coeff_factor: (f32, f32)) -> TexturePositionEnum {
    let texture_direction: TexturePositionEnum;
    let (coeff_factor_x, coeff_factor_y) = coeff_factor;

    let cond_1 = coeff_factor_x.abs() > coeff_factor_y.abs();
    let cond_2 = (coeff_factor_x - coeff_factor_y) >= 0.;

    if (coeff_factor_x.abs() - coeff_factor_y.abs()) == 0. {
        // Special case, strict coeff factor... Use Y
        if coeff_factor_y > 0. {
            texture_direction = TexturePositionEnum::UP
        } else {
            texture_direction = TexturePositionEnum::DOWN
        }
    } else {
        match (cond_1, cond_2) {
            (true, true) => texture_direction = TexturePositionEnum::RIGHT,
            (true, false) => texture_direction = TexturePositionEnum::LEFT,
            (false, true) => texture_direction = TexturePositionEnum::DOWN,
            (false, false) => texture_direction = TexturePositionEnum::UP,
        }
    }

    texture_direction
}

/// Enumerator about the different textures available
pub enum TextureToGenerate {
    PLAYER,
    ZOMBIE,
}

/// Generate a texture thanks to a "TextureToGenerate"
pub fn generate_texture(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    texture_type: TextureToGenerate,
) -> Handle<TextureAtlas> {
    let texture_path: &str;

    match texture_type {
        TextureToGenerate::PLAYER => texture_path = PLAYER_ASSET_PATH,
        TextureToGenerate::ZOMBIE => texture_path = ZOMBIE_ASSET_PATH,
    }

    let texture_handle = asset_server.load(texture_path);
    let generated_texture = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(80.0, 80.0), 
        8, 
        4,
        None,
        None
    );

    texture_atlases.add(generated_texture)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coeff_factor_to_texture_up() {
        let generated_direction = generate_texture_position_from_coeff_factor((0.0, 1.0));
        assert_eq!(generated_direction, TexturePositionEnum::UP);
    }

    #[test]
    fn coeff_factor_to_texture_down() {
        let generated_direction = generate_texture_position_from_coeff_factor((0.0, -1.0));
        assert_eq!(generated_direction, TexturePositionEnum::DOWN);
    }

    #[test]
    fn coeff_factor_to_texture_left() {
        let generated_direction = generate_texture_position_from_coeff_factor((-1.0, 0.0));
        assert_eq!(generated_direction, TexturePositionEnum::LEFT);
    }

    #[test]
    fn coeff_factor_to_texture_right() {
        let generated_direction = generate_texture_position_from_coeff_factor((1.0, 0.0));
        assert_eq!(generated_direction, TexturePositionEnum::RIGHT);
    }

    #[test]
    fn coeff_factor_to_texture_priority_up() {
        let generated_direction = generate_texture_position_from_coeff_factor((1.0, 1.0));
        assert_eq!(generated_direction, TexturePositionEnum::UP);
    }

    #[test]
    fn coeff_factor_to_texture_priority_down() {
        let generated_direction = generate_texture_position_from_coeff_factor((1.0, -1.0));
        assert_eq!(generated_direction, TexturePositionEnum::DOWN);
    }
}
