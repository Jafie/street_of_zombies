use crate::game_entity::MoveableSprite;
use crate::weapons::Weapon;
use crate::weapons::Pistol;

use bevy::{
    prelude::*
};

// Default pistol weapon data
static PROJECTILE_SPEED: f32 = 300.0;
static AMO_IN_WEAPON: u32 = 3;
static LIMIT_OF_FIRE: u32 = 500;
static FIRE_RATE: f32 = 0.5;
static NUMBER_OF_LIFE: i32 = 3;

/// The Ennemy
struct EnnemyInternalData {
    speed: f32,
    initial_position: (f32, f32),
    current_position : (f32, f32),
    move_direction: (f32, f32),
    fire_direction: (f32, f32),
    life: i32,
    current_weapon: Box<dyn Weapon + Send + Sync>,
    tick_elapsed: f32,
    cooldown_tick: f32,
    points_per_hits: u32
}

pub struct Ennemy {
    internal_data: EnnemyInternalData
}

impl MoveableSprite for Ennemy {
    fn get_speed(&self) -> f32 {
        self.internal_data.speed
    }
    fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.internal_data.move_direction = direction;
    }
    fn get_direction(&self) -> (f32, f32) {
        self.internal_data.move_direction
    }
    fn get_position(&self) -> (f32, f32) {
        self.internal_data.current_position
    }
    fn set_new_position(&mut self, position: (f32, f32)) {
        self.internal_data.current_position = position;
    }
}

impl Ennemy {
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), initial_pos: (f32, f32), fire_direction: (f32, f32), points: u32) -> Self {
        Ennemy { internal_data: EnnemyInternalData {
                            speed: speed_to_set,
                            initial_position: initial_pos,
                            current_position: initial_pos,
                            move_direction: direction_to_set,
                            fire_direction: fire_direction,
                            life: NUMBER_OF_LIFE,
                            current_weapon: Box::new(Pistol::new(PROJECTILE_SPEED, FIRE_RATE, AMO_IN_WEAPON, LIMIT_OF_FIRE)),
                            tick_elapsed: 0.,
                            cooldown_tick: 2.5,
                            points_per_hits: points}
        }
    }

    pub fn reduce_life(&mut self) {
        self.internal_data.life -= 1;
    }

    pub fn is_dead(&self) -> bool {
        self.internal_data.life < 1
    }

    pub fn launch_attack(&mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        time: &Res<Time>) {
            self.internal_data.current_weapon.fire_global(commands, materials, &time, self.internal_data.fire_direction, self.get_position(), true);

            self.internal_data.tick_elapsed += time.delta_seconds();

            if self.internal_data.tick_elapsed > self.internal_data.cooldown_tick {
                self.internal_data.current_weapon.reload();
                self.internal_data.tick_elapsed = 0.;
            }
    }

    pub fn get_initial_position(&self) -> (f32, f32) {
        self.internal_data.initial_position
    }

    pub fn get_point_value_per_hits(&self) -> u32 {
        self.internal_data.points_per_hits
    }

    pub fn get_point_value_on_death(&self) -> u32 {
        self.internal_data.points_per_hits*4
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn ennemy_initial_speed() {
        let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);

        assert_eq!(ennemy.get_speed(), 500.0);
    }

    #[test]
    fn ennemy_initial_direction() {
        let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);

        assert_eq!(ennemy.get_direction(), (5., 10.));
    }

    #[test]
    fn ennemy_initial_position() {
        let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);

        assert_eq!(ennemy.get_position(), (15., 20.));
    }

    #[test]
    fn ennemy_initial_initial_position() {
        let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);

        assert_eq!(ennemy.get_initial_position(), (15., 20.));
    }

    #[test]
    fn ennemy_get_value_per_hits() {
        let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);

        assert_eq!(ennemy.get_point_value_per_hits(), 50);
    }

    
    #[test]
    fn ennemy_set_position_without_alter_default() {
        let mut ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
        ennemy.set_new_position((60., 40.));
        assert_eq!(ennemy.get_initial_position(), (15., 20.));
    }

    #[test]
    fn ennemy_set_direction() {
        let mut ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
        ennemy.set_new_direction((100., 45.));
        assert_eq!(ennemy.get_direction(), (100., 45.));
    }

    #[test]
    fn ennemy_set_position() {
        let mut ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
        ennemy.set_new_position((60., 40.));
        assert_eq!(ennemy.get_position(), (60., 40.));
    }

    #[test]
    fn ennemy_reduce_life() {
        let mut ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);

        ennemy.reduce_life();

        assert_eq!(ennemy.is_dead(), NUMBER_OF_LIFE == 1);
    }

    #[test]
    pub fn ennemy_death_test() {
        let mut ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);

        for _ in 0..NUMBER_OF_LIFE {
            ennemy.reduce_life();
        }

        assert_eq!(ennemy.is_dead(), true);
    }
}
