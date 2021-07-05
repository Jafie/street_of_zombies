pub mod math_cartesian;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide}
};

use crate::game_entity::*;

// Fire projectiles
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
        if projectile.is_out_of_distance() {
            commands.entity(projectile_entity).despawn();
        }
    }
}

// Ennemy AI system
pub fn ennemy_ai_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut ennemy_query: Query<(&mut ennemies::Ennemy, &mut Transform)>) {

    for (mut ennemy, mut ennemy_transform) in ennemy_query.iter_mut() {
        let ennemy_direction = ennemy.get_direction();
        ennemy.move_sprite(&time, &ennemy_direction, &mut ennemy_transform.translation);

        if math_cartesian::calculate_cartesian_distance(ennemy.get_initial_position(), ennemy.get_position()) > 300. {
            // Reverse direction
            ennemy.set_new_direction((-ennemy_direction.0, -ennemy_direction.1));
        }

        // Attack
        ennemy.launch_attack(&mut commands, &mut materials, &time)
    }
}


pub fn projectile_collision_system(
    mut commands: Commands,
    mut query_set: QuerySet<(
        Query<(&mut ennemies::Ennemy, &Transform, &Sprite, Entity)>,
        Query<(&mut player::Player, &Transform, &Sprite, Entity)>
    )>,
    projectile_query: Query<(Entity, &projectiles::Projectile, &Transform, &Sprite)>,
) 
{ 

    // check collision with objects
    for (collider_entity, projectile, transform, sprite) in projectile_query.iter() {
        if projectile.is_coming_from_ennemy() {
            check_collision_with_player(&mut commands, &mut query_set.q1_mut(), sprite, &collider_entity, transform);
        }
        else {
            check_collision_with_ennemy(&mut commands, &mut query_set.q0_mut(), sprite, &collider_entity, transform);
        }

    }


}

fn check_collision_with_ennemy(
    commands: &mut Commands,
    entity_query: &mut Query<(&mut ennemies::Ennemy, &Transform, &Sprite, Entity)>,
    projectile_sprite: &Sprite,
    projectile_entity: &Entity,
    projectile_transform: &Transform) {
    
    for (mut ennemy, ennemy_transform, sprite, entity_ennemy) in entity_query.iter_mut() {
        let ennemy_size = sprite.size;
        let collision = collide(
            ennemy_transform.translation,
            ennemy_size,
            projectile_transform.translation,
            projectile_sprite.size,
        );
        if let Some(_) = collision {
                commands.entity(*projectile_entity).despawn();
                ennemy.reduce_life();
                check_and_treat_ennemy_life(commands, &mut ennemy, entity_ennemy);
        }
    }
}

fn check_collision_with_player(
    commands: &mut Commands,
    entity_query: &mut Query<(&mut player::Player, &Transform, &Sprite, Entity)>,
    projectile_sprite: &Sprite,
    projectile_entity: &Entity,
    projectile_transform: &Transform) {
    
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
        }
    }
}


fn check_and_treat_ennemy_life(commands: &mut Commands, ennemy: &mut ennemies::Ennemy, entity: Entity) {
    if ennemy.is_dead() {
        commands.entity(entity).despawn();
    }
}
