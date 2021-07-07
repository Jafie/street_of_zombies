pub mod math_cartesian;
mod scoreboard;

use crate::game_entity::*;

use rand::Rng;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide}
};

// Game area limit
pub static GAME_AREA_LIMIT_X: f32 = 500.0;
pub static GAME_AREA_LIMIT_Y: f32 = 300.0;

static MAXIMUM_ENNEMY_DISTANCE: f32 = 300.;
static INITIAL_ENNEMY_SPEED: f32 = 200.0;

// Main character initialization
static INITIAL_PLAYER_POSITION: f32 = 0.0;
static INITIAL_PLAYER_POSITION_Y: f32 = -215.0;
static INITIAL_PLAYER_SPEED: f32 = 350.0;
static INITIAL_PLAYER_DIRECTION: (f32, f32) = (0.0, 1.0);

/// Initial setup
pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());


    // Main character
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.0, 1.0).into()),
            transform: Transform::from_xyz(INITIAL_PLAYER_POSITION, INITIAL_PLAYER_POSITION_Y, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(player::Player::new(INITIAL_PLAYER_SPEED, 
            INITIAL_PLAYER_DIRECTION,
            (INITIAL_PLAYER_POSITION, INITIAL_PLAYER_POSITION_Y)));

    // Scoreboard
    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "Life".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 1.0, 0.5),
                    },
                },
                TextSection {
                    value: "Difficulty".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(scoreboard::ScoreAndInfo::new());
}


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
        if (projectile.is_out_of_distance())
        || is_next_movement_out_of_game_area(projectile.get_position(), projectile.get_direction()) {
            commands.entity(projectile_entity).despawn();
        }
    }
}

/// Game System: AI management for ennemies  and manage the part "Difficulty" of the score system
pub fn ennemy_ai_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut ennemy_query: Query<(&mut ennemies::Ennemy, &mut Transform)>,
    scoreboard_query: Query<&scoreboard::ScoreAndInfo>
) {
    let current_scoreboard = scoreboard_query.single().unwrap();
    movement_of_ennemies(&mut commands, &mut materials, &time, &mut ennemy_query);
    ennemy_spawn_system(&mut commands, &mut materials, current_scoreboard.get_difficulty_level());
}

/// Game System: The collision system with projectiles and manage the part "Life + Score" of the score system
pub fn projectile_collision_and_score_system(
    mut commands: Commands,
    mut query_set: QuerySet<(
        Query<(&mut ennemies::Ennemy, &Transform, &Sprite, Entity)>,
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
    
            if (math_cartesian::calculate_cartesian_distance(ennemy.get_initial_position(), ennemy.get_position()) > MAXIMUM_ENNEMY_DISTANCE)
                || (is_next_movement_out_of_game_area(ennemy.get_position(), ennemy_direction)) {
                // Reverse direction
                ennemy.set_new_direction((-ennemy_direction.0, -ennemy_direction.1));
            }
    
            // Attack
            ennemy.launch_attack(commands, materials, time)
        }
}

fn ennemy_spawn_system(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    difficulty_level: u32) 
{
    static SPAWN_FACTOR_CLASSIC_ENNEMY: u32 = 1100;
    let generated_spawn_factor = SPAWN_FACTOR_CLASSIC_ENNEMY - (200*difficulty_level);

    let mut rng = rand::thread_rng();
    let rand_system = rng.gen_range(0..generated_spawn_factor);

    
    if rand_system <= 2 {
        generate_new_ennemy(commands, materials);
    }
}

fn generate_new_ennemy(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,) {

    // Random generation
    let ennemy_initial_position: (f32, f32) = math_cartesian::generate_random_position(GAME_AREA_LIMIT_X, GAME_AREA_LIMIT_Y);
    let ennemy_initial_direction: (f32, f32) = math_cartesian::generate_random_direction_factor();
    let ennemy_fire_direction: (f32, f32) = math_cartesian::generate_random_direction_factor_strict();

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
        ennemy_fire_direction,
        50)
    );
}

fn check_collision_with_ennemy(
    commands: &mut Commands,
    entity_query: &mut Query<(&mut ennemies::Ennemy, &Transform, &Sprite, Entity)>,
    projectile_sprite: &Sprite,
    projectile_entity: &Entity,
    projectile_transform: &Transform,
    score_struct: &mut scoreboard::ScoreAndInfo) {

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
                score_struct.add_to_score(ennemy.get_point_value_per_hits());
                check_and_treat_ennemy_life(commands, &mut ennemy, entity_ennemy, score_struct);
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
                score_struct.remove_life(1);
        }
    }
}


fn check_and_treat_ennemy_life(commands: &mut Commands, ennemy: &mut ennemies::Ennemy, entity: Entity, score: &mut scoreboard::ScoreAndInfo) {
    if ennemy.is_dead() {
        score.add_to_score(ennemy.get_point_value_on_death());
        commands.entity(entity).despawn();
    }
}

pub fn is_next_movement_out_of_game_area(position: (f32, f32), direction_factor: (f32, f32)) -> bool {
    let next_movement_coord : (f32, f32) = ((position.0 + direction_factor.0).abs(), (position.1 + direction_factor.1).abs());
    
    next_movement_coord.0 > GAME_AREA_LIMIT_X || next_movement_coord.1 > GAME_AREA_LIMIT_Y
}
