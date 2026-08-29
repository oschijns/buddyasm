use crate::{
    data::{coords::TileSize, flip::Flip, mapping::CharacterMapping, palette::PaletteSetRgba},
    profile::Profile,
};
use asefile::{AsepriteFile, AsepriteParseError};
use core::{error, fmt};
use image::{ImageError, RgbaImage};
use std::path::PathBuf;

/// Define a stack of data to process
#[derive(Debug)]
pub struct InputStack {
    /// System profile
    pub profile: Profile,

    /// Configuration of the tileset to build
    pub config: InputConfig,

    /// Stack of images to process
    pub stack: Vec<InputEntry>,
}

/// Define a configuration to process the input images
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputConfig {
    /// Size of the tileset to produce
    pub tile_count: usize,

    /// With and height of a tile in pixels
    pub tile_size: TileSize,

    /// Specify wherever tiles can be flipped horizontally and/or vertically
    pub flip: Flip,
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
        mapping: CharacterMapping,
    },

    /// Animated sprite from Aseprite
    Aseprite(Box<AsepriteFile>),

    /// Tile set from Tiled
    TiledTileset(Box<tiled::Tileset>),

    /// Tile map made in Tiled
    TiledMap(Box<tiled::Map>),
}

// MARK: Error

/// Represents errors that are encountered when loading the data to process.
#[derive(Debug)]
pub struct InputStackError {
    /// List of errors
    pub errors: Vec<InError>,
}

/// Formats the error message for the input stack error
impl fmt::Display for InputStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in &self.errors {
            writeln!(f, "  - {}", error)?;
        }
        Ok(())
    }
}

/// Implements the error trait for the input stack error
impl error::Error for InputStackError {}

/// Error encountered when loading an input stack config
#[derive(thiserror::Error, Debug)]
pub enum InError {
    /// Missing file extension
    #[error("Failed to identify file type \"{0}\"")]
    FileExt(PathBuf),

    /// Invalid entry name
    #[error("Missing name for entry with path \"{0}\"")]
    InvalidName(PathBuf),

    /// No palette specified
    #[error("No palette specified")]
    NoPalette,

    /// Failed loading a palette
    #[error("Failed to load palette at path \"{0}\": {1}")]
    Palette(PathBuf, ImageError),

    /// Failed loading an image file
    #[error("Failed loading image at path \"{0}\": {1}")]
    Image(PathBuf, ImageError),

    /// Failed loading an aseprite file
    #[error("Failed loading Aseprite file \"{0}\": {1}")]
    Aseprite(PathBuf, AsepriteParseError),

    /// Failed loading an TSX file
    #[error("Failed loading Tiled TSX file \"{0}\": {1}")]
    TiledTSX(PathBuf, tiled::Error),

    /// Failed loading an TMX file
    #[error("Failed loading Tiled TMX file \"{0}\": {1}")]
    TiledTMX(PathBuf, tiled::Error),
}
