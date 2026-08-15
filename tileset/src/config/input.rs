//! Input data to process.

use crate::data::{coords::TileSize, mapping::Mapping, palette::PaletteSetRgba};
use asefile::AsepriteFile;
use image::RgbaImage;
use serde::Deserialize;
use std::path::PathBuf;

/// Define a stack of data to process
#[derive(Debug)]
pub struct InputStack {
    /// Configuration of the tileset to build
    pub config: BuilderConfig,

    /// Stack of images to process
    pub stack: Vec<(PathBuf, InputImage, PaletteSetRgba)>,
}

/// Define one element to process.
/// It can be a simple image to convert into tiles, or a font to embed in the
/// tileset or an animated sprite from Aseprite or a map made with Tiled.
#[derive(Debug)]
pub enum InputImage {
    /// static image
    Static(RgbaImage),

    /// static image with fixed target positions
    FixedPosition {
        /// Input image to process
        image: RgbaImage,

        /// Fixed mapping for the tiles
        mapping: Mapping,
    },

    /// Animated sprite from Aseprite
    Aseprite(Box<AsepriteFile>),

    /// Tile set from Tiled
    TiledTileset(Box<tiled::Tileset>),

    /// Tile map made in Tiled
    TiledMap(Box<tiled::Map>),
}

/// Define a configuration to process the input images
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct BuilderConfig {
    /// Size of the tileset to produce
    pub tile_count: usize,

    /// With of a tile in pixels
    pub tile_width: u32,

    /// Height of a tile in pixels
    pub tile_height: u32,

    /// Specify wherever tiles can be flipped horizontally
    pub flip_horizontal: bool,

    /// Specify wherever tiles can be flipped vertically
    pub flip_vertical: bool,
}

impl BuilderConfig {
    /// Create a config from parameters
    #[inline]
    pub fn new(
        tile_count: usize,
        tile_width: u32,
        tile_height: u32,
        flip_horizontal: bool,
        flip_vertical: bool,
    ) -> Self {
        Self {
            tile_count,
            tile_width,
            tile_height,
            flip_horizontal,
            flip_vertical,
        }
    }

    /// Get the size of the tile
    #[inline]
    pub fn tile_size(&self) -> TileSize {
        TileSize::new(self.tile_width, self.tile_height)
    }
}

impl Default for BuilderConfig {
    fn default() -> Self {
        Self {
            tile_count: 256,
            tile_width: 8,
            tile_height: 8,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }
}
