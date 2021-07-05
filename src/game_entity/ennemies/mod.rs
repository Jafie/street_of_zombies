use crate::game_entity::MoveableSprite;
use crate::weapons::Weapon;
use crate::weapons::Pistol;

use bevy::{
    prelude::*
};

// Default pistol weapon data
static PROJECTILE_SPEED: f32 = 500.0;
static AMO_IN_WEAPON: u32 = 3;
static LIMIT_OF_FIRE: u32 = 500;
static FIRE_RATE: f32 = 0.2;

/// The Ennemy
pub struct Ennemy {
    speed: f32,
    initial_position: (f32, f32),
    current_position : (f32, f32),
    move_direction: (f32, f32),
    fire_direction: (f32, f32),
    life: i32,
    current_weapon: Box<dyn Weapon + Send + Sync>,
    tick_elapsed: f32,
    cooldown_tick: f32
}

impl MoveableSprite for Ennemy {
    fn get_speed(&self) -> f32 {
        self.speed
    }
    fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.move_direction = direction;
    }
    fn get_direction(&self) -> (f32, f32) {
        self.move_direction
    }
    fn get_position(&self) -> (f32, f32) {
        self.current_position
    }
    fn set_new_position(&mut self, position: (f32, f32)) {
        self.current_position = position;
    }
}

impl Ennemy {
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), initial_pos: (f32, f32)) -> Self {
        Ennemy {
             speed: speed_to_set,
             initial_position: initial_pos,
             current_position: initial_pos,
             move_direction: direction_to_set,
             fire_direction: (0.0, -1.0),
             life: 30,
             current_weapon: Box::new(Pistol::new(PROJECTILE_SPEED, FIRE_RATE, AMO_IN_WEAPON, LIMIT_OF_FIRE)),
             tick_elapsed: 0.,
             cooldown_tick: 2.0}
    }

    pub fn reduce_life(&mut self) {
        self.life -= 1;
        println!("HIT! Ennemy life = {}", self.life);
    }

    pub fn is_dead(&self) -> bool {
        self.life < 1
    }

    pub fn launch_attack(&mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        time: &Res<Time>) {
            self.current_weapon.fire_global(commands, materials, &time, self.fire_direction, self.get_position(), true);

            self.tick_elapsed += time.delta_seconds();

            if self.tick_elapsed > self.cooldown_tick {
                self.current_weapon.reload();
                self.tick_elapsed = 0.;
            }
    }

    pub fn get_initial_position(&self) -> (f32, f32) {
        self.initial_position
    }
}
