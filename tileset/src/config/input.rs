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
    pub stack: Vec<InputEntry>,
}

/// Define an entry to process
#[derive(Debug)]
pub struct InputEntry {
    /// Path to the file to process
    pub path: PathBuf,

    /// Name of the entry (used as the output file name)
    pub name: String,

    /// Image to process
    pub image: InputImage,

    /// Palette to use for the image
    pub palette: PaletteSetRgba,

    /// Whether to output a JSON file with the tile data
    pub output_json: bool,

    /// Path to the template file to use for the output
    pub template: Option<PathBuf>,
}

impl InputEntry {
    /// Create a new input entry.
    #[inline]
    pub fn new(
        path: PathBuf,
        name: String,
        image: InputImage,
        palette: PaletteSetRgba,
        output_json: bool,
        template: Option<PathBuf>,
    ) -> Self {
        Self {
            path,
            name,
            image,
            palette,
            output_json,
            template,
        }
    }
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
