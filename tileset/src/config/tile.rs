//! Define sprites configuration

use crate::data::coords::TileSize;
use serde::Deserialize;
use std::str::FromStr;
use strum::EnumString;

/// Get the size of the sprite from the config selected
pub trait TileConfig {
    #[inline]
    fn flipping(&self) -> [bool; 2] {
        [true, true]
    }

    #[inline]
    fn tile_size(&self) -> TileSize {
        TileSize::new(8, 8)
    }
}

/// Non-configurable tile size and mode.
/// Used for hardware which only supports one type of tile.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct FixedTile<const FLIP: bool = true, const SIZE: u32 = 8>;

/// Implement trait to meet required trait bounds
impl<const FLIP: bool, const SIZE: u32> FromStr for FixedTile<FLIP, SIZE> {
    type Err = ();

    #[inline]
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self)
    }
}

/// Sprite modes supported by the Famicom / GameBoy
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, EnumString)]
pub enum SpriteNintendo1 {
    /// Small 8x8 sprite
    #[default]
    #[serde(rename = "8x8")]
    #[strum(serialize = "8x8")]
    S8x8,

    /// Larger 8x16 sprite
    #[serde(rename = "8x16")]
    #[strum(serialize = "8x16")]
    S8x16,
}

/// Sprite modes supported by the Super Famicom / Virtual Boy
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, EnumString)]
pub enum SpriteNintendo2 {
    /// 8x8 sprite
    #[default]
    #[serde(rename = "8x8")]
    #[strum(serialize = "8x8")]
    S8x8,

    /// 16x16 sprite
    #[serde(rename = "16x16")]
    #[strum(serialize = "16x16")]
    S16x16,

    /// 32x32 sprite
    #[serde(rename = "32x32")]
    #[strum(serialize = "32x32")]
    S32x32,

    /// 64x64 sprite
    #[serde(rename = "64x64")]
    #[strum(serialize = "64x64")]
    S64x64,
}

/// Sprite modes supported by the PC-Engine
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, EnumString)]
pub enum SpritePcEngine {
    /// 16x16 sprite
    #[default]
    #[serde(rename = "16x16")]
    #[strum(serialize = "16x16")]
    S16x16,

    /// 16x32 sprite
    #[serde(rename = "16x32")]
    #[strum(serialize = "16x32")]
    S16x32,

    /// 16x48 sprite
    #[serde(rename = "16x48")]
    #[strum(serialize = "16x48")]
    S16x48,

    /// 16x64 sprite
    #[serde(rename = "16x64")]
    #[strum(serialize = "16x64")]
    S16x64,

    /// 32x16 sprite
    #[serde(rename = "32x16")]
    #[strum(serialize = "32x16")]
    S32x16,

    /// 32x32 sprite
    #[serde(rename = "32x32")]
    #[strum(serialize = "32x32")]
    S32x32,

    /// 32x48 sprite
    #[serde(rename = "32x48")]
    #[strum(serialize = "32x48")]
    S32x48,

    /// 32x64 sprite
    #[serde(rename = "32x64")]
    #[strum(serialize = "32x64")]
    S32x64,
}

/// Sprite modes supported by the SEGA Master System
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, EnumString)]
pub enum SpriteSegaMS {
    /// Small 8x8 sprite
    #[default]
    #[serde(rename = "8x8")]
    #[strum(serialize = "8x8")]
    S8x8,

    /// Larger 8x16 sprite
    #[serde(rename = "8x16")]
    #[strum(serialize = "8x16")]
    S8x16,
}

/// Sprite modes supported by the SEGA MegaDrive
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, EnumString)]
pub enum SpriteSegaMD {
    /// 8x8 sprite
    #[default]
    #[serde(rename = "8x8")]
    #[strum(serialize = "8x8")]
    S8x8,

    /// 16x16 sprite
    #[serde(rename = "16x16")]
    #[strum(serialize = "16x16")]
    S16x16,

    /// 24x24 sprite
    #[serde(rename = "24x24")]
    #[strum(serialize = "24x24")]
    S24x24,

    /// 32x32 sprite
    #[serde(rename = "32x32")]
    #[strum(serialize = "32x32")]
    S32x32,
}

/// Sprite modes supported by the NeoGeo Pocket
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, EnumString)]
pub enum SpriteNeoGeoPocket {
    /// 8x8 sprite
    #[default]
    #[serde(rename = "8x8")]
    #[strum(serialize = "8x8")]
    S8x8,

    /// 8x16 sprite
    #[serde(rename = "8x16")]
    #[strum(serialize = "8x16")]
    S8x16,

    /// 8x32 sprite
    #[serde(rename = "8x32")]
    #[strum(serialize = "8x32")]
    S8x32,

    /// 16x8 sprite
    #[serde(rename = "16x8")]
    #[strum(serialize = "16x8")]
    S16x8,

    /// 16x16 sprite
    #[serde(rename = "16x16")]
    #[strum(serialize = "16x16")]
    S16x16,

    /// 16x32 sprite
    #[serde(rename = "16x32")]
    #[strum(serialize = "16x32")]
    S16x32,

