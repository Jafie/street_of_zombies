pub mod math_cartesian;

use crate::game_entity::*;

use rand::Rng;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide}
};


static MAXIMUM_ENNEMY_DISTANCE: f32 = 300.;
static INITIAL_ENNEMY_SPEED: f32 = 200.0;
// One spawn about each 10 seconds
static SPAWN_FACTOR_CLASSIC_ENNEMY: u32 = 650;


/// Game System: Automatic movement of the projectiles.
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

/// Game System: AI management for ennemies.
pub fn ennemy_ai_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut ennemy_query: Query<(&mut ennemies::Ennemy, &mut Transform)>) {
        movement_of_ennemies(&mut commands, &mut materials, &time, &mut ennemy_query);
        ennemy_spawn_system(&mut commands, &mut materials);
}

/// Game System: The collision system with projectiles.
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

pub fn keyboard_capture(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut player::Player, &mut Transform)>) {
    
    if let Ok((mut player, mut transform)) = query.single_mut() {
        let mut direction : (f32, f32) = (0.0, 0.0);
        let mut number_of_valid_pressure : u8 = 0;
    
        // Fire capture
        if keyboard_input.pressed(KeyCode::Space) {
            player.fire(&mut commands, &mut materials, &time);
        }
        else {
            player.reload_weapon();
        }

        // Movement        
        if keyboard_input.pressed(KeyCode::Left) {
            direction.0 -= 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.0 += 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.1 += 1.0;
            number_of_valid_pressure += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.1 -= 1.0;
            number_of_valid_pressure += 1;
        }

        match number_of_valid_pressure {
            0 => return,
            1 => (),
            _ => { 
                direction.0 = direction.0 / 1.5;
                direction.1 = direction.1 / 1.5;
            }
        }


        player.move_sprite(&time, &direction, &mut transform.translation);
    }
}

fn movement_of_ennemies(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    time: &Res<Time>,
    ennemy_query: &mut Query<(&mut ennemies::Ennemy, &mut Transform)>) {
        for (mut ennemy, mut ennemy_transform) in ennemy_query.iter_mut() {
            let ennemy_direction = ennemy.get_direction();
            ennemy.move_sprite(time, &ennemy_direction, &mut ennemy_transform.translation);
    
            if math_cartesian::calculate_cartesian_distance(ennemy.get_initial_position(), ennemy.get_position()) > MAXIMUM_ENNEMY_DISTANCE {
                // Reverse direction
                ennemy.set_new_direction((-ennemy_direction.0, -ennemy_direction.1));
            }
    
            // Attack
            ennemy.launch_attack(commands, materials, time)
        }
}

fn ennemy_spawn_system(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>) 
{
    let mut rng = rand::thread_rng();
    let rand_system = rng.gen_range(0..SPAWN_FACTOR_CLASSIC_ENNEMY);

    
    if rand_system == 1 {
        generate_new_ennemy(commands, materials);
    }
}

fn generate_new_ennemy(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,) {

    let mut rng = rand::thread_rng();

    let ennemy_initial_position: (f32, f32) = (rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
    let ennemy_initial_direction: (f32, f32) = (rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
    let ennemy_fire_direction: (f32, f32) = (rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

    // Ennemy
    commands
    .spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.0, 0.3).into()),
        transform: Transform::from_xyz(ennemy_initial_position.0, ennemy_initial_position.1, 0.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    })
    .insert(ennemies::Ennemy::new(
        INITIAL_ENNEMY_SPEED,
        ennemy_initial_direction,
        ennemy_initial_position,
        ennemy_fire_direction)
    );
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
