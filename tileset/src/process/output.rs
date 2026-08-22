//! Output data to write to disk.

use crate::data::{flip::Flip, tile::TileSet};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, rc::Rc};

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
    pub template: Option<String>,

    /// Main image data for the output entry
    #[serde(flatten)]
    pub image: OutputImage,
}

impl OutputEntry {
    /// Create a new output entry
    #[inline]
    pub fn new(
        name: String,
        image: OutputImage,
        output_json: bool,
        template: Option<String>,
    ) -> Self {
        Self {
            name,
            output_json,
            template,
            image,
        }
    }
}

/// Result of processing a single image from the stack
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum OutputImage {
    /// Output a single static image
    Static(IndexMap),

    /// Output an animated image
    Animated(HashMap<String, OutputAnimation>),
}

/// Store the data to reconstruct an animated sprite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputAnimation {
    /// Produce a single animation
    Normal(IndexMap),

    /// Produce left and right variants of the animation
    LeftRight {
        /// Left variant of the animation
        left: IndexMap,

        /// RIght variant of the animation
        right: IndexMap,
    },

    /// Produce up and down variants of the animation
    UpDown {
        /// Up variant of the animation
        up: IndexMap,

        /// Down variant of the animation
        down: IndexMap,
    },

    /// Produce variants for all four directions
    FourWays {
        /// Up-Left variant of the animation
        up_left: IndexMap,

        /// Up-Right variant of the animation
        up_right: IndexMap,

        /// Down-Left variant of the animation
        down_left: IndexMap,

        /// Down-Right variant of the animation
        down_right: IndexMap,
    },
}

/// Indexes map to reconstruct the pictural data
#[derive(Debug, Clone)]
pub struct IndexMap(pub(crate) Rc<Array2<IndexTile>>);

/// Indexes to reconstruct the pictural data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexTile {
    /// Index of the tile
    pub(crate) tile: usize,

    /// Index of the palette
    pub(crate) palette: usize,

    /// Flip horizontally and/or vertically
    pub(crate) flip: Flip,
}

impl IndexTile {
    /// Create index data for a tile
    #[inline]
    pub fn new(tile: usize, palette: usize, flip: Flip) -> Self {
        Self {
            tile,
            palette,
            flip,
        }
    }
}

impl IndexMap {
    /// Create a new index map
    #[inline]
    pub fn new(data: Array2<IndexTile>) -> Self {
        Self(Rc::new(data))
    }
}

impl Serialize for IndexMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_ref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IndexMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data: Array2<IndexTile> = serde::Deserialize::deserialize(deserializer)?;
        Ok(IndexMap::new(data))
    }
}
