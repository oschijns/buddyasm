//! Define sprites configuration

use crate::data::coords::TileSize;
use serde::Deserialize;

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

/// Most background tile are limited to 8x8 with possibly X & Y flipping
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct BgTile<const FLIP: bool = false>;

/// Sprite modes supported by the Famicom / GameBoy
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SpriteNintendo1 {
    /// Small 8x8 sprite
    #[default]
    S8x8,

    /// Larger 8x16 sprite
    S8x16,
}

/// Sprite modes supported by the Super Famicom / Virtual Boy
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SpriteNintendo2 {
    /// 8x8 sprite
    #[default]
    S8x8,

    /// 16x16 sprite
    S16x16,

    /// 32x32 sprite
    S32x32,

    /// 64x64 sprite
    S64x64,
}

/// Sprite modes supported by the PC-Engine
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SpritePcEngine {
    /// 16x16 sprite
    #[default]
    S16x16,

    /// 16x32 sprite
    S16x32,

    /// 16x48 sprite
    S16x48,

    /// 16x64 sprite
    S16x64,

    /// 32x16 sprite
    S32x16,

    /// 32x32 sprite
    S32x32,

    /// 32x48 sprite
    S32x48,

    /// 32x64 sprite
    S32x64,
}

/// Sprite modes supported by the NeoGeo Pocket
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SpriteNeoGeoPocket {
    /// 8x8 sprite
    #[default]
    S8x8,

    /// 8x16 sprite
    S8x16,

    /// 8x32 sprite
    S8x32,

    /// 16x8 sprite
    S16x8,

    /// 16x16 sprite
    S16x16,

    /// 16x32 sprite
    S16x32,

    /// 32x8 sprite
    S32x8,

    /// 32x16 sprite
    S32x16,

    /// 32x64 sprite
    S32x64,
}

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

impl<const FLIP: bool> TileConfig for BgTile<FLIP> {
    #[inline]
    fn flipping(&self) -> [bool; 2] {
        [FLIP, FLIP]
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
            Self::S32x64 => TileSize::new(32, 32),
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
