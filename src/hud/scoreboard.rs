//! The scoreboard in the black band above the play area: health, difficulty, progress towards the
//! next level and score. The active bonus is shown in the black band below the play area.

use bevy::prelude::*;

use super::{format_score, text_font, ACCENT_COLOR, SCORE_COLOR};
use crate::bonus::BonusSlot;
use crate::combat::Health;
use crate::game::{GameAssets, GameSet};
use crate::player::Player;
use crate::progression::{Difficulty, Score};

/// Height of the black band above the play area, in the background image
const TOP_BAND_HEIGHT: f32 = 43.0;
/// Height of the black band below the play area, in the background image
const BOTTOM_BAND_HEIGHT: f32 = 39.0;
const BAR_SIDE_PADDING: f32 = 20.0;
/// Thickness of the line between the scoreboard and the play area
const BAR_BORDER_WIDTH: f32 = 2.0;
/// Space between a caption and its value
const GROUP_GAP: f32 = 8.0;
/// Wide enough for the longest difficulty name, so the groups on its right stay in place
const DIFFICULTY_NAME_WIDTH: f32 = 150.0;
/// Wide enough for the percent shown at the maximum level
const PERCENT_WIDTH: f32 = 48.0;
const PROGRESS_BAR_WIDTH: f32 = 140.0;
const PROGRESS_BAR_HEIGHT: f32 = 12.0;

const LABEL_FONT_SIZE: f32 = 14.0;
const VALUE_FONT_SIZE: f32 = 24.0;
const PERCENT_FONT_SIZE: f32 = 18.0;
const BONUS_FONT_SIZE: f32 = 20.0;

/// A shade lighter than the black bands of the background image
const BAND_BACKGROUND_COLOR: Color = Color::srgb(0.04, 0.04, 0.05);
/// Color of the captions next to the values
const LABEL_COLOR: Color = Color::srgb(0.6, 0.6, 0.66);
const FULL_HEART_COLOR: Color = Color::srgb(0.92, 0.12, 0.18);
const EMPTY_HEART_COLOR: Color = Color::srgb(0.4, 0.4, 0.45);
const PERCENT_COLOR: Color = Color::WHITE;
const PROGRESS_TRACK_COLOR: Color = Color::srgb(0.15, 0.15, 0.18);
const BONUS_COLOR: Color = Color::srgb(0.3, 0.85, 1.0);
/// One color per difficulty level, from calm green to deadly purple. Higher levels use the last.
const LEVEL_COLORS: [Color; 6] = [
    Color::srgb(0.35, 0.85, 0.35),
    Color::srgb(0.65, 0.9, 0.3),
    Color::srgb(1.0, 0.8, 0.2),
    Color::srgb(1.0, 0.5, 0.15),
    Color::srgb(0.95, 0.2, 0.2),
    Color::srgb(0.8, 0.25, 1.0),
];

/// Drawn with the symbol font: the main font has no hearts
const FULL_HEART: char = '♥';
const EMPTY_HEART: char = '♡';
/// Shown instead of the progress towards the next level once the maximum level is reached
const MAX_DIFFICULTY_PERCENT: u32 = 666;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_scoreboard_system)
        .add_systems(Update, update_scoreboard_system.in_set(GameSet::Presentation));
}

/// A text of the scoreboard, rewritten when the value it shows changes
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum ScoreboardText {
    FullHearts,
    EmptyHearts,
    DifficultyName,
    DifficultyPercent,
    Score,
    Bonus,
}

/// The filled part of the progress bar towards the next difficulty level
#[derive(Component)]
struct ProgressFill;

/// The frame around the active bonus, hidden while no bonus is active
#[derive(Component)]
struct BonusBadge;

