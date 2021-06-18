use bevy::{
    prelude::*,
};


// Traits definition
pub trait MoveableSprite {
    fn get_speed(&self) -> f32;
    fn set_new_direction(&mut self, direction: (f32, f32));
    fn move_sprite(&self, time: &Res<Time>, direction: &(f32, f32), translated_movement: &mut bevy::prelude::Vec3) {
        // move the sprite
        translated_movement.x += time.delta_seconds() * direction.0 * &self.get_speed();
        translated_movement.y += time.delta_seconds() * direction.1 * &self.get_speed();
    }
}

pub trait ArmedSprite {
    fn fire_projectile(&self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        direction: (f32, f32)) {
            
        commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(5.0, 5.0)),
            ..Default::default()
        })
        .insert(Projectile { speed: 500.0, direction_factor: direction });
    }
}

// The main character, moveable by user
pub struct MainCharacter {
    pub speed: f32,
    pub direction: (f32, f32)
}

impl MoveableSprite for MainCharacter {
    fn get_speed(&self) -> f32 {
        self.speed
    }
    fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.direction = direction;
    }

}

impl ArmedSprite for MainCharacter {
}

// PROJECTILE DEFINITION
pub struct Projectile {
    speed: f32,
    direction_factor: (f32, f32)
}

impl MoveableSprite for Projectile {
    fn get_speed(&self) -> f32 {
        self.speed
    }
    fn set_new_direction(&mut self, direction: (f32, f32)) {
        self.direction_factor = direction;
    }
}
