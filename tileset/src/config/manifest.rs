//! Structure of the manifest file describing all the elements to load and process.

use crate::{
    config::input::{BuilderConfig, InputImage, InputStack},
    data::{coords::Dimensions, mapping::Mapping, palette::PaletteSetRgba},
};
use asefile::{AsepriteFile, AsepriteParseError};
use image::{ImageError, open};
use serde::Deserialize;
use std::path::PathBuf;

/// Configuration of the input stack to process
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    /// Tileset configuration
    pub(crate) config: BuilderConfig,

    /// Default palette to use
    pub(crate) default_palette: PathBuf,

    /// Entries to process
    #[serde(flatten)]
    pub(crate) entries: Vec<Entry>,
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
    /// Failed loading a palette
    #[error("Failed to load palette at path \"{0}\": {1}")]
    Palette(PathBuf, ImageError),

    /// Failed loading an image file
    #[error("Failed loading image at path \"{0}\": {1}")]
    Image(PathBuf, ImageError),

    /// Failed loading an aseprite file
    #[error("Failed loading Aseprite file \"{0}\": {1}")]
    Aseprite(PathBuf, AsepriteParseError),
}

/// Load an input stack from the provided config
impl TryFrom<Manifest> for InputStack {
    type Error = Vec<InputStackError>;

    fn try_from(value: Manifest) -> Result<Self, Self::Error> {
        let tile_size = value.config.tile_size();

        // Collect errors encountered
        let mut errors = Vec::with_capacity(value.entries.len());

        // Load the default palette
        let palette = match PaletteSetRgba::load_palette(&value.default_palette) {
            Ok(palette) => palette,
            Err(err) => {
                // Without a default palette, we cannot do much => stop here
                errors.push(InputStackError::Palette(value.default_palette, err));
                return Err(errors);
            }
        };

        // Build a stack from the provided entries
        let mut stack = Vec::with_capacity(value.entries.len());

        // Process the entries
        for entry in value.entries.iter() {
            let path = &entry.image;

            // Check if it is an aseprite file
            if let Some(ext) = path.extension()
                && ext.eq_ignore_ascii_case("aseprite")
            {
                match AsepriteFile::read_file(path) {
                    Ok(image) => {
                        let image = InputImage::Animated(Box::new(image));
                        stack.push((path.clone(), image, palette.clone()));
                    }
                    Err(err) => {
                        errors.push(InputStackError::Aseprite(path.clone(), err));
                    }
                }
            } else {
                // evaluate the number of entries to generate
                match open(&entry.image) {
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
                        stack.push((path.clone(), image, palette.clone()));
                    }
                    Err(err) => {
                        errors.push(InputStackError::Image(path.clone(), err));
                    }
                }
            }
        }

        // Check if we encountered errors
        if errors.is_empty() {
            Err(errors)
        } else {
            // Complete the stack
            Ok(Self {
                config: value.config,
                palette,
                stack,
            })
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
