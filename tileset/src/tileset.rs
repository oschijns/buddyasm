/// Basic operations on the existing types
mod base;

/// Load an input imgae to convert it into a tileset
pub mod builder;

/// Load data from files
mod load;

/// Serialize the tileset
mod serial;

use image::{Luma, LumaA, Rgb, Rgba};
use ndarray::{Array2, Ix, Ix2};
use std::{collections::HashMap, rc::Rc};

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

/// RGBA palette set
pub type PaletteSetRgba = PaletteSet<Rgba<u8>>;

/// RGB palette set
pub type PaletteSetRgb = PaletteSet<Rgb<u8>>;

/// Grayscale + Alpha palette set
pub type PaletteSetLumaA = PaletteSet<LumaA<u8>>;

/// Grayscale palette set
pub type PaletteSetLuma = PaletteSet<Luma<u8>>;

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

/// Enforce a fixed mapping between a tile position and a target index
#[derive(Default, Debug, Clone)]
pub struct Mapping(Rc<HashMap<[u32; 2], usize>>);

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

/// RGBA palette set
pub type TileMapRgba = TileMap<Rgba<u8>>;

/// RGB palette set
pub type TileMapRgb = TileMap<Rgb<u8>>;

/// Grayscale + Alpha palette set
pub type TileMapLumaA = TileMap<LumaA<u8>>;

/// Grayscale palette set
pub type TileMapLuma = TileMap<Luma<u8>>;

/// Convert image coordinates into ndarray coordinates
#[inline]
fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}
