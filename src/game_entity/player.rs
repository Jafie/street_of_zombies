use crate::game_entity::MoveableSprite;
use crate::weapons::Weapon;
use crate::weapons::Pistol;

use bevy::{
    prelude::*
};

// Default pistol weapon data
static PROJECTILE_SPEED: f32 = 700.0;
static AMO_IN_WEAPON: u32 = 800;
static LIMIT_OF_FIRE: u32 = 700;
static FIRE_RATE: f32 = 0.18;

/// The Main Character. Controllable by the player.
pub struct Player {
    player_data: PlayerInternal
}

struct PlayerInternal {
    speed: f32,
    current_position : (f32, f32),
    direction: (f32, f32),
    current_weapon: Box<dyn Weapon + Send + Sync>
}

impl MoveableSprite for Player {
    fn get_speed(&self) -> f32 {
        self.player_data.speed
    }
    fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.player_data.direction = direction;
    }
    fn get_direction(&self) -> (f32, f32) {
        self.player_data.direction
    }
    fn get_position(&self) -> (f32, f32) {
        self.player_data.current_position
    }
    fn set_new_position(&mut self, position: (f32, f32)) {
        self.player_data.current_position = position;
    }
}

impl Player {
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), initial_pos: (f32, f32)) -> Self {
        Player {
                player_data : PlayerInternal {
                            speed: speed_to_set,
                            current_position: initial_pos,
                            direction: direction_to_set,
                            current_weapon: Box::new(Pistol::new(PROJECTILE_SPEED, FIRE_RATE, AMO_IN_WEAPON, LIMIT_OF_FIRE)),
                        }
        }
    }

    pub fn fire(&mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        time: &Res<Time>) {
            self.player_data.current_weapon.fire_global(commands, materials, &time, self.get_direction(), self.get_position(), false);
    }

    pub fn reload_weapon(&mut self) {
        self.player_data.current_weapon.reload();
    }
}


mod tests {
    use super::*;

    #[test]
    fn player_initial_speed() {
        let player = Player::new(500.0, (5., 10.), (15., 20.));

        assert_eq!(player.get_speed(), 500.0);
    }

    #[test]
    fn player_initial_direction() {
        let player = Player::new(500.0, (5., 10.), (15., 20.));

        assert_eq!(player.get_direction(), (5., 10.));
    }

    #[test]
    fn player_initial_position() {
        let player = Player::new(500.0, (5., 10.), (15., 20.));

        assert_eq!(player.get_position(), (15., 20.));
    }

    #[test]
    fn player_set_direction() {
        let mut player = Player::new(500.0, (5., 10.), (15., 20.));
        player.set_new_direction((100., 45.));
        assert_eq!(player.get_direction(), (100., 45.));
    }

    #[test]
    fn player_set_position() {
        let mut player = Player::new(500.0, (5., 10.), (15., 20.));
        player.set_new_position((60., 40.));
        assert_eq!(player.get_position(), (60., 40.));
    }
}
