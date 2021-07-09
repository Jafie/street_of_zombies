
use bevy::{
    prelude::*
};
use crate::game_entity::*;

static ZOMBIE_ASSET_PATH: &'static str = "sprites/zombie.png";
static PLAYER_ASSET_PATH: &'static str = "sprites/woman.png";

enum TexturePositionEnum {
    DOWN,
    LEFT,
    RIGHT,
    UP
}

static COLS_PER_SPRITES : usize= 8;

pub fn animate_sprite_system(
    time: Res<Time>,
    mut query_set: QuerySet<(
        Query<(&mut ennemies::Ennemy, &mut Timer, &mut TextureAtlasSprite)>,
        Query<(&mut player::Player, &mut Timer, &mut TextureAtlasSprite)>
    )>
) {

    for (mut ennemy, mut timer, mut sprite) in query_set.q0_mut().iter_mut() {
        animate_sprite(&mut ennemy.get_moveable_interface_mut(), &time, &mut timer, &mut sprite);
    }

    if let Ok((mut player, mut timer, mut sprite)) = query_set.q1_mut().single_mut() {
        animate_sprite(&mut player.get_moveable_interface_mut(), &time, &mut timer, &mut sprite);
    }
}

fn animate_sprite(
    entity: &mut MoveableSprite,
    time: &Res<Time>,
    timer: &mut Timer,
    sprite: &mut Mut<TextureAtlasSprite>
)
{
    timer.tick(time.delta());
    if timer.finished() && entity.is_sprite_moved_after_last_call() {
        let coef_val : usize;

        match generate_texture_position_from_coeff_factor(entity.get_direction()) {
            TexturePositionEnum::DOWN => coef_val = 0*COLS_PER_SPRITES,
            TexturePositionEnum::LEFT => coef_val = 1*COLS_PER_SPRITES,
            TexturePositionEnum::RIGHT => coef_val = 2*COLS_PER_SPRITES,
            TexturePositionEnum::UP => coef_val = 3*COLS_PER_SPRITES
        }

        let calculated_index =  (((sprite.index as usize + 1) % COLS_PER_SPRITES) + coef_val) as u32;
        sprite.index = calculated_index;
    }
}

fn generate_texture_position_from_coeff_factor(coeff_factor: (f32, f32)) ->  TexturePositionEnum {
    let texture_direction: TexturePositionEnum;
    let (coeff_factor_x, coeff_factor_y) = coeff_factor;

    let cond_1 = coeff_factor_x.abs() > coeff_factor_y.abs();
    let cond_2 = (coeff_factor_x - coeff_factor_y) >= 0.;

    if  (coeff_factor_x.abs() - coeff_factor_y.abs()) == 0. {
        // Special case, strict coeff factor... Use Y
        if coeff_factor_y > 0. {
            texture_direction = TexturePositionEnum::UP
        }
        else {
            texture_direction = TexturePositionEnum::DOWN
        }

    }
    else {
        match (cond_1, cond_2) {
            (true, true) => texture_direction = TexturePositionEnum::RIGHT,
            (true, false) => texture_direction = TexturePositionEnum::LEFT,
            (false, true) => texture_direction = TexturePositionEnum::DOWN,
            (false, false) => texture_direction = TexturePositionEnum::UP
        }
    }

    texture_direction
}

pub enum TextureToGenerate {
    PLAYER,
    ZOMBIE
}

pub fn generate_texture(
    asset_server: & Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    texture_type: TextureToGenerate) -> Handle<TextureAtlas>
    
{
    let texture_path: &str;

    match texture_type {
        TextureToGenerate::PLAYER => texture_path = PLAYER_ASSET_PATH,
        TextureToGenerate::ZOMBIE => texture_path = ZOMBIE_ASSET_PATH,
    }

    let texture_handle = asset_server.load(texture_path);
    let generated_texture = TextureAtlas::from_grid(texture_handle, Vec2::new(80.0, 80.0), 8, 4);
    
    texture_atlases.add(generated_texture)
}