fn spawn_scoreboard_system(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(TOP_BAND_HEIGHT),
                padding: UiRect::horizontal(Val::Px(BAR_SIDE_PADDING)),
                border: UiRect::bottom(Val::Px(BAR_BORDER_WIDTH)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(BAND_BACKGROUND_COLOR),
            BorderColor::all(ACCENT_COLOR),
        ))
        .with_children(|bar| {
            bar.spawn(group_node()).with_children(|group| {
                group.spawn(label(&assets, "HEALTH"));
                // No gap between the full and the empty hearts
                group.spawn(Node::default()).with_children(|hearts| {
                    hearts.spawn(scoreboard_text(
                        ScoreboardText::FullHearts,
                        &assets.symbol_font,
                        VALUE_FONT_SIZE,
                        FULL_HEART_COLOR,
                    ));
                    hearts.spawn(scoreboard_text(
                        ScoreboardText::EmptyHearts,
                        &assets.symbol_font,
                        VALUE_FONT_SIZE,
                        EMPTY_HEART_COLOR,
                    ));
                });
            });

            bar.spawn(group_node()).with_children(|group| {
                group.spawn(label(&assets, "DIFFICULTY"));
                group.spawn((
                    scoreboard_text(
                        ScoreboardText::DifficultyName,
                        &assets.font,
                        VALUE_FONT_SIZE,
                        level_color(0),
                    ),
                    Node {
                        min_width: Val::Px(DIFFICULTY_NAME_WIDTH),
                        ..Default::default()
                    },
                ));
            });

            bar.spawn(group_node()).with_children(|group| {
                group.spawn(label(&assets, "NEXT LEVEL"));
                group
                    .spawn((
                        Node {
                            width: Val::Px(PROGRESS_BAR_WIDTH),
                            height: Val::Px(PROGRESS_BAR_HEIGHT),
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(PROGRESS_BAR_HEIGHT / 2.0)),
                            overflow: Overflow::clip(),
                            ..Default::default()
                        },
                        BackgroundColor(PROGRESS_TRACK_COLOR),
                        BorderColor::all(LABEL_COLOR),
                    ))
                    .with_children(|track| {
                        track.spawn((
                            ProgressFill,
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                border_radius: BorderRadius::all(Val::Px(
                                    PROGRESS_BAR_HEIGHT / 2.0,
                                )),
                                ..Default::default()
                            },
                            BackgroundColor(level_color(0)),
                        ));
                    });
                group.spawn((
                    scoreboard_text(
                        ScoreboardText::DifficultyPercent,
                        &assets.font,
                        PERCENT_FONT_SIZE,
                        PERCENT_COLOR,
                    ),
                    Node {
                        min_width: Val::Px(PERCENT_WIDTH),
                        ..Default::default()
                    },
                ));
            });

            bar.spawn(group_node()).with_children(|group| {
                group.spawn(label(&assets, "SCORE"));
                group.spawn(scoreboard_text(
                    ScoreboardText::Score,
                    &assets.font,
                    VALUE_FONT_SIZE,
                    SCORE_COLOR,
                ));
            });
        });

    // The active bonus, centered in the black band below the play area
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(BOTTOM_BAND_HEIGHT),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .with_children(|band| {
            band.spawn((
                BonusBadge,
                Node {
                    padding: UiRect::horizontal(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..Default::default()
                },
                BackgroundColor(BAND_BACKGROUND_COLOR),
                BorderColor::all(BONUS_COLOR),
                Visibility::Hidden,
            ))
            .with_children(|badge| {
                badge.spawn(scoreboard_text(
                    ScoreboardText::Bonus,
                    &assets.font,
                    BONUS_FONT_SIZE,
                    BONUS_COLOR,
                ));
            });
        });
}

/// A caption and its value, side by side
fn group_node() -> Node {
    Node {
        align_items: AlignItems::Center,
        column_gap: Val::Px(GROUP_GAP),
        ..Default::default()
    }
}

fn label(assets: &GameAssets, caption: &str) -> impl Bundle {
    (
        Text::new(caption),
        text_font(&assets.font, LABEL_FONT_SIZE),
        TextColor(LABEL_COLOR),
    )
}

fn scoreboard_text(
    field: ScoreboardText,
    font: &Handle<Font>,
    font_size: f32,
    color: Color,
) -> impl Bundle {
    (
        field,
        Text::new(""),
        text_font(font, font_size),
        TextColor(color),
    )
}

fn update_scoreboard_system(
    score: Res<Score>,
    difficulty: Res<Difficulty>,
    player: Query<(&Health, &BonusSlot), With<Player>>,
    // The player is despawned when it dies: remember how many hearts to show empty
    mut max_health: Local<u32>,
    mut texts: Query<(&ScoreboardText, &mut Text, &mut TextColor)>,
    progress_fill: Single<(&mut Node, &mut BackgroundColor), With<ProgressFill>>,
    mut bonus_badge: Single<&mut Visibility, With<BonusBadge>>,
) {
    let (health, bonus_status) = match player.single() {
        Ok((health, bonus_slot)) => {
            *max_health = health.max();
            (health.current(), bonus_slot.status())
        }
        Err(_) => (0, None),
    };
    let view = ScoreboardView::new(
        health,
        *max_health,
        &difficulty,
        score.value(),
        bonus_status,
    );

    for (field, mut text, mut color) in texts.iter_mut() {
        let new_text = view.text(*field);
        // Only write changes, so the text is not laid out again every frame
        if text.0 != new_text {
            text.0 = new_text.to_string();
        }
        if *field == ScoreboardText::DifficultyName {
            color.set_if_neq(TextColor(view.difficulty_color));
        }
    }

    let (mut fill_node, mut fill_color) = progress_fill.into_inner();
    let fill_width = Val::Percent(view.progress_percent);
    if fill_node.width != fill_width {
        fill_node.width = fill_width;
    }
    fill_color.set_if_neq(BackgroundColor(view.difficulty_color));

    bonus_badge.set_if_neq(if view.bonus.is_some() {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    });
}

/// Everything the scoreboard shows
#[derive(Debug, Clone, PartialEq)]
struct ScoreboardView {
    full_hearts: String,
    empty_hearts: String,
    difficulty_name: &'static str,
    difficulty_color: Color,
    difficulty_percent: String,
    /// Width of the filled part of the progress bar, in percent
    progress_percent: f32,
    score: String,
    /// `None` when no bonus is active
    bonus: Option<String>,
}

