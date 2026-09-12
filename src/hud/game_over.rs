//! The panel shown over the play area once the player is dead.

use bevy::prelude::*;

use super::{format_score, text_font, ACCENT_COLOR, SCORE_COLOR};
use crate::game::{GameAssets, GameSet, GameState};
use crate::progression::Score;

const TITLE_FONT_SIZE: f32 = 72.0;
const FINAL_SCORE_FONT_SIZE: f32 = 32.0;
const HINT_FONT_SIZE: f32 = 24.0;
const TITLE_COLOR: Color = Color::srgb(0.9, 0.1, 0.12);
const HINT_COLOR: Color = Color::WHITE;
const PANEL_BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);
const TITLE_SHADOW: TextShadow = TextShadow {
    offset: Vec2::splat(4.0),
    color: Color::BLACK,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_game_over_panel_system)
        .add_systems(OnEnter(GameState::Playing), hide_game_over_panel_system)
        .add_systems(OnEnter(GameState::GameOver), show_game_over_panel_system)
        .add_systems(
            Update,
            update_final_score_system
                .in_set(GameSet::Presentation)
                .run_if(in_state(GameState::GameOver)),
        );
}

/// Marks the root of the game over panel
#[derive(Component)]
struct GameOverPanel;

/// Marks the text showing the score of the game that just ended
#[derive(Component)]
struct FinalScoreText;

fn spawn_game_over_panel_system(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            GameOverPanel,
            // Covers the window only to center the panel
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // Hidden from the start: the first `OnEnter(Playing)` runs before this is spawned
            Visibility::Hidden,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(12.0),
                    padding: UiRect::axes(Val::Px(48.0), Val::Px(24.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..Default::default()
                },
                BackgroundColor(PANEL_BACKGROUND_COLOR),
                BorderColor::all(ACCENT_COLOR),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("GAME OVER"),
                    text_font(&assets.font, TITLE_FONT_SIZE),
                    TextColor(TITLE_COLOR),
                    TITLE_SHADOW,
                ));
                panel.spawn((
                    FinalScoreText,
                    Text::new(""),
                    text_font(&assets.font, FINAL_SCORE_FONT_SIZE),
                    TextColor(SCORE_COLOR),
                ));
                panel.spawn((
                    Text::new("PRESS R TO RESTART"),
                    text_font(&assets.font, HINT_FONT_SIZE),
                    TextColor(HINT_COLOR),
                ));
            });
        });
}

fn show_game_over_panel_system(mut panel: Query<&mut Visibility, With<GameOverPanel>>) {
    for mut visibility in panel.iter_mut() {
        *visibility = Visibility::Inherited;
    }
}

fn hide_game_over_panel_system(mut panel: Query<&mut Visibility, With<GameOverPanel>>) {
    for mut visibility in panel.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

/// Projectiles still flying after the player's death can score, so the text keeps up
fn update_final_score_system(
    score: Res<Score>,
    mut final_score: Single<&mut Text, With<FinalScoreText>>,
) {
    let new_text = final_score_text(score.value());
    // Only write changes, so the text is not laid out again every frame
    if final_score.0 != new_text {
        final_score.0 = new_text;
    }
}

fn final_score_text(score: u32) -> String {
    format!("FINAL SCORE  {}", format_score(score))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_score_text_test() {
        assert_eq!(final_score_text(5050), "FINAL SCORE  0005050");
    }
}
