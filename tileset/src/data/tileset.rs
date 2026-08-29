//! Tile and TileSet

use crate::data::{flip::Flip, palette::to_index};
use core::hash::{Hash, Hasher};
use ndarray::{Array2, Axis};
use std::rc::Rc;

/// Define a pixel in a tile
pub type Pix = u8;

/// Tile to serialize in a given binary layout
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tile(pub(crate) Rc<Array2<Pix>>);

/// Tileset containing tiles
#[derive(Debug, Clone)]
pub struct TileSet(pub(crate) Rc<Vec<Tile>>);

impl TileSet {
    /// Create a new tileset from raw data
    #[inline]
    pub fn new(data: Vec<Tile>) -> Self {
        Self(Rc::new(data))
    }

    /// Get the number of tiles in this tileset
    #[inline]
    pub fn count(&self) -> usize {
        self.0.as_ref().len()
    }

    /// Get the total number of pixels in this tileset.
    /// This assumes that all tiles stored have the same size.
    #[inline]
    pub fn pixel_count(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.count() * self.0.as_ref()[0].pixel_count()
        }
    }
}

impl Tile {
    /// Create a new tile from data
    #[inline]
    pub fn new(mut data: Array2<Pix>) -> Self {
        // reorder pixels to be easier to serialize
        data.reverse_axes();
        data.invert_axis(Axis(1));
        Self(Rc::new(data))
    }

    /// Create a new empty tile
    #[inline]
    pub fn new_empty(w: u32, h: u32) -> Self {
        Self(Rc::new(Array2::zeros(to_index(w, h))))
    }

    /// Get the number of pixels in this tile
    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.0.as_ref().len()
    }

    /// Flip the tile horizontally
    #[inline]
    pub fn flip_horizontal(&self) -> Self {
        let mut data = self.0.as_ref().clone();
        data.invert_axis(Axis(0));
        Self::new(data)
    }

    /// Flip the tile vertically
    #[inline]
    pub fn flip_vertical(&self) -> Self {
        let mut data = self.0.as_ref().clone();
        data.invert_axis(Axis(1));
        Self::new(data)
    }

    /// Flip the tile both ways
    #[inline]
    pub fn flip_both(&self) -> Self {
        let mut data = self.0.as_ref().clone();
        data.invert_axis(Axis(0));
        data.invert_axis(Axis(1));
        Self::new(data)
    }

    /// Flip the tile and build a new tile
    pub fn flip(&self, flip: Flip) -> Self {
        match flip {
            Flip::None => self.clone(),
            Flip::Horizontal => self.flip_horizontal(),
            Flip::Vertical => self.flip_vertical(),
            Flip::Both => self.flip_both(),
        }
    }

    /// Compare both tile to deduce if they are identical with possible flipping.
    pub fn similarity(&self, other: &Self, flip_h: bool, flip_v: bool) -> Option<Flip> {
        let ref_tile = self.0.as_ref();

        // Check if they are identical as is
        if ref_tile == other.0.as_ref() {
            return Some(Flip::None);
        }

        // Check if they are identical after a horizontal flip
        if flip_h && ref_tile == other.flip_horizontal().0.as_ref() {
            return Some(Flip::Horizontal);
        }

        // Check if they are identical after a vertical flip
        if flip_v && ref_tile == other.flip_vertical().0.as_ref() {
            return Some(Flip::Vertical);
        }

        // Check if they are identical after horizontal and vertical flips
        if flip_h && flip_v && ref_tile == other.flip_both().0.as_ref() {
            return Some(Flip::Both);
        }

        None
    }
}

impl Hash for Tile {
    /// Generate a hash from the provided tile data
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}
