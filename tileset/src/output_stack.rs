use crate::data::{coords::Coords, flip::Flip, tilemap::TileData, tileset::TileSet};
use core::{error, fmt};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    rc::Rc,
};

/// TileSet generated and associated index maps
#[derive(Debug)]
pub struct OutputStack {
    /// Generated tileset
    pub tileset: TileSet,

    /// Entries that have been processed from the input stack
    pub entries: Vec<OutputEntry>,
}

/// Result of processing a entry from the input stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputEntry {
    /// Name of the output entry
    pub name: String,

    /// Should the output JSON be generated?
    pub output_json: bool,

    /// Path to the template file to use for the output entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<PathBuf>,

    /// Main image data for the output entry
    #[serde(flatten)]
    pub image: OutputImage,
}

/// Result of processing a single image from the stack
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum OutputImage {
    /// Output a single static image
    Static(OutMap),

    /// Output an animated image
    Animated(HashMap<String, OutputAnimation>),
}

/// Store the data to reconstruct an animated sprite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputAnimation {
    /// Produce a single animation
    Normal(OutMap),

    /// Produce left and right variants of the animation
    LeftRight {
        /// Left variant of the animation
        left: OutMap,

        /// RIght variant of the animation
        right: OutMap,
    },

    /// Produce up and down variants of the animation
    UpDown {
        /// Up variant of the animation
        up: OutMap,

        /// Down variant of the animation
        down: OutMap,
    },

    /// Produce variants for all four directions
    FourWays {
        /// Up-Left variant of the animation
        up_left: OutMap,

        /// Up-Right variant of the animation
        up_right: OutMap,

        /// Down-Left variant of the animation
        down_left: OutMap,

        /// Down-Right variant of the animation
        down_right: OutMap,
    },
}

/// Indexes map to reconstruct the pictural data
#[derive(Debug, Clone)]
pub struct OutMap(pub Rc<Array2<OutTile>>);

/// Indexes to reconstruct the pictural data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutTile {
    /// Tile data identified
    #[serde(flatten)]
    pub data: TileData,

    /// Tile data encoded for the target system
    #[serde(flatten)]
    pub encoded: EncodedTile,
}

/// Encoded tile for the target system
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodedTile {
    /// Encoded tile index for the target system
    pub index: u16,

    /// Encoded tile attributes for the target system
    pub attr: u16,
}

impl OutMap {
    /// Create a new index map
    #[inline]
    pub fn new(data: Array2<OutTile>) -> Self {
        Self(Rc::new(data))
    }
}

impl Serialize for OutMap {
    /// Serialize the resulting map using serde
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_ref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OutMap {
    /// Deserialize a resulting map using serde
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data: Array2<OutTile> = serde::Deserialize::deserialize(deserializer)?;
        Ok(OutMap::new(data))
    }
}

// MARK: Error

/// Represents errors that occur during the processing of the input stack
#[derive(Debug)]
pub struct OutputStackError {
    /// List of errors grouped by file path
    pub errors: BTreeMap<PathBuf, Vec<OutError>>,
}

/// Formats the error message for the output stack error
impl fmt::Display for OutputStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (path, errors) in &self.errors {
            writeln!(f, "{}:", path.display())?;
            for error in errors {
                writeln!(f, "  - {}", error)?;
            }
        }
        Ok(())
    }
}

/// Implements the error trait for the output stack error
impl error::Error for OutputStackError {}

/// Error encountered when processing an input image
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutError {
    /// If no palette match the given tile
    #[error("No matching palette for tile at {0}")]
    NoPaletteMatch(Coords),

    /// There are too many different tiles in the provided image
    #[error("Too many distinct tiles starting at {0}")]
    DistinctOverflow(Coords),

    /// The requested tile position is out of the image
    #[error("Requested tile position {0} is out of bound")]
    OutOfBound(Coords),

    /// The given index is out of the boundaries of the target tileset
    #[error("Given index 0x{0:4x} is out of bound")]
    InvalidIndex(usize),
}
