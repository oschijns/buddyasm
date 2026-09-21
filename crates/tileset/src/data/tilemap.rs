use crate::data::flip::Flip;
use ndarray::{Array2, Axis};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

/// Indexes map to reconstruct the pictural data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileMap(pub Rc<Array2<TileData>>);

impl TileMap {
    /// Create a new index map
    #[inline]
    pub fn new(data: Array2<TileData>) -> Self {
        Self(Rc::new(data))
    }

    /// Flip the tile map horizontally
    pub fn flip_h(&self) -> Self {
        // Clone the matrix, flip it along the X-axis
        let mut out_map = self.0.as_ref().clone();
        out_map.invert_axis(Axis(0));

        // Adjust the flipping attribute of each tile
        for tile in out_map.iter_mut() {
            tile.flip = tile.flip.flip_h();
        }
        Self(Rc::new(out_map))
    }

    /// Flip the tile map vertically
    pub fn flip_v(&self) -> Self {
        // Clone the matrix, flip it along the X-axis
        let mut out_map = self.0.as_ref().clone();
        out_map.invert_axis(Axis(1));

        // Adjust the flipping attribute of each tile
        for tile in out_map.iter_mut() {
            tile.flip = tile.flip.flip_v();
        }
        Self(Rc::new(out_map))
    }

    /// Flip the tile map both horizontally and vertically
    pub fn flip_both(&self) -> Self {
        // Clone the matrix, flip it along the X-axis
        let mut out_map = self.0.as_ref().clone();
        out_map.invert_axis(Axis(0));
        out_map.invert_axis(Axis(1));

        // Adjust the flipping attribute of each tile
        for tile in out_map.iter_mut() {
            tile.flip = tile.flip.flip_both();
        }
        Self(Rc::new(out_map))
    }
}

/// Indexes to reconstruct the pictural data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileData {
    /// Index of the tile
    pub tile_index: u16,

    /// Index of the palette
    pub palette_index: u16,

    /// Flip horizontally and/or vertically
    pub flip: Flip,
}

impl TileData {
    /// Create a tile data
    #[inline]
    pub fn new(tile_index: usize, palette_index: usize, flip: Flip) -> Self {
        Self {
            tile_index: tile_index as u16,
            palette_index: palette_index as u16,
            flip,
        }
    }
}

macro_rules! impl_flip {
    ( $flip:ident ) => {
        impl TileData {
            #[inline]
            pub fn $flip(self) -> Self {
                Self {
                    tile_index: self.tile_index,
                    palette_index: self.palette_index,
                    flip: self.flip.$flip(),
                }
            }
        }
    };
}

impl_flip!(flip_h);
impl_flip!(flip_v);
impl_flip!(flip_both);

/*
 * TODO:
 * The SEGA Master System in the only hardware which does not supports sprite
 * flipping. Eventtually, we could pass a reference to find matching flipped
 * sprite and apply their index. But this is not a priority right now.
 */
