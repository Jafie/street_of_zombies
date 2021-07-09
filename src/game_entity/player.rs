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
static DEFAULT_PLAYER_HITBOX_SIZE : (f32, f32) = (35., 40.);

/// The Main Character entity, Controllable by the player. - A Player object contains all the information dedicated to a the player.
pub struct Player {
    player_data: PlayerInternal
}

struct PlayerInternal {
    speed: f32,
    current_position : (f32, f32),
    direction: (f32, f32),
    hitbox_size: (f32, f32),
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
    fn get_hitbox_size(&self) -> (f32, f32) {
        self.player_data.hitbox_size
    }
}

impl Player {
    /// Returns a new Player object - A Player object contains all the information dedicated to a the player.
    ///
    /// # Arguments
    ///
    /// * `speed_to_set` - The speed of the player
    /// * `direction_to_set` - The direction of the player
    /// * `initial_pos` - The initial position of the player
    /// # Examples
    ///
    /// ```
    ///     let player = Player::new(500.0, (5., 10.), (15., 20.));
    /// ```
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), initial_pos: (f32, f32)) -> Self {
        Player {
                player_data : PlayerInternal {
                            speed: speed_to_set,
                            current_position: initial_pos,
                            direction: direction_to_set,
                            hitbox_size: DEFAULT_PLAYER_HITBOX_SIZE,
                            current_weapon: Box::new(Pistol::new(PROJECTILE_SPEED, FIRE_RATE, AMO_IN_WEAPON, LIMIT_OF_FIRE)),
                        }
        }
    }

    /// Launch a Projectile with the player weapon
    ///
    /// # Arguments
    ///
    /// * `commands` - The bevy command object.
    /// * `materials` - The bevy material object.
    /// * `time` - The timer generated by the bevy system.
    /// ```
    pub fn fire(&mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        time: &Res<Time>) {
            self.player_data.current_weapon.fire_global(commands, materials, &time, self.get_direction(), self.get_position(), false);
    }

    /// Reload the player weapon
    ///
    /// # Examples
    ///
    /// ```
    ///     let player = Player::new(500.0, (5., 10.), (15., 20.));
    ///     player.reload_weapon()
    /// ```
    pub fn reload_weapon(&mut self) {
        self.player_data.current_weapon.reload();
    }
}

#[cfg(test)]
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

    #[test]
    fn player_get_hitbox_size() {
        let player = Player::new(500.0, (5., 10.), (15., 20.));
        let hitbox_size = player.get_hitbox_size();
        assert_eq!(hitbox_size, DEFAULT_PLAYER_HITBOX_SIZE);
    }

    #[test]
    fn player_weapon_reload_test() {
        let mut player = Player::new(500.0, (5., 10.), (15., 20.));
        // Reload in case of "uncharged initial weapon"
        player.reload_weapon();
        let initial_amo = player.player_data.current_weapon.get_amo();
        player.player_data.current_weapon.reduce_amo();
        player.reload_weapon();
        assert_eq!(player.player_data.current_weapon.get_amo(), initial_amo);
    }
}
