
use bevy::{
    prelude::*,
};

pub trait MoveableSprite {
    fn new(speed_to_set: f32) -> Self;
    fn get_speed(&self) -> f32;
    fn move_sprite(&self, time: &Res<Time>, direction: &(f32, f32), translated_movement: &mut bevy::prelude::Vec3) {
        // move the sprite
        translated_movement.x += time.delta_seconds() * direction.0 * &self.get_speed();
        translated_movement.y += time.delta_seconds() * direction.1 * &self.get_speed();
    }
}

pub struct MainCharacter {
    speed: f32
}

impl MoveableSprite for MainCharacter {
    fn new(speed_to_set: f32) -> Self {
        MainCharacter { speed: speed_to_set }
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }
}

