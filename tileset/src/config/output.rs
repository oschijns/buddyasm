//! Output data to write to disk.

use crate::data::{flip::Flip, tile::TileSet};
use ndarray::Array2;
use std::{collections::HashMap, path::PathBuf, rc::Rc};

/// TileSet generated and associated index maps
#[derive(Debug)]
pub struct OutputStack {
    /// Generated tileset
    pub tileset: TileSet,

    /// Associated index maps
    pub images: HashMap<PathBuf, OutputImage>,
}

/// Result of processing a single image from the stack
#[derive(Debug, Clone)]
pub enum OutputImage {
    /// Output a single static image
    Static(IndexMap),

    /// Output an animated image
    Animated(HashMap<String, OutputAnimation>),
}

/// Store the data to reconstruct an animated sprite
#[derive(Debug, Clone)]
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
        /// Left variant of the animation
        left: IndexMap,

        /// RIght variant of the animation
        right: IndexMap,

        /// Up variant of the animation
        up: IndexMap,

        /// Down variant of the animation
        down: IndexMap,
    },
}

/// Indexes map to reconstruct the pictural data
#[derive(Debug, Clone)]
pub struct IndexMap(pub(crate) Rc<Array2<IndexTile>>);

/// Indexes to reconstruct the pictural data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
