use crate::game_entity::*;
use crate::game_system::*;
use crate::sprite_manager_system::*;
use rand::Rng;

use bevy::prelude::*;

static MAXIMUM_NUMBER_OF_ENNEMIES: usize = 40;

/// Game System: AI management for ennemies  and manage the part "Difficulty" of the score system. Managed by as a "Bevy System"
pub fn ennemy_ai_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut ennemy_query: Query<(&mut ennemies::Ennemy, &mut Transform)>,
    scoreboard_query: Query<&scoreboard::ScoreAndInfo>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let current_scoreboard = scoreboard_query.single().unwrap();
    movement_of_ennemies(&mut commands, &mut materials, &time, &mut ennemy_query);

    let ennemies_spawned = ennemy_query.iter_mut().count();

    if ennemies_spawned < MAXIMUM_NUMBER_OF_ENNEMIES {
        ennemy_spawn_system(
            &mut commands,
            current_scoreboard.get_difficulty_level(),
            &asset_server,
            &mut texture_atlases,
        );
    }
}

fn movement_of_ennemies(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    time: &Res<Time>,
    ennemy_query: &mut Query<(&mut ennemies::Ennemy, &mut Transform)>,
) {
    for (mut ennemy, mut ennemy_transform) in ennemy_query.iter_mut() {
        let ennemy_direction = ennemy.get_moveable_interface().get_direction();
        ennemy.get_moveable_interface_mut().move_sprite(
            time,
            &ennemy_direction,
            &mut ennemy_transform.translation,
        );

        if (math_and_generator::calculate_cartesian_distance(
            ennemy.get_initial_position(),
            ennemy.get_moveable_interface().get_position(),
        ) > MAXIMUM_ENNEMY_DISTANCE)
            || (is_next_movement_out_of_game_area(
                ennemy.get_moveable_interface().get_position(),
                ennemy_direction,
            ))
        {
            // Reverse direction
            ennemy
                .get_moveable_interface_mut()
                .set_new_direction((-ennemy_direction.0, -ennemy_direction.1));
        }

        // Attack
        ennemy.launch_attack(commands, materials, time)
    }
}

fn ennemy_spawn_system(
    commands: &mut Commands,
    difficulty_level: u32,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    static SPAWN_FACTOR_CLASSIC_ENNEMY: u32 = 1100;
    let generated_spawn_factor = SPAWN_FACTOR_CLASSIC_ENNEMY - (200 * difficulty_level);

    let mut rng = rand::thread_rng();
    let rand_system = rng.gen_range(0..generated_spawn_factor);

    if rand_system <= 2 {
        generate_new_ennemy(commands, asset_server, texture_atlases);
    }
}

fn generate_new_ennemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    // Random generation
    let ennemy_initial_position: (f32, f32) =
        math_and_generator::generate_random_position(GAME_AREA_LIMIT_X, GAME_AREA_LIMIT_Y);
    let ennemy_initial_direction: (f32, f32) =
        math_and_generator::generate_random_direction_factor();
    let ennemy_fire_direction: (f32, f32) =
        math_and_generator::generate_random_direction_factor_strict();

    // Ennemy
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: generate_texture(
                asset_server,
                texture_atlases,
                TextureToGenerate::ZOMBIE,
            ),
            transform: Transform::from_xyz(
                ennemy_initial_position.0,
                ennemy_initial_position.1,
                0.0,
            ),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .insert(ennemies::Ennemy::new(
            INITIAL_ENNEMY_SPEED,
            ennemy_initial_direction,
            ennemy_initial_position,
            ennemy_fire_direction,
            50,
        ))
        .insert(Timer::from_seconds(0.1, true));
}