    /// 32x8 sprite
    #[serde(rename = "32x8")]
    #[strum(serialize = "32x8")]
    S32x8,

    /// 32x16 sprite
    #[serde(rename = "32x16")]
    #[strum(serialize = "32x16")]
    S32x16,

    /// 32x32 sprite
    #[serde(rename = "32x32")]
    #[strum(serialize = "32x32")]
    S32x32,
}

/// Error encountered when we could not identify a sprite format based on provided string
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Could not identify sprite size from \"{0}\"")]
pub struct ParseError(pub(crate) String);

/// Specify if we are processing tile or sprite data
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum TileOrSprite<T, S>
where
    T: Clone + PartialEq,
    S: Clone + PartialEq,
{
    /// Tile data
    Tile(T),

    /// Sprite data
    Sprite(S),
}

impl<T, S> TileOrSprite<T, S>
where
    T: Clone + PartialEq + Default,
    S: Clone + PartialEq,
{
    /// We are making tiles
    #[inline]
    pub fn default_tile() -> Self {
        Self::Tile(T::default())
    }
}

impl<'s, T, S> TileOrSprite<T, S>
where
    T: Clone + PartialEq,
    S: Clone + PartialEq + Default + FromStr,
{
    /// We are making sprites
    #[inline]
    pub fn default_sprite() -> Self {
        Self::Sprite(S::default())
    }

    /// We are making sprites
    #[inline]
    pub fn sprite(sprite_size: Option<&'s str>) -> Result<Self, ParseError> {
        if let Some(size) = sprite_size {
            if let Ok(s) = S::from_str(size) {
                Ok(Self::Sprite(s))
            } else {
                Err(ParseError(size.to_string()))
            }
        } else {
            Ok(Self::Sprite(S::default()))
        }
    }
}

impl<const FLIP: bool, const SIZE: u32> TileConfig for FixedTile<FLIP, SIZE> {
    #[inline]
    fn flipping(&self) -> [bool; 2] {
        [FLIP, FLIP]
    }

    #[inline]
    fn tile_size(&self) -> TileSize {
        TileSize::new(SIZE, SIZE)
    }
}

impl TileConfig for SpriteNintendo1 {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8  => TileSize::new(8,  8),
            Self::S8x16 => TileSize::new(8, 16),
        }
    }
}

impl TileConfig for SpriteNintendo2 {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8   => TileSize::new( 8,  8),
            Self::S16x16 => TileSize::new(16, 16),
            Self::S32x32 => TileSize::new(32, 32),
            Self::S64x64 => TileSize::new(64, 64),
        }
    }
}

impl TileConfig for SpritePcEngine {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S16x16 => TileSize::new(16, 16),
            Self::S16x32 => TileSize::new(16, 32),
            Self::S16x48 => TileSize::new(16, 48),
            Self::S16x64 => TileSize::new(16, 64),
            Self::S32x16 => TileSize::new(32, 16),
            Self::S32x32 => TileSize::new(32, 32),
            Self::S32x48 => TileSize::new(32, 48),
            Self::S32x64 => TileSize::new(32, 64),
        }
    }
}

impl TileConfig for SpriteSegaMS {
    /// The Master System supported tile flipping
    /// but not sprite flipping for some reason...
    #[inline]
    fn flipping(&self) -> [bool; 2] {
        [false, false]
    }

    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8  => TileSize::new(8,  8),
            Self::S8x16 => TileSize::new(8, 16),
        }
    }
}

impl TileConfig for SpriteSegaMD {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8   => TileSize::new( 8,  8),
            Self::S16x16 => TileSize::new(16, 16),
            Self::S24x24 => TileSize::new(24, 24),
            Self::S32x32 => TileSize::new(32, 32),
        }
    }
}

impl TileConfig for SpriteNeoGeoPocket {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8   => TileSize::new( 8,  8),
            Self::S8x16  => TileSize::new( 8, 16),
            Self::S8x32  => TileSize::new( 8, 16),
            Self::S16x8  => TileSize::new(16,  8),
            Self::S16x16 => TileSize::new(16, 16),
            Self::S16x32 => TileSize::new(16, 32),
            Self::S32x8  => TileSize::new(32,  8),
            Self::S32x16 => TileSize::new(32, 16),
            Self::S32x32 => TileSize::new(32, 32),
        }
    }
}

impl<T, S> Default for TileOrSprite<T, S>
where
    T: Clone + PartialEq + Default,
    S: Clone + PartialEq,
{
    fn default() -> Self {
        Self::Tile(Default::default())
    }
}

impl<T, S> TileConfig for TileOrSprite<T, S>
where
    T: TileConfig + Clone + PartialEq,
    S: TileConfig + Clone + PartialEq,
{
    fn flipping(&self) -> [bool; 2] {
        match self {
            Self::Tile(tile) => tile.flipping(),
            Self::Sprite(sprite) => sprite.flipping(),
        }
    }

    fn tile_size(&self) -> TileSize {
        match self {
            Self::Tile(tile) => tile.tile_size(),
            Self::Sprite(sprite) => sprite.tile_size(),
        }
    }
}
