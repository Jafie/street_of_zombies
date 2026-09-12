//! Projectiles: launching them, their range, and the damage they deal on contact.

use bevy::prelude::*;

use crate::combat::{Faction, Health, Hit, Weapon, WeaponStats};
use crate::game::{despawn_on_restart, is_outside_arena, GameSet};
use crate::math::calculate_cartesian_distance;
use crate::physics::{Hitbox, Movement};

const PROJECTILE_HITBOX: Hitbox = Hitbox(Vec2::new(10., 10.));
const PROJECTILE_SPRITE_SIZE: Vec2 = Vec2::new(5., 5.);
/// Health points removed by a projectile
const PROJECTILE_DAMAGE: u32 = 1;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            despawn_spent_projectiles_system.in_set(GameSet::Behavior),
            projectile_hit_system.in_set(GameSet::Collision),
        ),
    );
}

/// A projectile in flight, which disappears past its range
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Projectile {
    origin: Vec2,
    range: f32,
}

impl Projectile {
    pub fn new(origin: Vec2, range: f32) -> Self {
        Projectile { origin, range }
    }

    /// True when the projectile went past its range, or is about to leave the play area
    fn is_spent(&self, position: Vec2, direction: Vec2) -> bool {
        calculate_cartesian_distance(self.origin, position) > self.range
            || is_outside_arena(position + direction)
    }
}

/// Pull the trigger of a weapon for this frame. When it fires, launch a projectile of the
/// shooter's side from `position` towards `direction`.
pub fn fire_weapon(
    commands: &mut Commands,
    weapon: &mut Weapon,
    delta_secs: f32,
    faction: Faction,
    position: Vec2,
    direction: Vec2,
) {
    if weapon.try_fire(delta_secs) {
        spawn_projectile(commands, weapon.stats(), faction, position, direction);
    }
}

fn spawn_projectile(
    commands: &mut Commands,
    stats: WeaponStats,
    faction: Faction,
    position: Vec2,
    direction: Vec2,
) {
    commands.spawn((
        Sprite {
            color: projectile_color(faction),
            custom_size: Some(PROJECTILE_SPRITE_SIZE),
            ..Default::default()
        },
        Transform::from_translation(position.extend(0.0)),
        Projectile::new(position, stats.range),
        Movement::new(stats.projectile_speed, direction),
        PROJECTILE_HITBOX,
        faction,
        despawn_on_restart(),
    ));
}

fn projectile_color(faction: Faction) -> Color {
    match faction {
        Faction::Player => Color::WHITE,
        Faction::Ennemy => Color::srgb(1.0, 0.0, 0.0),
    }
}

/// Despawn the projectiles past their range or leaving the play area
fn despawn_spent_projectiles_system(
    mut commands: Commands,
    query: Query<(Entity, &Projectile, &Movement, &Transform)>,
) {
    for (entity, projectile, movement, transform) in query.iter() {
        if projectile.is_spent(transform.translation.truncate(), movement.direction()) {
            commands.entity(entity).despawn();
        }
    }
}

/// Each projectile touching a living target of the other side damages it and disappears.
/// A projectile hits a single target.
fn projectile_hit_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &Hitbox, &Faction), With<Projectile>>,
    mut targets: Query<(Entity, &Transform, &Hitbox, &Faction, &mut Health)>,
    mut hits: MessageWriter<Hit>,
) {
    for (projectile_entity, projectile_transform, projectile_hitbox, projectile_faction) in
        projectiles.iter()
    {
        let projectile_position = projectile_transform.translation.truncate();

        let touched_target = targets
            .iter_mut()
            .find(|(_, target_transform, target_hitbox, target_faction, health)| {
                projectile_faction.is_hostile_to(**target_faction)
                    && !health.is_dead()
                    && projectile_hitbox.collides_with(
                        projectile_position,
                        target_hitbox,
                        target_transform.translation.truncate(),
                    )
            });

        if let Some((target_entity, _, _, _, mut health)) = touched_target {
            health.take_damage(PROJECTILE_DAMAGE);
            hits.write(Hit {
                target: target_entity,
                is_fatal: health.is_dead(),
            });
            commands.entity(projectile_entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projectile_out_of_range() {
        let projectile = Projectile::new(Vec2::new(-400., 0.), 500.);

        assert_eq!(projectile.is_spent(Vec2::new(200., 0.), Vec2::X), true);
    }

    #[test]
    fn projectile_still_in_range() {
        let projectile = Projectile::new(Vec2::new(15., 20.), 500.);

        assert_eq!(projectile.is_spent(Vec2::new(50., 20.), Vec2::X), false);
    }

    #[test]
    fn projectile_leaving_arena_is_spent() {
        let projectile = Projectile::new(Vec2::ZERO, 5000.);
        let arena_edge = crate::game::clamp_to_arena(Vec2::new(5000., 0.));

        assert_eq!(projectile.is_spent(arena_edge, Vec2::X), true);
        assert_eq!(projectile.is_spent(arena_edge, -Vec2::X), false);
    }

    #[test]
    fn projectile_color_depends_on_side() {
        assert_eq!(projectile_color(Faction::Player), Color::WHITE);
        assert_ne!(projectile_color(Faction::Ennemy), Color::WHITE);
    }
}
