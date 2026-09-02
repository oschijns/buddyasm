use crate::data::flip::Flip;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

/// Indexes map to reconstruct the pictural data
#[derive(Debug, Clone)]
pub struct TileMap(pub Rc<Array2<TileData>>);

/// Indexes to reconstruct the pictural data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileData {
    /// Index of the tile
    pub tile_index: usize,

    /// Index of the palette
    pub palette_index: usize,

    /// Flip horizontally and/or vertically
    pub flip: Flip,
}

impl TileMap {
    /// Create a new index map
    #[inline]
    pub fn new(data: Array2<TileData>) -> Self {
        Self(Rc::new(data))
    }
}
