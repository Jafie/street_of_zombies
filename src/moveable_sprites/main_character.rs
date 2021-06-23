use crate::moveable_sprites::MoveableSprite;
use crate::moveable_sprites::ennemies::Ennemy;
use crate::weapons::Weapon;
use crate::weapons::Pistol;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide},
};


/// The Main Character. Controllable by the player.
pub struct MainCharacter {
    speed: f32,
    current_position : (f32, f32),
    direction: (f32, f32),
    current_weapon: Box<dyn Weapon + Send + Sync>
}

impl MoveableSprite for MainCharacter {
    fn get_speed(&self) -> f32 {
        self.speed
    }
    fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.direction = direction;
    }
    fn get_direction(&self) -> (f32, f32) {
        self.direction
    }
    fn get_position(&self) -> (f32, f32) {
        self.current_position
    }
    fn set_new_position(&mut self, position: (f32, f32)) {
        self.current_position = position;
    }
}

impl MainCharacter {
    pub fn new(speed_to_set: f32, direction_to_set: (f32, f32), initial_pos: (f32, f32)) -> Self {
        MainCharacter {
             speed: speed_to_set,
             current_position: initial_pos,
             direction: direction_to_set,
             current_weapon: Box::new(Pistol::new())}
    }

    pub fn fire(&mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        time: &Res<Time>) {
            self.current_weapon.fire_global(commands, materials, &time, self.get_direction(), self.get_position());
    }

    pub fn reload_weapon(&mut self) {
        self.current_weapon.reload();
    }
}

pub fn keyboard_capture(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut MainCharacter, &mut Transform, &Sprite)>,
    query_ennemy: Query<(&Ennemy, &Sprite)>
) {
    if let Ok((mut main_character, mut transform, main_char_sprite)) = query.single_mut() {
        let mut direction : (f32, f32) = (0.0, 0.0);
        let mut number_of_valid_pressure : u8 = 0;
        let saved_position = (transform.translation.x, transform.translation.y);
    
        // Fire capture
        if keyboard_input.pressed(KeyCode::Space) {
            main_character.fire(&mut commands, &mut materials, &time);
        }
        else {
            main_character.reload_weapon();
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


        main_character.move_sprite(&time, &direction, &mut transform.translation);

        // Block the movement if a collision with an object is detected
        if is_next_movement_collide_with_blockable_object(&query_ennemy, &transform, &main_char_sprite) {
            transform.translation.x = saved_position.0;
            transform.translation.y = saved_position.1;
        }
    }
}

fn is_next_movement_collide_with_blockable_object(query_ennemy: &Query<(&Ennemy, &Sprite)>, movement: &Transform, main_char_sprite: &Sprite) -> bool {
    for (ennemy, sprite) in query_ennemy.iter() {

           let ennemy_position : bevy::prelude::Vec3 = bevy::prelude::Vec3::new(ennemy.get_position().0, ennemy.get_position().1, 0.0);

           let collision = collide(
                movement.translation,
                main_char_sprite.size,
                ennemy_position,
                sprite.size,
            );

            if let Some(_) = collision {
                    return true;
            }
    }

    false
}