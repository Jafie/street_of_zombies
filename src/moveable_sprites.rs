use bevy::{
    prelude::*,
};


// Traits definition
pub trait MoveableSprite {
    fn new(speed: f32, direction: (f32, f32), initial_pos: (f32, f32)) -> Self;
    fn get_speed(&self) -> f32;
    fn get_direction(&self) -> (f32, f32);
    fn set_new_direction(&mut self, direction: (f32, f32));
    fn get_position(&self) -> (f32, f32);
    fn set_new_position(&mut self, position: (f32, f32));
    fn move_sprite(&mut self, time: &Res<Time>, direction: &(f32, f32), translated_movement: &mut bevy::prelude::Vec3) {
        // move the sprite
        translated_movement.x += time.delta_seconds() * direction.0 * &self.get_speed();
        translated_movement.y += time.delta_seconds() * direction.1 * &self.get_speed();
        self.set_new_direction(*direction);
        self.set_new_position((translated_movement.x, translated_movement.y));
    }
    fn get_sprite_name() -> String;
}

pub trait Weapon {
    fn fire_global(&mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        direction: (f32, f32),
        initial_pos: (f32, f32)) {
            if self.get_amo() > 0 {
            let (pos_x, pox_y) = initial_pos;
            commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.3, 0.0, 1.0).into()),
                transform: Transform::from_xyz(pos_x, pox_y, 0.0),
                sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                ..Default::default()
            })
            .insert(Projectile::new(500.0, direction, initial_pos));
            self.reduce_amo();
        }
    }
    fn reload(&mut self);
    fn get_amo(&self) -> u32;
    fn reduce_amo(&mut self);
}


// The main character, moveable by user
pub struct MainCharacter {
    speed: f32,
    current_position : (f32, f32),
    direction: (f32, f32),
    amo : u32
}

impl MoveableSprite for MainCharacter {
    fn new(speed_to_set: f32, direction_to_set: (f32, f32), initial_pos: (f32, f32)) -> Self {
        MainCharacter { speed: speed_to_set, current_position: initial_pos, direction: direction_to_set, amo: 1 }
    }
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
        println!("New position of {} is equal to {:?}", MainCharacter::get_sprite_name(), self.current_position);
        self.current_position = position;
    }
    fn get_sprite_name() -> String {
        String::from("Josay")
    }
}

impl Weapon for MainCharacter {
    fn reload(&mut self) {
        self.amo = 1;
    }
    fn get_amo(&self) -> u32 {
        self.amo
    }
    fn reduce_amo(&mut self) {
        self.amo -= 1;
    }
}

impl MainCharacter {
    pub fn fire(&mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>) {
            self.fire_global(commands, materials, self.get_direction(), self.get_position());
    }
}

// PROJECTILE DEFINITION
pub struct Projectile {
    speed: f32,
    direction: (f32, f32),
    current_position : (f32, f32)
}

impl MoveableSprite for Projectile {
    fn new(speed_to_set: f32, direction_to_set: (f32, f32), initial_pos: (f32, f32)) -> Self {
        Projectile { speed: speed_to_set, direction: direction_to_set,  current_position: initial_pos}
    }

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
        println!("New position of {} is equal to {:?}", Projectile::get_sprite_name(), self.current_position);
        self.current_position = position;
    }
    fn get_sprite_name() -> String {
        String::from("Amo")
    }
}
