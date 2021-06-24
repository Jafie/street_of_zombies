mod moveable_sprites;
mod weapons;
mod math_cartesian;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide},
};

use crate::moveable_sprites::*;


// Game area limit
static GAME_AREA_LIMIT_X: f32 = 500.0;
static GAME_AREA_LIMIT_Y: f32 = 300.0;


// Main character initialization
static INITIAL_POS_MAIN_CHAR_X: f32 = 0.0;
static INITIAL_POS_MAIN_CHAR_Y: f32 = -215.0;

static INITIAL_POS_ENN_CHAR_X: f32 = 0.0;
static INITIAL_POS_ENN_CHAR_Y: f32 = 215.0;

static INITIAL_SPEED_MAIN_CHAR: f32 = 200.0;
static INITIAL_DIRECTION_MAIN_CHAR: (f32, f32) = (0.0, 1.0);


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "street_of_zombies".to_string(),
            width: GAME_AREA_LIMIT_X,
            height: GAME_AREA_LIMIT_Y,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_system(set_window_title.system())
        .add_system(main_character::keyboard_capture.system())
        .add_system(projectile_movement_system.system())
        .add_system(projectile_collision_system.system())
        .add_system(ennemy_ai_system.system())
        .run();
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());


    // Main character
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.0, 1.0).into()),
            transform: Transform::from_xyz(INITIAL_POS_MAIN_CHAR_X, INITIAL_POS_MAIN_CHAR_Y, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(moveable_sprites::main_character::MainCharacter::new(INITIAL_SPEED_MAIN_CHAR, INITIAL_DIRECTION_MAIN_CHAR, (INITIAL_POS_MAIN_CHAR_X, INITIAL_POS_MAIN_CHAR_Y)));
    // Ennemy
    commands
    .spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.0, 0.3).into()),
        transform: Transform::from_xyz(INITIAL_POS_ENN_CHAR_X, INITIAL_POS_ENN_CHAR_Y, 0.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    })
    .insert(moveable_sprites::ennemies::Ennemy::new(INITIAL_SPEED_MAIN_CHAR, INITIAL_DIRECTION_MAIN_CHAR, (INITIAL_POS_ENN_CHAR_X, INITIAL_POS_ENN_CHAR_Y)));
}

/// This system will then change the title during execution
fn set_window_title(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(String::from("Street of Zombies"));
}

// Fire projectiles
fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(&mut moveable_sprites::projectiles::Projectile,
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
fn ennemy_ai_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(&mut moveable_sprites::ennemies::Ennemy, &mut Transform)>) {


}


fn projectile_collision_system(
    mut commands: Commands,
    mut query_set: QuerySet<(
        Query<(&mut moveable_sprites::ennemies::Ennemy, &Transform, &Sprite, Entity)>,
        Query<(&mut moveable_sprites::main_character::MainCharacter, &Transform, &Sprite, Entity)>
    )>,
    projectile_query: Query<(Entity, &projectiles::Projectile, &Transform, &Sprite)>,
) 
{
    for (mut ennemy, ennemy_transform, sprite, entity_ennemy) in query_set.q0_mut().iter_mut() {
        let ennemy_size = sprite.size;

        // check collision with objects
        for (collider_entity, _, transform, sprite) in projectile_query.iter() {
            let collision = collide(
                ennemy_transform.translation,
                ennemy_size,
                transform.translation,
                sprite.size,
            );

            if let Some(_) = collision {
                    commands.entity(collider_entity).despawn();
                    ennemy.reduce_life();
                    check_and_treat_ennemy_life(&mut commands, &mut ennemy, entity_ennemy);
            }
        }
    }
}

fn check_and_treat_ennemy_life(commands: &mut Commands, ennemy: &mut moveable_sprites::ennemies::Ennemy, entity: Entity) {
    if ennemy.is_dead() {
        commands.entity(entity).despawn();
    }
}