use crate::{
    data::{coords::TileSize, flip::Flip, mapping::CharacterMapping, palette::PaletteSetRgba},
    profile::Profile,
};
use aseprite_loader::loader::{AsepriteFile, LoadSpriteError};
use core::{error, fmt};
use image::{ImageError, RgbaImage};
use ouroboros::self_referencing;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

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
    Aseprite(Aseprite),

    /// Tile set from Tiled
    TiledTileset(Box<tiled::Tileset>),

    /// Tile map made in Tiled
    TiledMap(Box<tiled::Map>),
}

/// Wrapper for owned Aseprite file
#[self_referencing]
pub struct Aseprite {
    /// Binary data loaded from the file
    raw: Vec<u8>,

    /// Parsed data
    #[borrows(raw)]
    #[covariant]
    file: Box<AsepriteFile<'this>>,
}

impl Aseprite {
    /// Load Aseprite file from disk
    pub fn load(path: &Path) -> Result<Self, InError> {
        let raw = fs::read(path)?;
        Self::try_new(raw, |raw| match AsepriteFile::load(&raw) {
            Ok(file) => Ok(Box::new(file)),
            Err(err) => Err(InError::Aseprite(err)),
        })
    }
}

/// Manual implementation of Debug trait because of ouroboros macro
impl fmt::Debug for Aseprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Aseprite").finish()
    }
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
    #[error("Failed to identify file type")]
    FileExt,

    /// Invalid entry name
    #[error("Missing name for entry")]
    InvalidName,

    /// No palette specified
    #[error("No palette specified")]
    NoPalette,

    /// Failed to load file
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Failed loading a palette
    #[error("Failed to load palette at path: {0}")]
    Palette(ImageError),

    /// Failed loading an image file
    #[error("Failed loading image at path: {0}")]
    Image(#[from] ImageError),

    /// Failed loading an aseprite file
    #[error("Failed loading Aseprite file: {0}")]
    Aseprite(#[from] LoadSpriteError),

    /// Failed loading an TSX or TMX file
    #[error("Failed loading Tiled file: {0}")]
    Tiled(#[from] tiled::Error),
}
