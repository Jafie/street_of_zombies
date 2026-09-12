//! The zombies: what they are made of, how they behave and when they appear.

mod ai;
mod spawn;

use bevy::prelude::*;

use crate::animation::WalkAnimation;
use crate::combat::{Faction, Health, Weapon, WeaponStats};
use crate::game::{despawn_on_restart, CharacterSprite, GameAssets};
use crate::physics::{Hitbox, Movement};
use crate::progression::PointValue;

const ENNEMY_SPEED: f32 = 200.0;
const ENNEMY_HEALTH: u32 = 3;
const ENNEMY_HITBOX: Hitbox = Hitbox(Vec2::new(40., 50.));
const ENNEMY_POINTS_PER_HIT: u32 = 50;
const ENNEMY_WEAPON: WeaponStats = WeaponStats::ZOMBIE_PISTOL;

pub fn plugin(app: &mut App) {
    app.add_plugins((ai::plugin, spawn::plugin));
}

/// Marks a zombie
#[derive(Component)]
pub struct Ennemy;

/// Spawn a zombie wandering around `position`
fn spawn_ennemy(commands: &mut Commands, assets: &GameAssets, position: Vec2, direction: Vec2) {
    commands.spawn((
        Ennemy,
        assets.character_sprite(CharacterSprite::Zombie),
        Transform::from_translation(position.extend(0.0)),
        Movement::new(ENNEMY_SPEED, direction),
        ENNEMY_HITBOX,
        Health::new(ENNEMY_HEALTH),
        Faction::Ennemy,
        PointValue::new(ENNEMY_POINTS_PER_HIT),
        // Empty: the first shots come after the first reload
        Weapon::new(ENNEMY_WEAPON),
        ai::Wander::new(position),
        ai::ReloadCooldown::default(),
        WalkAnimation::new(position),
        despawn_on_restart(),
    ));
}
