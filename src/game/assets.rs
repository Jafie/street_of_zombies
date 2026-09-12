//! Handles to the files under `assets/`, loaded once for the whole run.

use bevy::prelude::*;

const BACKGROUND_PATH: &str = "images/background_street_of_zombies.png";
const FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";
const SYMBOL_FONT_PATH: &str = "fonts/DejaVuSans-Bold.ttf";
const PLAYER_SPRITE_SHEET_PATH: &str = "sprites/woman.png";
const ZOMBIE_SPRITE_SHEET_PATH: &str = "sprites/zombie.png";

/// Size of one frame of a character sprite sheet, in pixels
const CHARACTER_FRAME_SIZE: UVec2 = UVec2::new(77, 77);
/// Number of animation frames per row of a character sprite sheet
pub const CHARACTER_SHEET_COLUMNS: u32 = 8;
/// Number of rows of a character sprite sheet: one per direction
const CHARACTER_SHEET_ROWS: u32 = 4;
/// Frame shown until the character first moves
const CHARACTER_INITIAL_FRAME: usize = 1;

pub fn plugin(app: &mut App) {
    // Loaded while the app is built rather than by a startup system: the first
    // `OnEnter(GameState::Playing)` runs before `PreStartup` and already spawns the player.
    app.init_resource::<GameAssets>();
}

/// A character drawn from a sprite sheet
#[derive(Clone, Copy, Debug)]
pub enum CharacterSprite {
    Player,
    Zombie,
}

/// Every asset of the game. Holding the handles keeps the files loaded, so a sprite never
/// appears before its texture.
#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub font: Handle<Font>,
    /// Has the symbols `font` lacks, such as the hearts of the scoreboard
    pub symbol_font: Handle<Font>,
    player_sprite_sheet: Handle<Image>,
    zombie_sprite_sheet: Handle<Image>,
    /// Shared by every character: all sprite sheets use the same grid
    character_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let layout = TextureAtlasLayout::from_grid(
            CHARACTER_FRAME_SIZE,
            CHARACTER_SHEET_COLUMNS,
            CHARACTER_SHEET_ROWS,
            None,
            None,
        );
        let character_layout = world
            .resource_mut::<Assets<TextureAtlasLayout>>()
            .add(layout);

        let asset_server = world.resource::<AssetServer>();
        GameAssets {
            background: asset_server.load(BACKGROUND_PATH),
            font: asset_server.load(FONT_PATH),
            symbol_font: asset_server.load(SYMBOL_FONT_PATH),
            player_sprite_sheet: asset_server.load(PLAYER_SPRITE_SHEET_PATH),
            zombie_sprite_sheet: asset_server.load(ZOMBIE_SPRITE_SHEET_PATH),
            character_layout,
        }
    }
}

impl GameAssets {
    /// Create the animated sprite of a character, showing its initial frame
    pub fn character_sprite(&self, character: CharacterSprite) -> Sprite {
        let sprite_sheet = match character {
            CharacterSprite::Player => &self.player_sprite_sheet,
            CharacterSprite::Zombie => &self.zombie_sprite_sheet,
        };

        Sprite::from_atlas_image(
            sprite_sheet.clone(),
            TextureAtlas {
                layout: self.character_layout.clone(),
                index: CHARACTER_INITIAL_FRAME,
            },
        )
    }
}
