use bevy::{
    prelude::*,
};

pub struct MainCharacter {
    pub speed: f32,
}

impl MainCharacter {
    pub fn move_sprite(&self, time: &Res<Time>, direction: &f32, translated_movement: &mut f32) {
        // move the main player 
        *translated_movement += time.delta_seconds() * direction * self.speed;
    }

    pub fn fire(&self) {

    }
}
