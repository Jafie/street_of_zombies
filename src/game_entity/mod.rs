pub mod ennemies;
pub mod player;
pub mod projectiles;

use crate::game_system::GAME_AREA_LIMIT_X;
use crate::game_system::GAME_AREA_LIMIT_Y;

use bevy::prelude::*;

pub struct MoveableSprite {
    internal_data: MoveableSpriteData,
}

struct MoveableSpriteData {
    speed: f32,
    direction: (f32, f32),
    initial_position: (f32, f32),
    current_position: (f32, f32),
    previous_position: (f32, f32),
    hitbox_size: (f32, f32),
}

pub trait MoveableSpriteTrait {
    fn get_moveable_interface(&self) -> &MoveableSprite;
    fn get_moveable_interface_mut(&mut self) -> &mut MoveableSprite;
}

/// A sprite which is able to move
impl MoveableSprite {
    fn new(
        speed_to_set: f32,
        direction_to_set: (f32, f32),
        current_position_to_set: (f32, f32),
        hitbox_size: (f32, f32),
    ) -> Self {
        MoveableSprite {
            internal_data: MoveableSpriteData {
                speed: speed_to_set,
                direction: direction_to_set,
                initial_position: current_position_to_set,
                current_position: current_position_to_set,
                previous_position: current_position_to_set,
                hitbox_size: hitbox_size,
            },
        }
    }

    /// Get the current move speed of a moveable sprite in the game area
    ///
    /// # Examples
    ///
    /// ```
    /// let current_speed : f32 = my_moveable_sprit.get_speed();
    /// ```
    pub fn get_speed(&self) -> f32 {
        self.internal_data.speed
    }

    /// Get the current direction of a moveable sprite in the game area
    ///
    /// # Examples
    ///
    /// ```
    /// let current_speed : f32 = my_moveable_sprit.get_direction();
    /// ```
    pub fn get_direction(&self) -> (f32, f32) {
        self.internal_data.direction
    }

    /// Set the new direction of a moveable sprite
    ///
    /// # Arguments
    ///
    /// * `direction` - The new direction of your moveable sprite, set as (x, y)
    ///
    pub fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.internal_data.direction = direction;
    }

    /// Get the current position of a moveable sprite in the game area
    pub fn get_position(&self) -> (f32, f32) {
        self.internal_data.current_position
    }

    /// Set the new position of a moveable sprite
    ///
    /// # Arguments
    ///
    /// * `position` - The new position of your moveable sprite, set as (x, y)
    ///
    pub fn set_new_position(&mut self, position: (f32, f32)) {
        self.internal_data.current_position = position;
    }

    /// Get the hitbox size of the MoveableCharacter in format (x, y)
    ///
    /// # Examples
    ///
    /// ```
    /// let hitbox : (f32, f32) = my_moveable_sprit.get_hitbox_size();
    /// ```
    pub fn get_hitbox_size(&self) -> (f32, f32) {
        self.internal_data.hitbox_size
    }

    /// Check if the sprite moved between each calls.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));
    ///
    /// moveable_sprite.set_new_position((20., 50.0));
    /// assert_eq!(moveable_sprite.is_sprite_moved_after_last_call(), true);
    /// ```
    pub fn is_sprite_moved_after_last_call(&mut self) -> bool {
        if self.internal_data.previous_position != self.internal_data.current_position {
            self.internal_data.previous_position = self.internal_data.current_position;
            return true;
        }

        return false;
    }

    /// Move the sprite to a new position.
    ///
    /// # Arguments
    ///
    /// * `time` - The timer provided by Bevy engine.
    /// * `direction` - The movement direction.
    /// * `translated_movement` - The "translated movement" returned to bevy engine.
    ///
    pub fn move_sprite(
        &mut self,
        time: &Res<Time>,
        direction: &(f32, f32),
        translated_movement: &mut bevy::prelude::Vec3,
    ) {
        // move the sprite
        translated_movement.x += time.delta_seconds() * direction.0 * &self.get_speed();
        translated_movement.y += time.delta_seconds() * direction.1 * &self.get_speed();

        position_to_game_area_limit(translated_movement);

        self.set_new_position((translated_movement.x, translated_movement.y));
        self.set_new_direction(*direction);
    }
}

/// Force the position to the game area limit
///
/// # Arguments
///
/// * `translated_movement` - The "translated movement" returned to bevy engine.
///
fn position_to_game_area_limit(translated_movement: &mut bevy::prelude::Vec3) {
    translated_movement.x = translated_movement
        .x
        .min(GAME_AREA_LIMIT_X)
        .max(-GAME_AREA_LIMIT_X);
    translated_movement.y = translated_movement
        .y
        .min(GAME_AREA_LIMIT_Y)
        .max(-GAME_AREA_LIMIT_Y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moveable_sprite_initial_speed() {
        let moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));

        assert_eq!(moveable_sprite.get_speed(), 500.0);
    }

    #[test]
    fn moveable_sprite_initial_direction() {
        let moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));

        assert_eq!(moveable_sprite.get_direction(), (5., 10.));
    }

    #[test]
    fn moveable_sprite_initial_position() {
        let moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));

        assert_eq!(moveable_sprite.get_position(), (15., 20.));
    }

    #[test]
    fn moveable_sprite_set_direction() {
        let mut moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));

        moveable_sprite.set_new_direction((100., 45.));
        assert_eq!(moveable_sprite.get_direction(), (100., 45.));
    }

    #[test]
    fn moveable_sprite_set_position() {
        let mut moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));

        moveable_sprite.set_new_position((60., 40.));
        assert_eq!(moveable_sprite.get_position(), (60., 40.));
    }

    #[test]
    fn projectile_get_hitbox_size() {
        let moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));

        let hitbox_size = moveable_sprite.get_hitbox_size();
        assert_eq!(hitbox_size, (10., 10.));
    }

    #[test]
    fn sprite_moved_test() {
        let mut moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));

        moveable_sprite.set_new_position((20., 50.0));
        assert_eq!(moveable_sprite.is_sprite_moved_after_last_call(), true);
    }

    #[test]
    fn sprite_not_moved_test() {
        let mut moveable_sprite = MoveableSprite::new(500.0, (5., 10.), (15., 20.), (10., 10.));

        assert_eq!(moveable_sprite.is_sprite_moved_after_last_call(), false);
    }

    #[test]
    fn inside_game_limit_test() {
        let mut vect_in_game_are = Vec3::new(0.0, 0.0, 0.0);
        position_to_game_area_limit(&mut vect_in_game_are);

        assert_eq!(vect_in_game_are.x, 0.0);
        assert_eq!(vect_in_game_are.y, 0.0);
    }

    #[test]
    fn outside_game_limit_test() {
        let mut vect_in_game_are =
            Vec3::new(GAME_AREA_LIMIT_X + 50.0, GAME_AREA_LIMIT_Y + 50.0, 0.0);
        position_to_game_area_limit(&mut vect_in_game_are);

        assert_eq!(vect_in_game_are.x, GAME_AREA_LIMIT_X);
        assert_eq!(vect_in_game_are.y, GAME_AREA_LIMIT_Y);
    }
}
