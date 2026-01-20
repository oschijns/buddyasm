use super::{Flip, Tile};
use core::hash::{Hash, Hasher};
use ndarray::Axis;

impl Tile {
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

impl Flip {
    /// Return true if the tile is flipped horizontally
    #[inline]
    pub fn horizontal(self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }

    /// Return true if the tile is flipped vertically
    #[inline]
    pub fn vertical(self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
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
