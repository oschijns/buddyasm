//! Structure of the manifest file describing all the elements to load and process.

use crate::{
    config::{
        input::{InputImage, InputStack},
        profile::{Hardware, Profile, TileKind, ToConfig},
    },
    data::{coords::Dimensions, mapping::Mapping, palette::PaletteSetRgba},
};
use asefile::{AsepriteFile, AsepriteParseError};
use buddyasm_common::manifest::Manifest;
use image::{ImageError, open};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Configuration of the input stack to process
#[derive(Debug, Clone, Deserialize)]
pub struct TileSetManifest {
    /// Absolute path to this manifest file
    #[serde(default)]
    path: PathBuf,

    /// Main configuration
    pub config: Config,

    /// Entries to process
    #[serde(alias = "entry")]
    pub(crate) entries: Vec<Entry>,
}

/// Configure main components such as default palette and target hardware
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Target hardware
    pub hardware: Hardware,

    /// Type of tiles to generate
    pub kind: TileKind,

    /// Sprite size parameter (if necessary)
    #[serde(default)]
    pub sprite_size: Option<String>,

    /// Bit plane configuration (if necessary)
    #[serde(default)]
    pub bit_plane: Option<String>,

    /// Default palette to use
    pub default_palette: PathBuf,
}

/// An input image to load
#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// Image to process
    pub(crate) image: PathBuf,

    /// Optional palette override
    #[serde(default)]
    pub(crate) palette: Option<PathBuf>,

    /// Optional fixed mapping
    #[serde(default)]
    pub(crate) mapping: Option<Vec<MapRange>>,
}

/// Map a sequence of tiles from the input image to a specific index
#[derive(Debug, Clone, Deserialize)]
pub struct MapRange {
    /// First tile of the sequence to map to a target
    #[serde(default)]
    pub(crate) start: usize,

    /// Last tile of the sequence to map (excluded)
    pub(crate) end: usize,

    /// Target index of the tiles
    pub(crate) target: usize,
}

/// Error encountered when loading an input stack config
#[derive(thiserror::Error, Debug)]
pub enum InputStackError {
    /// Missing file extension
    #[error("Failed to identify file type \"{0}\"")]
    FileExt(PathBuf),

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

/// Load an input stack from the provided config
impl TryFrom<TileSetManifest> for InputStack {
    type Error = Vec<InputStackError>;

    fn try_from(value: TileSetManifest) -> Result<Self, Self::Error> {
        let profile = Profile::new(
            value.config.hardware,
            value.config.kind,
            value.config.sprite_size.as_deref(),
        )
        .expect("Invalid config in hardware specification");
        let config = profile.to_config();
        let tile_size = config.tile_size();

        // Collect errors encountered
        let mut errors = Vec::with_capacity(value.entries.len());

        // Load the default palette
        let path = value.evaluate_path(&value.config.default_palette);
        let palette = match PaletteSetRgba::load_palette(&path) {
            Ok(palette) => palette,
            Err(err) => {
                // Without a default palette, we cannot do much => stop here
                errors.push(InputStackError::Palette(path, err));
                return Err(errors);
            }
        };

        // Create a loader for tiled files
        let mut tiled_loader = tiled::Loader::new();

        // Build a stack from the provided entries
        let mut stack = Vec::with_capacity(value.entries.len());

        // Process the entries
        for entry in value.entries.iter() {
            // Check if we have a palette override for this entry
            let palette = if let Some(path_palette) = &entry.palette {
                let path = value.evaluate_path(path_palette);
                match PaletteSetRgba::load_palette(&path) {
                    Ok(palette) => palette,
                    Err(err) => {
                        // Without a default palette, we cannot do much => stop here
                        errors.push(InputStackError::Palette(path, err));
                        return Err(errors);
                    }
                }
            } else {
                palette.clone()
            };

            let path = value.evaluate_path(&entry.image);

            // Get the extension of the file
            let Some(ext) = path.extension() else {
                errors.push(InputStackError::FileExt(path.clone()));
                continue;
            };

            // Check if it is an aseprite file
            if ext.eq_ignore_ascii_case("aseprite") {
                match AsepriteFile::read_file(&path) {
                    Ok(image) => {
                        let image = InputImage::Aseprite(Box::new(image));
                        stack.push((path.clone(), image, palette));
                    }
                    Err(err) => {
                        errors.push(InputStackError::Aseprite(path.clone(), err));
                    }
                }
            } else if ext.eq_ignore_ascii_case("tsx") {
                match tiled_loader.load_tsx_tileset(&path) {
                    Ok(tileset) => {
                        let image = InputImage::TiledTileset(Box::new(tileset));
                        stack.push((path.clone(), image, palette));
                    }
                    Err(err) => {
                        errors.push(InputStackError::TiledTSX(path.clone(), err));
                    }
                }
            } else if ext.eq_ignore_ascii_case("tmx") {
                match tiled_loader.load_tmx_map(&path) {
                    Ok(map) => {
                        let image = InputImage::TiledMap(Box::new(map));
                        stack.push((path.clone(), image, palette));
                    }
                    Err(err) => {
                        errors.push(InputStackError::TiledTMX(path.clone(), err));
                    }
                }
            } else {
                // evaluate the number of entries to generate
                match open(&path) {
                    Ok(image) => {
                        // We only handle RGBA images
                        let image = image.to_rgba8();

                        // Check if we require a fixed mapping
                        let image = if let Some(ranges) = &entry.mapping {
                            // When a fixed mapping is required processing mapping data
                            let dims = Dimensions::from_img(image.dimensions(), tile_size);
                            let mapping = Mapping::from_ranges(dims, ranges);
                            InputImage::FixedPosition { image, mapping }
                        } else {
                            // Static image to process
                            InputImage::Static(image)
                        };
                        stack.push((path.clone(), image, palette));
                    }
                    Err(err) => {
                        errors.push(InputStackError::Image(path.clone(), err));
                    }
                }
            }
        }

        // Check if we encountered errors
        if errors.is_empty() {
            // Complete the stack
            Ok(Self {
                config,
                palette,
                stack,
            })
        } else {
            Err(errors)
        }
    }
}

impl MapRange {
    /// Get the number of tiles covered by the map range
    #[inline]
    pub fn size(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

impl Manifest for TileSetManifest {
    /// Path to the directory containing this manifest file
    fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    /// Path to the directory containing this manifest file
    #[inline]
    fn get_path(&self) -> &Path {
        &self.path
    }
}
