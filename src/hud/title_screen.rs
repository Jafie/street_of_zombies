//! The front page shown when the game is launched, until space is pressed.

use bevy::prelude::*;

use super::{text_font, ACCENT_COLOR, SCORE_COLOR, TITLE_COLOR, TITLE_SHADOW};
use crate::game::{GameAssets, GameSet, GameState};

const TITLE_FONT_SIZE: f32 = 80.0;
const TAGLINE_FONT_SIZE: f32 = 24.0;
const CONTROL_FONT_SIZE: f32 = 22.0;
const PROMPT_FONT_SIZE: f32 = 32.0;
/// Width of the key column of the controls, so the actions line up
const CONTROL_KEY_WIDTH: f32 = 160.0;

/// Darkens the street behind the front page
const OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
const PANEL_BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);
const TAGLINE_COLOR: Color = Color::srgb(0.6, 0.6, 0.66);
const CONTROL_ACTION_COLOR: Color = Color::WHITE;
const PROMPT_COLOR: Color = Color::WHITE;

/// The start prompt blinks: it is shown during the first part of every period
const PROMPT_BLINK_PERIOD_SECS: f32 = 1.0;
const PROMPT_SHOWN_SECS: f32 = 0.65;

/// Each key and what it does
const CONTROLS: [(&str, &str); 3] = [
    ("ARROW KEYS", "MOVE"),
    ("SPACE", "FIRE"),
    ("R", "RESTART AFTER GAME OVER"),
];

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_title_screen_system).add_systems(
        Update,
        blink_start_prompt_system
            .in_set(GameSet::Presentation)
            .run_if(in_state(GameState::Title)),
    );
}

/// Marks the "press space" text
#[derive(Component)]
struct StartPrompt;

fn spawn_title_screen_system(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(32.0),
                ..Default::default()
            },
            BackgroundColor(OVERLAY_COLOR),
            // Covers the scoreboard too
            GlobalZIndex(1),
            // Shown only once: R restarts straight into a new game
            DespawnOnExit(GameState::Title),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("STREET OF ZOMBIES"),
                text_font(&assets.font, TITLE_FONT_SIZE),
                TextColor(TITLE_COLOR),
                TITLE_SHADOW,
            ));
            root.spawn((
                Text::new("SURVIVE THE HORDE AS LONG AS YOU CAN"),
                text_font(&assets.font, TAGLINE_FONT_SIZE),
                TextColor(TAGLINE_COLOR),
            ));

            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    padding: UiRect::axes(Val::Px(40.0), Val::Px(20.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..Default::default()
                },
                BackgroundColor(PANEL_BACKGROUND_COLOR),
                BorderColor::all(ACCENT_COLOR),
            ))
            .with_children(|panel| {
                for (key, action) in CONTROLS {
                    panel.spawn(Node::default()).with_children(|row| {
                        row.spawn((
                            Text::new(key),
                            text_font(&assets.font, CONTROL_FONT_SIZE),
                            TextColor(SCORE_COLOR),
                            Node {
                                min_width: Val::Px(CONTROL_KEY_WIDTH),
                                ..Default::default()
                            },
                        ));
                        row.spawn((
                            Text::new(action),
                            text_font(&assets.font, CONTROL_FONT_SIZE),
                            TextColor(CONTROL_ACTION_COLOR),
                        ));
                    });
                }
            });

            root.spawn((
                StartPrompt,
                Text::new("PRESS SPACE TO START"),
                text_font(&assets.font, PROMPT_FONT_SIZE),
                TextColor(PROMPT_COLOR),
            ));
        });
}

fn blink_start_prompt_system(
    time: Res<Time>,
    mut prompt: Single<&mut Visibility, With<StartPrompt>>,
) {
    // Hidden keeps its place in the layout, so the page does not move while blinking
    prompt.set_if_neq(if is_start_prompt_shown(time.elapsed_secs()) {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    });
}

/// # Arguments
///
/// * `elapsed_secs` - Time since the game was launched
fn is_start_prompt_shown(elapsed_secs: f32) -> bool {
    elapsed_secs % PROMPT_BLINK_PERIOD_SECS < PROMPT_SHOWN_SECS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_prompt_shown_at_launch_test() {
        assert!(is_start_prompt_shown(0.0));
    }

    #[test]
    fn start_prompt_hidden_at_end_of_period_test() {
        assert!(!is_start_prompt_shown(0.8));
    }

    #[test]
    fn start_prompt_blinks_every_period_test() {
        assert!(is_start_prompt_shown(1.2));
        assert!(!is_start_prompt_shown(1.9));
    }
}
