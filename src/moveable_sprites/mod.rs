pub mod main_character;
pub mod projectiles;
pub mod ennemies;

use bevy::{
    prelude::*,
};


/// A sprite which is able to move
pub trait MoveableSprite {
    /// Get the current move speed of a moveable sprite in the game area
    ///
    /// # Examples
    ///
    /// ```
    /// let current_speed : f32 = my_moveable_sprit.get_speed();
    /// ```
    fn get_speed(&self) -> f32;

    /// Get the current direction of a moveable sprite in the game area
    ///
    /// # Examples
    ///
    /// ```
    /// let current_speed : f32 = my_moveable_sprit.get_direction();
    /// ```
    fn get_direction(&self) -> (f32, f32);

    
    /// Set the new direction of a moveable sprite
    ///
    /// # Arguments
    ///
    /// * `direction` - The new direction of your moveable sprite, set as (x, y)
    ///
    fn set_new_direction(&mut self, direction: (f32, f32));

    /// Get the current position of a moveable sprite in the game area
    fn get_position(&self) -> (f32, f32);

    /// Set the new position of a moveable sprite
    ///
    /// # Arguments
    ///
    /// * `position` - The new position of your moveable sprite, set as (x, y)
    ///
    fn set_new_position(&mut self, position: (f32, f32));

    /// Move the sprite to a new position.
    ///
    /// # Arguments
    ///
    /// * `time` - The timer provided by Bevy engine.
    /// * `direction` - The movement direction.
    /// * `translated_movement` - The "translated movement" returned to bevy engine.
    ///
    fn move_sprite(&mut self, time: &Res<Time>, direction: &(f32, f32), translated_movement: &mut bevy::prelude::Vec3) {
        // move the sprite
        translated_movement.x += time.delta_seconds() * direction.0 * &self.get_speed();
        translated_movement.y += time.delta_seconds() * direction.1 * &self.get_speed();
        self.set_new_position((translated_movement.x, translated_movement.y));
        self.set_new_direction(*direction);
    }
}
