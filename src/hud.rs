//! The text over the game: score, health, difficulty and active bonus, or the game over screen.

use bevy::prelude::*;

use crate::bonus::BonusSlot;
use crate::combat::Health;
use crate::game::{GameAssets, GameSet, GameState};
use crate::player::Player;
use crate::progression::{Difficulty, Score};

const FONT_SIZE: f32 = 40.0;
const SCORE_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const HEALTH_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const DIFFICULTY_COLOR: Color = Color::WHITE;
const BONUS_COLOR: Color = Color::srgb(1.0, 0.84, 0.0);
/// Shown instead of the progress towards the next level once the maximum level is reached
const MAX_DIFFICULTY_PERCENT: u32 = 666;
/// Offset of the game over text from the top-left corner, in percent of the window size
const GAME_OVER_OFFSET_PERCENT: f32 = 25.0;

/// The HUD is one `Text` with three `TextSpan` children, written by section index:
/// section 0 is the root `Text`, sections 1 to 3 are the spans in spawn order.
/// Each array below holds one entry per section, so they cannot get out of sync.
type HudSections = [String; 4];
const SPAN_COLORS: [Color; 3] = [HEALTH_COLOR, DIFFICULTY_COLOR, BONUS_COLOR];

/// Marks the HUD text
#[derive(Component)]
struct Hud;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_hud_system)
        .add_systems(OnEnter(GameState::Playing), place_hud_for_playing_system)
        .add_systems(OnEnter(GameState::GameOver), place_hud_for_game_over_system)
        .add_systems(Update, update_hud_system.in_set(GameSet::Presentation));
}

fn spawn_hud_system(mut commands: Commands, assets: Res<GameAssets>) {
    let text_font = TextFont {
        font: assets.font.clone().into(),
        font_size: FontSize::Px(FONT_SIZE),
        ..Default::default()
    };

    commands
        .spawn((
            Hud,
            Text::new(""),
            text_font.clone(),
            TextColor(SCORE_COLOR),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            for span_color in SPAN_COLORS {
                parent.spawn((TextSpan::new(""), text_font.clone(), TextColor(span_color)));
            }
        });
}

fn place_hud_for_playing_system(mut hud: Query<&mut Node, With<Hud>>) {
    for mut node in hud.iter_mut() {
        node.top = Val::Px(0.0);
        node.left = Val::Px(0.0);
    }
}

fn place_hud_for_game_over_system(mut hud: Query<&mut Node, With<Hud>>) {
    for mut node in hud.iter_mut() {
        node.top = Val::Percent(GAME_OVER_OFFSET_PERCENT);
        node.left = Val::Percent(GAME_OVER_OFFSET_PERCENT);
    }
}

fn update_hud_system(
    state: Res<State<GameState>>,
    score: Res<Score>,
    difficulty: Res<Difficulty>,
    player: Query<(&Health, &BonusSlot), With<Player>>,
    hud: Single<Entity, With<Hud>>,
    mut writer: TextUiWriter,
) {
    let sections = match state.get() {
        GameState::Playing => {
            // The player is gone for the frame between its death and the game over screen
            let (health, bonus_status) = player
                .single()
                .map(|(health, bonus_slot)| (health.current(), bonus_slot.status()))
                .unwrap_or((0, None));

            playing_sections(score.value(), health, &difficulty, bonus_status)
        }
        GameState::GameOver => game_over_sections(score.value()),
    };

    for (index, section_text) in sections.into_iter().enumerate() {
        let mut text = writer.text(*hud, index);
        // Only write changes, so the text is not laid out again every frame
        if *text != section_text {
            *text = section_text;
        }
    }
}

fn playing_sections(
    score: u32,
    health: u32,
    difficulty: &Difficulty,
    bonus_status: Option<(&str, u32)>,
) -> HudSections {
    [
        format!("SCORE: {:10}", score),
        format!(" - HEALTH: {:2}", health),
        format!(
            " -  DIFFICULTY : {:20} - {:3}%",
            difficulty.name(),
            difficulty
                .percent_until_next_level()
                .unwrap_or(MAX_DIFFICULTY_PERCENT)
        ),
        format_bonus_text(bonus_status),
    ]
}

fn game_over_sections(score: u32) -> HudSections {
    [
        "- GAME OVER -    ".to_string(),
        format!("Score =  {:10}\n", score),
        " - PRESS R TO RESTART -".to_string(),
        String::new(),
    ]
}

/// Format the text of the active bonus. Empty when no bonus is active.
///
/// # Arguments
///
/// * `bonus_status` - The name of the active bonus and its remaining seconds, if any
fn format_bonus_text(bonus_status: Option<(&str, u32)>) -> String {
    match bonus_status {
        // New line: the first line of the HUD is already full
        Some((bonus_name, remaining_seconds)) => {
            format!("\n{}: {}s", bonus_name, remaining_seconds)
        }
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_text_with_active_bonus_test() {
        assert_eq!(
            format_bonus_text(Some(("MACHINE GUN", 7))),
            "\nMACHINE GUN: 7s"
        );
    }

    #[test]
    fn bonus_text_without_bonus_test() {
        assert_eq!(format_bonus_text(None), "");
    }

    #[test]
    fn playing_sections_test() {
        let sections = playing_sections(5050, 4, &Difficulty::default(), None);

        assert_eq!(sections[0], "SCORE:       5050");
        assert_eq!(sections[1], " - HEALTH:  4");
        assert!(sections[2].contains("EASY"));
        assert!(sections[2].ends_with("  0%"));
        assert_eq!(sections[3], "");
    }

    #[test]
    fn game_over_sections_test() {
        let sections = game_over_sections(5050);

        assert!(sections[1].contains("5050"));
        assert!(sections[2].contains("PRESS R"));
        assert_eq!(sections[3], "");
    }
}
