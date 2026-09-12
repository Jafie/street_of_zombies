//! A short flash of the sprite of anything hit, so every hit is felt.

use bevy::prelude::*;

use crate::combat::Hit;
use crate::game::GameSet;

/// How long a hit tints the sprite
const FLASH_DURATION_SECS: f32 = 0.1;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            start_hit_flash_system.in_set(GameSet::Resolution),
            update_hit_flash_system.in_set(GameSet::Presentation),
        ),
    );
}

/// Tints the sprite of an entity for a moment after it takes a hit
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct HitFlash {
    color: Color,
    remaining_secs: f32,
}

impl HitFlash {
    // The sprite color multiplies the texture, so a tint of 1 at most can only darken it. Channels
    // above 1 saturate once the output is clamped: the sprite turns into a silhouette of that
    // color, keeping its dark outlines.

    /// Turns the sprite bright red
    pub const RED: HitFlash = HitFlash::new(Color::linear_rgb(6., 0.4, 0.4));
    /// Turns the sprite white
    pub const WHITE: HitFlash = HitFlash::new(Color::linear_rgb(8., 8., 8.));

    const fn new(color: Color) -> Self {
        HitFlash {
            color,
            remaining_secs: 0.,
        }
    }

    /// Flash for the full duration, even when already flashing
    fn start(&mut self) {
        self.remaining_secs = FLASH_DURATION_SECS;
    }

    /// Let time pass
    fn update(&mut self, delta_secs: f32) {
        self.remaining_secs = (self.remaining_secs - delta_secs).max(0.);
    }

    /// The color the sprite shall be drawn with: the flash color while flashing, else no tint
    fn tint(&self) -> Color {
        if self.remaining_secs > 0. {
            self.color
        } else {
            Color::WHITE
        }
    }
}

/// Start the flash of every entity hit this frame
fn start_hit_flash_system(mut hits: MessageReader<Hit>, mut flashes: Query<&mut HitFlash>) {
    for hit in hits.read() {
        if let Ok(mut flash) = flashes.get_mut(hit.target) {
            flash.start();
        }
    }
}

/// Count the flashes down and tint the sprites accordingly
fn update_hit_flash_system(time: Res<Time>, mut query: Query<(&mut HitFlash, &mut Sprite)>) {
    for (mut flash, mut sprite) in query.iter_mut() {
        flash.update(time.delta_secs());

        let tint = flash.tint();
        if sprite.color != tint {
            sprite.color = tint;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_tint_before_any_hit() {
        assert_eq!(HitFlash::RED.tint(), Color::WHITE);
    }

    #[test]
    fn flash_color_after_a_hit() {
        let mut flash = HitFlash::RED;
        flash.start();

        assert_eq!(flash.tint(), HitFlash::RED.color);
    }

    #[test]
    fn flash_still_on_before_its_duration() {
        let mut flash = HitFlash::WHITE;
        flash.start();
        flash.update(FLASH_DURATION_SECS - 0.01);

        assert_eq!(flash.tint(), HitFlash::WHITE.color);
    }

    #[test]
    fn flash_ends_after_its_duration() {
        let mut flash = HitFlash::WHITE;
        flash.start();
        flash.update(FLASH_DURATION_SECS);

        assert_eq!(flash.tint(), Color::WHITE);
    }

    #[test]
    fn new_hit_restarts_the_flash() {
        let mut flash = HitFlash::RED;
        flash.start();
        flash.update(FLASH_DURATION_SECS - 0.01);
        flash.start();
        flash.update(FLASH_DURATION_SECS - 0.01);

        assert_eq!(flash.tint(), HitFlash::RED.color);
    }

    #[test]
    fn time_without_hit_does_not_shorten_the_next_flash() {
        let mut flash = HitFlash::RED;
        flash.update(5.);
        flash.start();
        flash.update(FLASH_DURATION_SECS - 0.01);

        assert_eq!(flash.tint(), HitFlash::RED.color);
    }
}
