use super::{Flip, Tile};
use ndarray::Axis;

impl Tile {
    /// Flip the tile and build a new tile
    pub fn flip(&self, flip: Flip) -> Self {
        let mut data = self.0.as_ref().clone();
        match flip {
            Flip::None => Self::new(data),
            Flip::Horizontal => {
                data.invert_axis(Axis(0));
                Self::new(data)
            }
            Flip::Vertical => {
                data.invert_axis(Axis(1));
                Self::new(data)
            }
            Flip::Both => {
                data.invert_axis(Axis(0));
                data.invert_axis(Axis(1));
                Self::new(data)
            }
        }
    }

    /// Compare both tile to deduce if they are identical with possible flipping.
    pub fn similarity(&self, other: &Self, flip_h: bool, flip_v: bool) -> Option<Flip> {
        // access both tiles
        let tile0 = self.0.as_ref();
        let tile1 = other.0.as_ref();

        // Check if they are identical as is
        if tile0 == tile1 {
            return Some(Flip::None);
        }

        // Check if they are identical after a horizontal flip
        if flip_h {
            let mut tile1 = tile1.clone();
            tile1.invert_axis(Axis(0));
            if tile0 == tile1 {
                return Some(Flip::Horizontal);
            }
        }

        // Check if they are identical after a vertical flip
        if flip_v {
            let mut tile1 = tile1.clone();
            tile1.invert_axis(Axis(1));
            if tile0 == tile1 {
                return Some(Flip::Vertical);
            }
        }

        // Check if they are identical after horizontal and vertical flips
        if flip_h && flip_v {
            let mut tile1 = tile1.clone();
            tile1.invert_axis(Axis(0));
            tile1.invert_axis(Axis(1));
            if tile0 == tile1 {
                return Some(Flip::Both);
            }
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
