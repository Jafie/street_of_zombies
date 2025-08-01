use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::game_entity::*;
use crate::game_system::*;

/// Game System: Automatic movement of the projectiles. Managed by as a "Bevy System"
pub fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(&mut projectiles::Projectile, &mut Transform, Entity)>,
) {
    for projectile_single_query in projectile_query.iter_mut() {
        let (mut projectile, mut transform, projectile_entity) = projectile_single_query;
        let direction_of_fire = projectile.get_moveable_interface().get_direction();
        projectile.get_moveable_interface_mut().move_sprite(
            &time,
            &direction_of_fire,
            &mut transform.translation,
        );

        // If outside of game area, delete
        if (projectile.is_out_of_distance())
            || is_next_movement_out_of_game_area(
                projectile.get_moveable_interface().get_position(),
                projectile.get_moveable_interface().get_direction(),
            )
        {
            commands.entity(projectile_entity).despawn();
        }
    }
}

/// Game System: The collision system with projectiles and manage the part "health + Score" of the score system. Managed by as a "Bevy System"
pub fn projectile_collision_and_score_system(
    mut commands: Commands,
    mut enemy_query: Query<(&mut ennemies::Ennemy, Entity)>,
    mut player_query: Query<(&mut player::Player, Entity)>,
    projectile_query: Query<(Entity, &projectiles::Projectile)>,
    mut scoreboard_query: Query<(&mut scoreboard::ScoreAndInfo, &mut Text, &mut Style)>,
) {
    if let Ok((mut score_struct, mut score_text, mut style_text)) = scoreboard_query.get_single_mut() {

    // check collision with objects
    for (collider_entity, projectile) in projectile_query.iter() {
        if projectile.is_coming_from_ennemy() {
            check_collision_with_player(
                &mut commands,
                &mut player_query,
                projectile,
                &collider_entity,
                &mut score_struct,
            );
        } else {
            check_collision_with_ennemy(
                &mut commands,
                &mut enemy_query,
                projectile,
                &collider_entity,
                &mut score_struct,
            );
        }
    }

        score_struct.update_percent_until_next_level();
        score_struct.update_scoarboard_text(&mut score_text, &mut style_text);
    }
}

fn check_collision_with_ennemy(
    commands: &mut Commands,
    entity_query: &mut Query<(&mut ennemies::Ennemy, Entity)>,
    projectile: &projectiles::Projectile,
    projectile_entity: &Entity,
    score_struct: &mut scoreboard::ScoreAndInfo,
) {
    for (mut ennemy, entity_ennemy) in entity_query.iter_mut() {
        let sprite_interface_one = ennemy.get_moveable_interface();
        let sprite_interface_two = projectile.get_moveable_interface();

        if is_entities_collides(&sprite_interface_one, sprite_interface_two) {
            commands.entity(*projectile_entity).despawn();
            ennemy.reduce_health();
            score_struct.add_to_score(ennemy.get_point_value_per_hits());
            check_and_treat_ennemy_health(commands, &mut ennemy, entity_ennemy, score_struct);
        }
    }
}

fn check_collision_with_player(
    commands: &mut Commands,
    entity_query: &mut Query<(&mut player::Player, Entity)>,
    projectile: &projectiles::Projectile,
    projectile_entity: &Entity,
    score_struct: &mut scoreboard::ScoreAndInfo,
) {
    for (player, entity_player) in entity_query.iter_mut() {
        let sprite_interface_one = player.get_moveable_interface();
        let sprite_interface_two = projectile.get_moveable_interface();

        if is_entities_collides(&sprite_interface_one, sprite_interface_two) {
            commands.entity(*projectile_entity).despawn();
            score_struct.remove_health(1);
            check_and_treat_player_health(commands, entity_player, score_struct);
        }
    }
}

fn check_and_treat_ennemy_health(
    commands: &mut Commands,
    ennemy: &mut ennemies::Ennemy,
    entity: Entity,
    score: &mut scoreboard::ScoreAndInfo,
) {
    if ennemy.is_dead() {
        score.add_to_score(ennemy.get_point_value_on_death());
        commands.entity(entity).despawn();
    }
}

fn check_and_treat_player_health(
    commands: &mut Commands,
    player_entity: Entity,
    score: &mut scoreboard::ScoreAndInfo,
) {
    if score.is_gameover() {
        commands.entity(player_entity).despawn();
    }
}

/// Retrieve the position and the hitbox converted in "Bevy" format from a "Moveable Sprite"
/// The format retrieved is a Tuple ((Position_x, Position_y, 0.), (Hibox_x, Hitbox_y))
///
/// # Examples
///
/// ```
///    let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
///    let (projectile_position, projectile_hitbox) = get_position_and_hitboxes(&projectile);
/// ```
fn get_position_and_hitboxes(entity: &MoveableSprite) -> (Vec3, Vec2) {
    let position = entity.get_position();
    let hibox_size = entity.get_hitbox_size();

    (
        Vec3::new(position.0, position.1, 0.),
        Vec2::new(hibox_size.0, hibox_size.1),
    )
}

/// Return true if the two entities collides
/// It means that there is a collision between the position and hit-boxes of the MoveableSprite.
///
/// # Examples
///
/// ```
///    let projectile = Projectile::new(500.0, (5., 10.), (15., 20.), 500, false);
///    let ennemy = ennemies::Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
///    assert_eq!(is_entities_collides(&ennemy, &projectile), true);
/// ```
fn is_entities_collides(first_entity: &MoveableSprite, second_entity: &MoveableSprite) -> bool {
    let (position_1, hitbox_1) = get_position_and_hitboxes(first_entity);
    let (position_2, hitbox_2) = get_position_and_hitboxes(second_entity);

    let collision = collide(position_1, hitbox_1, position_2, hitbox_2);

    collision.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_position_from_moveable_sprite() {
        let ennemy = ennemies::Ennemy::new(500.0, (5., 10.), (15., 20.), 50);
        let (ennemy_position, _) = get_position_and_hitboxes(ennemy.get_moveable_interface());

        assert_eq!(ennemy_position, Vec3::new(15., 20., 0.));
    }

    #[test]
    fn get_hitbox_from_moveable_sprite() {
        let ennemy = ennemies::Ennemy::new(500.0, (5., 10.), (15., 20.), 50);
        let (_, ennemy_hitbox) = get_position_and_hitboxes(ennemy.get_moveable_interface());
        let ennemy_hibox = ennemy.get_moveable_interface().get_hitbox_size();

        assert_eq!(ennemy_hitbox, Vec2::new(ennemy_hibox.0, ennemy_hibox.1));
    }

    /*
    TODO: Create a convienient test for this case.
    --- This test doesn't run... No clues why (Bevy engine? Integration issue? However, work correctly during gameplay) ---
    #[test]
    fn two_moveable_sprite_collides() {
        let ennemy = ennemies::Ennemy::new(500.0, (5., 10.), (15., 20.), (25., 30.), 50);
        let player = player::Player::new(500.0, (5., 10.), (15., 20.));

        assert_eq!(is_entities_collides(ennemy.get_moveable_interface(), player.get_moveable_interface()), true);
    }
    */

    #[test]
    fn two_moveable_sprite_not_collides() {
        let ennemy = ennemies::Ennemy::new(500.0, (5., 10.), (15., 20.), 50);
        let projectile = projectiles::Projectile::new(500.0, (5., 10.), (150., 2000.), 500, false);

        assert_eq!(
            is_entities_collides(
                ennemy.get_moveable_interface(),
                projectile.get_moveable_interface()
            ),
            false
        );
    }
}
