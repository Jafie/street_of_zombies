//! Hurting and killing: health, sides, weapons and the projectiles they fire.

mod health;
mod projectile;
mod weapon;

pub use health::Health;
pub use projectile::fire_weapon;
pub use weapon::{Weapon, WeaponStats};

use bevy::prelude::*;

use crate::game::GameSet;

pub fn plugin(app: &mut App) {
    app.add_message::<Hit>()
        .add_plugins(projectile::plugin)
        .add_systems(Update, despawn_dead_system.in_set(GameSet::Cleanup));
}

/// The side an entity fights on. Attacks only hurt the other side.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faction {
    Player,
    Ennemy,
}

impl Faction {
    /// True when an attack from this side can hurt an entity of the `target` side
    pub fn is_hostile_to(self, target: Faction) -> bool {
        self != target
    }
}

/// Sent when an entity takes damage
#[derive(Message, Debug, Clone, Copy)]
pub struct Hit {
    pub target: Entity,
    /// True when this hit killed the target
    pub is_fatal: bool,
}

/// Despawn every entity whose health reached zero
fn despawn_dead_system(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_attack_hurts_ennemy() {
        assert_eq!(Faction::Player.is_hostile_to(Faction::Ennemy), true);
    }

    #[test]
    fn ennemy_attack_hurts_player() {
        assert_eq!(Faction::Ennemy.is_hostile_to(Faction::Player), true);
    }

    #[test]
    fn attack_does_not_hurt_own_side() {
        assert_eq!(Faction::Player.is_hostile_to(Faction::Player), false);
        assert_eq!(Faction::Ennemy.is_hostile_to(Faction::Ennemy), false);
    }
}
