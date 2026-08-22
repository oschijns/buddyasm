//! Structure of the manifest file describing all the elements to load and process.

use crate::{
    config::{
        input::{InputEntry, InputImage, InputStack},
        profile::{Hardware, Profile, TileKind, ToConfig},
    },
    data::{coords::Dimensions, mapping::Mapping, palette::PaletteSetRgba},
};
use asefile::{AsepriteFile, AsepriteParseError};
use buddyasm_common::manifest::Manifest;
use core::{error, fmt};
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
    #[serde(default)]
    pub default_palette: Option<PathBuf>,

    /// Path to the output directory relative to the manifest file.
    /// If not specified, defaults to `output` next to the manifest file.
    #[serde(default)]
    pub output: Option<PathBuf>,

    /// Path to the CHR file.
    /// If not specified, defaults to `tileset.chr` in the output directory.
    #[serde(default)]
    pub file_chr: Option<PathBuf>,

    /// List of templates includes if any.
    #[serde(default)]
    pub template_includes: Vec<PathBuf>,
}

/// An input image to load
#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// Image to process
    pub(crate) image: PathBuf,

    /// Name of the entry (used as the output file name)
    #[serde(default)]
    pub(crate) name: Option<String>,

    /// Optional palette override
    #[serde(default)]
    pub(crate) palette: Option<PathBuf>,

    /// Optional fixed mapping
    #[serde(default)]
    pub(crate) mapping: Option<Vec<MapRange>>,

    /// Specify if the entry should generate a JSON file.
    /// This is used to visualize the mapping of tiles to indices.
    /// Or to pass the mapping to an external tool.
    #[serde(default)]
    pub(crate) output_json: bool,

    /// Optional template file to use for this entry
    #[serde(default)]
    pub(crate) template: Option<PathBuf>,
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

/// Represents errors that are encountered when loading the data to process.
#[derive(Debug)]
pub struct InputStackError {
    /// List of errors
    pub errors: Vec<InputError>,
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
pub enum InputError {
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

/// Load an input stack from the provided config
impl TryFrom<TileSetManifest> for InputStack {
    type Error = InputStackError;

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
        let palette = if let Some(path) = &value.config.default_palette {
            let path = value.evaluate_path(path);
            match PaletteSetRgba::load_palette(&path) {
                Ok(palette) => Some(palette),
                Err(err) => {
                    // Default palette is not valid
                    errors.push(InputError::Palette(path, err));
                    None
                }
            }
        } else {
            None
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
                        // Palette specified for entry is not valid
                        errors.push(InputError::Palette(path, err));
                        continue;
                    }
                }
            } else if let Some(palette) = &palette {
                palette.clone()
            } else {
                // Without a default palette, is not valid
                errors.push(InputError::NoPalette);
                continue;
            };

            // Resolve the image path
            let img_path = value.evaluate_path(&entry.image);

            // Resolve the name of the entry (either set in the manifest or the file name)
            let name = if let Some(name) = &entry.name {
                name.clone()
            } else if let Some(name) = img_path.file_name().and_then(|n| n.to_str()) {
                name.to_string()
            } else {
                errors.push(InputError::InvalidName(img_path.clone()));
                continue;
            };

            // Resolve the path to the template file if one is specified
            let template_path = entry.template.as_deref().map(|t| value.evaluate_path(t));

            // Get the extension of the file
            let Some(ext) = img_path.extension() else {
                errors.push(InputError::FileExt(img_path.clone()));
                continue;
            };

            // Check if it is an aseprite file
            if ext.eq_ignore_ascii_case("aseprite") {
                match AsepriteFile::read_file(&img_path) {
                    Ok(image) => {
                        let image = InputImage::Aseprite(Box::new(image));

                        stack.push(InputEntry::new(
                            img_path.clone(),
                            name,
                            image,
                            palette,
                            entry.output_json,
                            template_path,
                        ));
                    }
                    Err(err) => {
                        errors.push(InputError::Aseprite(img_path.clone(), err));
                    }
                }
            } else if ext.eq_ignore_ascii_case("tsx") {
                match tiled_loader.load_tsx_tileset(&img_path) {
                    Ok(tileset) => {
                        let image = InputImage::TiledTileset(Box::new(tileset));
                        stack.push(InputEntry::new(
                            img_path.clone(),
                            name,
                            image,
                            palette,
                            entry.output_json,
                            template_path,
                        ));
                    }
                    Err(err) => {
                        errors.push(InputError::TiledTSX(img_path.clone(), err));
                    }
                }
            } else if ext.eq_ignore_ascii_case("tmx") {
                match tiled_loader.load_tmx_map(&img_path) {
                    Ok(map) => {
                        let image = InputImage::TiledMap(Box::new(map));
                        stack.push(InputEntry::new(
                            img_path.clone(),
                            name,
                            image,
                            palette,
                            entry.output_json,
                            template_path,
                        ));
                    }
                    Err(err) => {
                        errors.push(InputError::TiledTMX(img_path.clone(), err));
                    }
                }
            } else {
                // evaluate the number of entries to generate
                match open(&img_path) {
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
                        stack.push(InputEntry::new(
                            img_path.clone(),
                            name,
                            image,
                            palette,
                            entry.output_json,
                            template_path,
                        ));
                    }
                    Err(err) => {
                        errors.push(InputError::Image(img_path.clone(), err));
                    }
                }
            }
        }

        // Check if we encountered errors
        if errors.is_empty() {
            // Complete the stack
            Ok(Self { config, stack })
        } else {
            Err(InputStackError { errors })
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
