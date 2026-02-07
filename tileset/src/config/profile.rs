//! Define hardware profiles

use serde::Deserialize;

/// Get the tile size for the given hardware profile
pub trait GetTileSize {
    #[inline]
    fn tile_size(&self) -> TileSize {
        [8, 8]
    }
}

/// Size of a tile
pub type TileSize = [u32; 2];

/// Famicom profile
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ProfileFamicom {
    /// Regular 8x8 tile
    #[default]
    Tile8x8,

    /// Larger 8x16 tile
    Tile8x16,
}

/// The famicom supports two sprites sizes
impl GetTileSize for ProfileFamicom {
    #[inline]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::Tile8x8 => [8, 8],
            Self::Tile8x16 => [8, 16],
        }
    }
}
