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
        .insert(moveable_sprites::main_character::MainCharacter::new(INITIAL_SPEED_MAIN_CHAR, INITIAL_DIRECTION_MAIN_CHAR, (INITIAL_POS_MAIN_CHAR_X, INITIAL_POS_MAIN_CHAR_Y)))
        .insert(Collider::MainCharacterCollision);

    // Ennemy
    commands
    .spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.0, 0.3).into()),
        transform: Transform::from_xyz(INITIAL_POS_ENN_CHAR_X, INITIAL_POS_ENN_CHAR_Y, 0.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    })
    .insert(moveable_sprites::ennemies::Ennemy::new(INITIAL_SPEED_MAIN_CHAR, INITIAL_DIRECTION_MAIN_CHAR, (INITIAL_POS_ENN_CHAR_X, INITIAL_POS_ENN_CHAR_Y)))
    .insert(Collider::EnnemyCollision);

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

fn projectile_collision_system(
    mut commands: Commands,
    mut ennemy_query: Query<(&mut moveable_sprites::ennemies::Ennemy, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) 
{
    for (_, ennemy_transform, sprite) in ennemy_query.iter_mut() {
        let ennemy_size = sprite.size;

        // check collision with objects
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ennemy_transform.translation,
                ennemy_size,
                transform.translation,
                sprite.size,
            );

            if let Some(_) = collision {
                // Check if the ennemy encounter a projectile
                if let Collider::ProjectileCollision = *collider {
                    commands.entity(collider_entity).despawn();
                }
            }
        }
    }
}
