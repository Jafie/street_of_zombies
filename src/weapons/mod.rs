pub mod pistol;

use crate::moveable_sprites::projectiles::Projectile;
use crate::moveable_sprites::Collider;

use bevy::{
    prelude::*,
};


/// A Pistol is a "Weapon", single fire
pub struct Pistol {
    /// Speed of the projectile
    speed: f32,
    /// Number of amo in the weapon
    amo: u32,
    /// Distance of fire of the projectiles generated by the weapon
    limit_of_fire: u32,
    /// The fire rate
    initial_fire_rate : f32,
    /// The fire rate
    current_fire_rate_timer : f32
}

/// Define a Weapon which is able to fire projectiles
pub trait Weapon {
    /// Create a new Weapon
    fn new() -> Self where Self: Sized;
    
    /// The generic fire command. Will generate a projectile following the weapon type defined
    ///
    /// # Arguments
    ///
    /// * `command` - The bevy command interface.
    /// * `materials` - The bevy material interface.
    /// * `direction` - The direction to fire the projectile.
    /// * `initial_pos` - The initial position of the projectile.
    ///
    fn fire_global(&mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        time: &Res<Time>,
        direction: (f32, f32),
        initial_pos: (f32, f32)) {

            if self.get_amo() > 0 && self.is_ready_to_fire(time.delta_seconds()){
            let (pos_x, pox_y) = initial_pos;
            commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.3, 0.0, 1.0).into()),
                transform: Transform::from_xyz(pos_x, pox_y, 0.0),
                sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                ..Default::default()
            })
            .insert(self.create_projectile(direction, initial_pos))
            .insert(Collider::ProjectileCollision);
            self.reduce_amo();
        }

    }

    /// Method to reload the weapon amo.
    fn reload(&mut self);

    /// Get the number of amo available in the weapon
    fn get_amo(&self) -> u32;

    /// Reduce the number of amo in the weapon by 1
    fn reduce_amo(&mut self);

    /// Get the fire rate of the weapon
    fn is_ready_to_fire(&mut self, time_elapsed_since_last_update: f32) -> bool;

    /// Create a new projectile
    ///
    /// # Arguments
    ///
    /// * `direction_to_set` - The direction of the projectile
    /// * `initial_position_to_set` - The initial position of the projectile
    ///
    fn create_projectile(&self, direction_to_set: (f32, f32), initial_position_to_set: (f32, f32)) -> Projectile;
}