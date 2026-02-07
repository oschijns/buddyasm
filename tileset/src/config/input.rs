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
    pub(crate) config: BuilderConfig,

    /// Default palette to use if one is not explicitely set
    pub(crate) palette: PaletteSetRgba,

    /// Stack of images to process
    pub(crate) stack: Vec<(PathBuf, InputImage, PaletteSetRgba)>,
}

/// Define one element to process
/// Either a static image or an animated sprite
#[derive(Debug)]
pub enum InputImage {
    /// static image
    Static(RgbaImage),

    /// Animated image from Aseprite
    Animated(Box<AsepriteFile>),

    /// static image with fixed target positions
    FixedPosition {
        /// Input image to process
        image: RgbaImage,

        /// Fixed mapping for the tiles
        mapping: Mapping,
    },
}

/// Define a configuration to process the input images
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct BuilderConfig {
    /// Size of the tileset to produce
    pub(crate) tile_count: usize,

    /// With of a tile in pixels
    pub(crate) tile_width: u32,

    /// Height of a tile in pixels
    pub(crate) tile_height: u32,

    /// Specify wherever tiles can be flipped horizontally
    pub(crate) flip_horizontal: bool,

    /// Specify wherever tiles can be flipped vertically
    pub(crate) flip_vertical: bool,
}

impl BuilderConfig {
    /// Get the size of the tile
    #[inline]
    pub fn tile_size(&self) -> TileSize {
        TileSize::new(self.tile_width, self.tile_height)
    }
}
