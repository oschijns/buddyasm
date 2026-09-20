use crate::data::{coords::Coords, flip::Flip};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

/// Indexes map to reconstruct the pictural data
#[derive(Debug, Clone)]
pub struct TileMap(pub Rc<Array2<TileData>>);

impl TileMap {
    /// Create a new index map
    #[inline]
    pub fn new(data: Array2<TileData>) -> Self {
        Self(Rc::new(data))
    }
}

/// Indexes to reconstruct the pictural data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileData {
    /// Index of the tile
    pub tile_index: u16,

    /// Index of the palette
    pub palette_index: u16,

    /// Position of the tile (in tiles)
    pub position_x: u16,

    /// Position of the tile (in tiles)
    pub position_y: u16,

    /// Flip horizontally and/or vertically
    pub flip: Flip,

    /// Indicate if the tile is actually empty
    pub empty: bool,
}

impl TileData {
    /// Create a tile data
    #[inline]
    pub fn new(
        tile_index: usize,
        palette_index: usize,
        coords: Coords,
        flip: Flip,
        empty: bool,
    ) -> Self {
        let [position_x, position_y] = coords;
        Self {
            tile_index: tile_index as u16,
            palette_index: palette_index as u16,
            position_x,
            position_y,
            flip,
            empty,
        }
    }
}
