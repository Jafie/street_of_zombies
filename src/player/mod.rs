//! The character controlled by the player.

mod input;

use bevy::prelude::*;

use crate::animation::WalkAnimation;
use crate::bonus::BonusSlot;
use crate::combat::{Faction, Health, HitFlash, Weapon, WeaponStats};
use crate::game::{despawn_on_restart, CharacterSprite, GameAssets, GameSet, GameState};
use crate::physics::{Hitbox, Movement};

const PLAYER_START_POSITION: Vec2 = Vec2::new(0.0, -215.0);
const PLAYER_START_DIRECTION: Vec2 = Vec2::Y;
const PLAYER_SPEED: f32 = 350.0;
const PLAYER_HEALTH: u32 = 5;
const PLAYER_HITBOX: Hitbox = Hitbox(Vec2::new(35., 40.));
const PLAYER_WEAPON: WeaponStats = WeaponStats::PISTOL;
const PLAYER_HIT_FLASH: HitFlash = HitFlash::WHITE;

pub fn plugin(app: &mut App) {
    app.add_plugins(input::plugin)
        .add_systems(OnEnter(GameState::Playing), spawn_player_system)
        .add_systems(
            Update,
            game_over_on_death_system
                .in_set(GameSet::Resolution)
                .run_if(in_state(GameState::Playing)),
        );
}

/// Marks the character controlled by the player
#[derive(Component)]
pub struct Player;

fn spawn_player_system(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Player,
        assets.character_sprite(CharacterSprite::Player),
        Transform::from_translation(PLAYER_START_POSITION.extend(0.0)),
        Movement::standing(PLAYER_SPEED, PLAYER_START_DIRECTION),
        PLAYER_HITBOX,
        Health::new(PLAYER_HEALTH),
        PLAYER_HIT_FLASH,
        Faction::Player,
        Weapon::new(PLAYER_WEAPON),
        BonusSlot::default(),
        WalkAnimation::new(PLAYER_START_POSITION),
        despawn_on_restart(),
    ));
}

/// End the game when the player dies
fn game_over_on_death_system(
    player: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player.iter().any(Health::is_dead) {
        next_state.set(GameState::GameOver);
    }
}
