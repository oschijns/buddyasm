/// Basic operations on the existing types
mod base;

/// Load an input imgae to convert it into a tileset
mod loader;

/// Serialize the tileset
mod serial;

use ndarray::{Array2, Ix, Ix2};
use std::rc::Rc;

/// Define a pixel in a tile
pub type Pix = u8;

/// Tile to serialize in a given binary layout
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tile(Rc<Array2<Pix>>);

/// Tileset containing tiles
#[derive(Debug, Clone)]
pub struct TileSet(Rc<Vec<Tile>>);

/// Set of palettes to look for in an input image
#[derive(Debug, Clone)]
pub struct PaletteSet<C>(Rc<Array2<C>>);

/// Flipping axes
#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flip {
    /// No flip
    #[default]
    None = 0b00,

    /// Horizontal flip
    Horizontal = 0b01,

    /// Vertical flip
    Vertical = 0b10,

    /// Flip horizontally and vertically
    Both = 0b11,
}

/// Indexes to reconstruct the pictural data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IndexTile {
    /// Index of the tile
    tile: usize,

    /// Index of the palette
    palette: usize,

    /// Flip horizontally and/or vertically
    flip: Flip,
}

/// Indexes map to reconstruct the pictural data
#[derive(Debug, Clone)]
pub struct IndexMap(Rc<Array2<IndexTile>>);

/// Reconstruction of the input image as a tileset, a palette and an indexed map.
#[derive(Debug, Clone)]
pub struct TileMap<C> {
    /// TileSet used
    tile_set: TileSet,

    /// PaletteSet used
    palette_set: PaletteSet<C>,

    /// Define how the reconstruct the image
    indexed_map: IndexMap,
}

impl Tile {
    /// Create a new tile from data
    #[inline]
    pub fn new(data: Array2<Pix>) -> Self {
        Self(Rc::new(data))
    }

    /// Get the number of pixels in this tile
    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.0.as_ref().len()
    }
}

impl TileSet {
    /// Create a new tileset from raw data
    #[inline]
    pub fn new(data: Vec<Tile>) -> Self {
        Self(Rc::new(data))
    }

    /// Get the number of tiles in this tileset
    #[inline]
    pub fn count(&self) -> usize {
        self.0.as_ref().len()
    }

    /// Get the total number of pixels in this tileset.
    /// This assumes that all tiles stored have the same size.
    #[inline]
    pub fn pixel_count(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.count() * self.0.as_ref()[0].pixel_count()
        }
    }
}

impl<C> PaletteSet<C> {
    /// Create a new palette set
    #[inline]
    pub fn new(data: Array2<C>) -> Self {
        Self(Rc::new(data))
    }
}

impl IndexTile {
    /// Create index data for a tile
    #[inline]
    pub fn new(tile: usize, palette: usize, flip: Flip) -> Self {
        Self {
            tile,
            palette,
            flip,
        }
    }
}

impl IndexMap {
    /// Create a new index map
    #[inline]
    pub fn new(data: Array2<IndexTile>) -> Self {
        Self(Rc::new(data))
    }
}

/// Convert image coordinates into ndarray coordinates
#[inline]
fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}
