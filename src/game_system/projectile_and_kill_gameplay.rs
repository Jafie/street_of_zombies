use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide}
};

use crate::game_entity::*;
use crate::game_system::*;

/// Game System: Automatic movement of the projectiles. Managed by as a "Bevy System"
pub fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(&mut projectiles::Projectile,
    &mut Transform, Entity)>) {

    for projectile_single_query in projectile_query.iter_mut() {
        let (mut projectile, mut transform, projectile_entity) = projectile_single_query;
        let direction_of_fire = projectile.get_direction();
        projectile.move_sprite(&time, &direction_of_fire, &mut transform.translation);

        // If outside of game area, delete
        if (projectile.is_out_of_distance())
        || is_next_movement_out_of_game_area(projectile.get_position(), projectile.get_direction()) {
            commands.entity(projectile_entity).despawn();
        }
    }
}

/// Game System: The collision system with projectiles and manage the part "health + Score" of the score system. Managed by as a "Bevy System"
pub fn projectile_collision_and_score_system(
    mut commands: Commands,
    mut query_set: QuerySet<(
        Query<(&mut ennemies::Ennemy, &Transform, Entity)>,
        Query<(&mut player::Player, &Transform, &Sprite, Entity)>
    )>,
    projectile_query: Query<(Entity, &projectiles::Projectile, &Transform, &Sprite)>,
    mut scoreboard_query: Query<(&mut scoreboard::ScoreAndInfo, &mut Text)>
) 
{ 
    let (mut score_struct, mut score_text) = scoreboard_query.single_mut().unwrap();

    // check collision with objects
    for (collider_entity, projectile, transform, sprite) in projectile_query.iter() {
        if projectile.is_coming_from_ennemy() {
            check_collision_with_player(&mut commands, 
                &mut query_set.q1_mut(), 
                sprite, 
                &collider_entity, 
                transform, 
                &mut score_struct);
        }
        else {
            check_collision_with_ennemy(&mut commands, 
                &mut query_set.q0_mut(), 
                sprite, 
                &collider_entity, 
                transform, 
                &mut score_struct);
        }
    }

    score_struct.update_percent_until_next_level();
    score_struct.update_scoarboard_text(&mut score_text);
}

fn check_collision_with_ennemy(
    commands: &mut Commands,
    entity_query: &mut Query<(&mut ennemies::Ennemy, &Transform, Entity)>,
    projectile_sprite: &Sprite,
    projectile_entity: &Entity,
    projectile_transform: &Transform,
    score_struct: &mut scoreboard::ScoreAndInfo) {

    for (mut ennemy, ennemy_transform, entity_ennemy) in entity_query.iter_mut() {
        let (hibox_x, hitbox_y) = ennemy.get_hitbox_size();
        let ennemy_size = Vec2::new(hibox_x, hitbox_y);
    
        let collision = collide(
            ennemy_transform.translation,
            ennemy_size,
            projectile_transform.translation,
            projectile_sprite.size,
        );
        if let Some(_) = collision {
                commands.entity(*projectile_entity).despawn();
                ennemy.reduce_health();
                score_struct.add_to_score(ennemy.get_point_value_per_hits());
                check_and_treat_ennemy_health(commands, &mut ennemy, entity_ennemy, score_struct);
        }
    }
}

fn check_collision_with_player(
    commands: &mut Commands,
    entity_query: &mut Query<(&mut player::Player, &Transform, &Sprite, Entity)>,
    projectile_sprite: &Sprite,
    projectile_entity: &Entity,
    projectile_transform: &Transform,
    score_struct: &mut scoreboard::ScoreAndInfo) {
    
    for (_, player_transform, sprite, _) in entity_query.iter_mut() {
        let player_size = sprite.size;
        let collision = collide(
            player_transform.translation,
            player_size,
            projectile_transform.translation,
            projectile_sprite.size,
        );
        if let Some(_) = collision {
                commands.entity(*projectile_entity).despawn();
                score_struct.remove_health(1);
        }
    }
}


fn check_and_treat_ennemy_health(commands: &mut Commands, ennemy: &mut ennemies::Ennemy, entity: Entity, score: &mut scoreboard::ScoreAndInfo) {
    if ennemy.is_dead() {
        score.add_to_score(ennemy.get_point_value_on_death());
        commands.entity(entity).despawn();
    }
}