impl ScoreboardView {
    fn new(
        health: u32,
        max_health: u32,
        difficulty: &Difficulty,
        score: u32,
        bonus_status: Option<(&str, u32)>,
    ) -> Self {
        let (full_hearts, empty_hearts) = hearts(health, max_health);
        let percent_until_next_level = difficulty.percent_until_next_level();

        ScoreboardView {
            full_hearts,
            empty_hearts,
            difficulty_name: difficulty.name(),
            difficulty_color: level_color(difficulty.level()),
            difficulty_percent: difficulty_percent_text(percent_until_next_level),
            // Full once the maximum level is reached
            progress_percent: percent_until_next_level.unwrap_or(100) as f32,
            score: format_score(score),
            bonus: bonus_status.map(bonus_text),
        }
    }

    fn text(&self, field: ScoreboardText) -> &str {
        match field {
            ScoreboardText::FullHearts => &self.full_hearts,
            ScoreboardText::EmptyHearts => &self.empty_hearts,
            ScoreboardText::DifficultyName => self.difficulty_name,
            ScoreboardText::DifficultyPercent => &self.difficulty_percent,
            ScoreboardText::Score => &self.score,
            ScoreboardText::Bonus => self.bonus.as_deref().unwrap_or(""),
        }
    }
}

/// One full heart per health point left, then one empty heart per health point lost
fn hearts(health: u32, max_health: u32) -> (String, String) {
    let full_count = health.min(max_health) as usize;
    let empty_count = max_health.saturating_sub(health) as usize;

    (
        std::iter::repeat_n(FULL_HEART, full_count).collect(),
        std::iter::repeat_n(EMPTY_HEART, empty_count).collect(),
    )
}

fn level_color(level: u32) -> Color {
    LEVEL_COLORS[(level as usize).min(LEVEL_COLORS.len() - 1)]
}

/// # Arguments
///
/// * `percent_until_next_level` - `None` at the maximum level
fn difficulty_percent_text(percent_until_next_level: Option<u32>) -> String {
    format!(
        "{}%",
        percent_until_next_level.unwrap_or(MAX_DIFFICULTY_PERCENT)
    )
}

/// # Arguments
///
/// * `bonus_status` - The name of the active bonus and its remaining seconds
fn bonus_text((bonus_name, remaining_seconds): (&str, u32)) -> String {
    format!("{}  {}s", bonus_name, remaining_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hearts_at_full_health_test() {
        assert_eq!(hearts(5, 5), ("♥♥♥♥♥".to_string(), String::new()));
    }

    #[test]
    fn hearts_after_damage_test() {
        assert_eq!(hearts(3, 5), ("♥♥♥".to_string(), "♡♡".to_string()));
    }

    #[test]
    fn hearts_when_dead_test() {
        assert_eq!(hearts(0, 5), (String::new(), "♡♡♡♡♡".to_string()));
    }

    #[test]
    fn level_color_per_level_test() {
        assert_eq!(level_color(0), LEVEL_COLORS[0]);
        assert_eq!(level_color(3), LEVEL_COLORS[3]);
    }

    #[test]
    fn level_color_past_the_last_level_test() {
        assert_eq!(level_color(99), LEVEL_COLORS[LEVEL_COLORS.len() - 1]);
    }

    #[test]
    fn difficulty_percent_text_test() {
        assert_eq!(difficulty_percent_text(Some(43)), "43%");
    }

    #[test]
    fn difficulty_percent_text_at_max_level_test() {
        assert_eq!(difficulty_percent_text(None), "666%");
    }

    #[test]
    fn bonus_text_test() {
        assert_eq!(bonus_text(("MACHINE GUN", 7)), "MACHINE GUN  7s");
    }

    #[test]
    fn scoreboard_view_at_game_start_test() {
        let view = ScoreboardView::new(5, 5, &Difficulty::default(), 5050, None);

        assert_eq!(view.text(ScoreboardText::FullHearts), "♥♥♥♥♥");
        assert_eq!(view.text(ScoreboardText::EmptyHearts), "");
        assert_eq!(view.text(ScoreboardText::DifficultyName), "EASY");
        assert_eq!(view.text(ScoreboardText::DifficultyPercent), "0%");
        assert_eq!(view.text(ScoreboardText::Score), "0005050");
        assert_eq!(view.text(ScoreboardText::Bonus), "");
        assert_eq!(view.difficulty_color, LEVEL_COLORS[0]);
        assert_eq!(view.progress_percent, 0.0);
        assert_eq!(view.bonus, None);
    }

    #[test]
    fn scoreboard_view_with_active_bonus_test() {
        let view = ScoreboardView::new(
            2,
            5,
            &Difficulty::default(),
            0,
            Some(("MACHINE GUN", 7)),
        );

        assert_eq!(view.text(ScoreboardText::FullHearts), "♥♥");
        assert_eq!(view.text(ScoreboardText::EmptyHearts), "♡♡♡");
        assert_eq!(view.text(ScoreboardText::Bonus), "MACHINE GUN  7s");
    }
}
