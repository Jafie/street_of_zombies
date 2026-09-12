//! The text over the game: the scoreboard in the black bands around the play area, and the game
//! over panel.

mod game_over;
mod scoreboard;

use bevy::prelude::*;

/// Dark red of the frames around the scoreboard and the game over panel
const ACCENT_COLOR: Color = Color::srgb(0.55, 0.06, 0.06);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.84, 0.0);

pub fn plugin(app: &mut App) {
    app.add_plugins((scoreboard::plugin, game_over::plugin));
}

fn text_font(font: &Handle<Font>, font_size: f32) -> TextFont {
    TextFont {
        font: font.clone().into(),
        font_size: FontSize::Px(font_size),
        ..Default::default()
    }
}

/// Zero-padded to 7 digits, like an arcade score
fn format_score(score: u32) -> String {
    format!("{:07}", score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_is_zero_padded_test() {
        assert_eq!(format_score(5050), "0005050");
    }

    #[test]
    fn long_score_is_not_truncated_test() {
        assert_eq!(format_score(123_456_789), "123456789");
    }
}
