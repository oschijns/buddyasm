use crate::{input::{InputImage, InputStack}, tileset::{ Mapping, PaletteSetRgba, builder::BuilderConfig}};
use asefile::{AsepriteFile, AsepriteParseError};
use image::ImageError;
use serde::Deserialize;
use std::{ collections::HashMap, path::PathBuf, rc::Rc};

/// Error encountered when loading an input stack config
#[derive(thiserror::Error, Debug)]
pub enum InputStackError {

    /// Failed loading a palette
    #[error("Failed to load palette at path \"{0}\": {1}")]
    Palette(PathBuf, ImageError),

    /// Failed loading an aseprite file
    #[error("Failed loading Aseprite file \"{0}\": {1}")]
    Aseprite(PathBuf, AsepriteParseError),
}

/// Configuration of the input stack to process
#[derive(Debug, Clone, Deserialize)]
pub struct InputStackConfig {
    /// Tileset configuration
    config: BuilderConfig,

    /// Default palette to use
    default_palette: PathBuf,

    /// Entries to process
    #[serde(flatten)]
    entries: Vec<Entry>,
}

/// An input image to load
#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// Image to process
    image: PathBuf,

    /// Optional palette override
    #[serde(default)]
    palette: Option<PathBuf>,

    /// Optional fixed mapping
    #[serde(default)]
    mapping: Option<Vec<MapRange>>,
}

/// Map a sequence of tiles from the input image to a specific index
#[derive(Debug, Clone, Deserialize)]
pub struct MapRange {
    /// First tile of the sequence to map to a target
    #[serde(default)]
    start: usize,

    /// Number of tiles in the sequence
    size: usize,

    /// Target index of the tiles
    target: usize,
}


/// Load an input stack from the provided config
impl TryFrom<InputStackConfig> for InputStack {
    type Error = Vec<InputStackError>;

    fn try_from(value: InputStackConfig) -> Result<Self, Self::Error> {
        // Collect errors encountered
        let mut errors = Vec::with_capacity(value.entries.len());

        // Load the default palette
        let palette = match PaletteSetRgba::load_palette(&value.default_palette) {
            Ok(palette) => {
                palette
            },
            Err(err) => {
                // Without a default palette, we cannot do much => stop here
                errors.push( InputStackError::Palette(value.default_palette, err));
                return Err(errors);
            },
        };

        // Build a stack from the provided entries
        let mut stack = Vec::with_capacity(value.entries.len());

        // Process the entries
        for entry in value.entries.iter() {
            let path = &entry.image;

            // Check if it is an aseprite file
            if let Some(ext) = path.extension() && ext.to_ascii_lowercase() == "aseprite" {
                match AsepriteFile::read_file(path) {
                    Ok(image) => {
                        let image = InputImage::Animated(Box::new(image));
                        stack.push((path.clone(), image, palette.clone()));
                    },
                    Err(err) => {
                        errors.push(InputStackError::Aseprite(path.clone(), err));
                    }
                }
            } else if let Some(mapping) = &entry.mapping {
                // evaluate the number of entries to generate


            } else {

            };

        }

        // Check if we encountered errors
        if errors.is_empty() {
            Err(errors)
        } else {

        // Complete the stack
        Ok(Self { config: value.config, palette, stack })
        }
    }
}


/// Convert a list of ranges into a mapping
fn ranges_to_mapping(input: &[MapRange], width: usize, height: usize) -> Mapping {
    // evaluate the number of entries to generate
    let mut count = 0usize;
    for r in input.iter() {
        count += r.size;
    }

    let mut out = HashMap::with_capacity(count);

    Mapping::new(out)
}