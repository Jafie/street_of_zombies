use crate::game_entity::MoveableSprite;
use crate::game_entity::MoveableSpriteTrait;

use crate::weapons::Pistol;
use crate::weapons::Weapon;

use bevy::prelude::*;
use bevy::ecs::component::Component;

// Default pistol weapon data
static PROJECTILE_SPEED: f32 = 300.0;
static AMO_IN_WEAPON: u32 = 3;
static LIMIT_OF_FIRE: u32 = 500;
static FIRE_RATE: f32 = 0.5;
static INITIAL_HEALTH_POINTS: i32 = 3;
static DEATH_POINT_COEF: u32 = 4;
static DEFAULT_ENNEMY_HITBOX_SIZE: (f32, f32) = (40., 50.);

struct EnnemyInternalData {
    health: i32,
    current_weapon: Box<dyn Weapon + Send + Sync>,
    tick_elapsed: f32,
    cooldown_tick: f32,
    points_per_hits: u32,
}

/// An ennemy entity - An Ennemy object contains all the data necessary for a single ennemy
#[derive(Component)]
pub struct Ennemy {
    sprite_data: MoveableSprite,
    internal_data: EnnemyInternalData,
}

impl MoveableSpriteTrait for Ennemy {
    fn get_moveable_interface(&self) -> &MoveableSprite {
        &self.sprite_data
    }

    fn get_moveable_interface_mut(&mut self) -> &mut MoveableSprite {
        &mut self.sprite_data
    }
}

impl Ennemy {
    /// Returns a new Ennemy object - An Ennemy object contains all the data necessary for a single ennemy
    ///
    /// # Arguments
    ///
    /// * `speed_to_set` - The speed of the ennemy
    /// * `initial_post` - The initial position on the game area
    /// * `fire_direction` - The default direction of fire
    /// * `points` - The number of points valued by the ennemy
    /// # Examples
    ///
    /// ```
    ///     let ennemy = let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), 50);
    /// ```
    pub fn new(
        speed_to_set: f32,
        direction_to_set: (f32, f32),
        initial_pos: (f32, f32),
        points: u32,
    ) -> Self {
        Ennemy {
            internal_data: EnnemyInternalData {
                health: INITIAL_HEALTH_POINTS,
                current_weapon: Box::new(Pistol::new(
                    PROJECTILE_SPEED,
                    FIRE_RATE,
                    AMO_IN_WEAPON,
                    LIMIT_OF_FIRE,
                )),
                tick_elapsed: 0.,
                cooldown_tick: 2.5,
                points_per_hits: points,
            },
            sprite_data: MoveableSprite::new(
                speed_to_set,
                direction_to_set,
                initial_pos,
                DEFAULT_ENNEMY_HITBOX_SIZE,
            ),
        }
    }

    /// Reduce the ennemy health by one
    ///
    /// # Examples
    ///
    /// ```
    ///     let ennemy = let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
    ///     ennemy.reduce_health();
    /// ```
    pub fn reduce_health(&mut self) {
        self.internal_data.health -= 1;
    }

    /// Return true if the ennemy health is equal to 0
    ///
    /// # Examples
    ///
    /// ```
    ///     let ennemy = let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
    ///     ennemy.reduce_health();
    ///     if ennemy.is_dead() {
    ///        println!("I am dead!");
    ///     }
    /// ```
    pub fn is_dead(&self) -> bool {
        self.internal_data.health < 1
    }

    /// The ennemy try to launch a projectile. If the weapon is charged (amo > 0), a projectile is launched.
    /// During an attack, the ennemy can "reload" its weapon if the "cooldown-tick" is passed
    ///
    /// # Arguments
    ///
    /// * `commands` - The bevy command
    /// * `time` - The timer (used for reloading)
    /// * `points` - The number of points valued by the ennemy
    /// ```
    pub fn launch_attack(
        &mut self,
        commands: &mut Commands,
        time: &Res<Time>,
    ) {
        self.internal_data.current_weapon.fire_global(
            commands,
            time,
            self.sprite_data.get_direction(),
            self.sprite_data.get_position(),
            true,
            // Add a dummy argument if required by the trait
        );

        self.internal_data.tick_elapsed += time.delta_seconds();

        if self.internal_data.tick_elapsed > self.internal_data.cooldown_tick {
            self.internal_data.current_weapon.reload();
            self.internal_data.tick_elapsed = 0.;
        }
    }

    /// Get the initial position where the ennemy was created
    ///
    /// # Examples
    ///
    /// ```
    ///     let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
    ///     assert_eq!(ennemy.get_initial_position(), (15., 20.));
    /// ```
    pub fn get_initial_position(&self) -> (f32, f32) {
        self.sprite_data.internal_data.initial_position
    }

    /// Get the value (in point for the score) of the ennemy per hits
    ///
    /// # Examples
    ///
    /// ```
    ///     let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
    ///     assert_eq!(ennemy.get_point_value_per_hits(), 50);
    /// ```
    pub fn get_point_value_per_hits(&self) -> u32 {
        self.internal_data.points_per_hits
    }

    /// Get the value (in point for the score) when the ennemy is dead
    ///
    /// # Examples
    ///
    /// ```
    ///     let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
    ///     assert_eq!(ennemy.get_point_value_on_death(), 50*DEATH_POINT_COEF);
    /// ```
    pub fn get_point_value_on_death(&self) -> u32 {
        self.internal_data.points_per_hits * DEATH_POINT_COEF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ennemy_get_initial_position() {
        let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), 50);

        assert_eq!(ennemy.get_initial_position(), (15., 20.));
    }

    #[test]
    fn ennemy_get_value_per_hits() {
        let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), 50);

        assert_eq!(ennemy.get_point_value_per_hits(), 50);
    }

    #[test]
    fn ennemy_get_value_on_death() {
        let ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), 50);

        assert_eq!(ennemy.get_point_value_on_death(), 50 * DEATH_POINT_COEF);
    }

    #[test]
    fn ennemy_set_position_without_alter_default() {
        let mut ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), 50);
        ennemy.sprite_data.set_new_position((60., 40.));
        assert_eq!(ennemy.get_initial_position(), (15., 20.));
    }

    #[test]
    fn ennemy_reduce_health() {
        let mut ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), 50);

        ennemy.reduce_health();

        assert_eq!(ennemy.is_dead(), INITIAL_HEALTH_POINTS == 1);
    }

    #[test]
    pub fn ennemy_death_test() {
        let mut ennemy = Ennemy::new(500.0, (5., 10.), (15., 20.), 50);

        for _ in 0..INITIAL_HEALTH_POINTS {
            ennemy.reduce_health();
        }

        assert_eq!(ennemy.is_dead(), true);
    }
}
