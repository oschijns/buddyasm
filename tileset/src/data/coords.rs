//! Define coordinates

use core::fmt;
use ndarray::{Ix, Ix2};

/// 2D vector
pub type Vec2 = [u32; 2];

/// Coordinates of a tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords(pub(crate) Vec2);

/// Size of a tile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileSize(pub(crate) Vec2);

/// Dimensions of an input image in tiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions(pub(crate) Vec2);

impl Coords {
    /// Coordinates from X and Y
    #[inline]
    pub const fn new(x: u32, y: u32) -> Self {
        Self([x, y])
    }

    /// Get the bounding box covering the tile in pixels
    /// so that it can be used directly with ImageBuffer::view method.
    pub fn bounds(self, size: TileSize) -> [u32; 4] {
        let [tx, ty] = self.0;
        let [sx, sy] = size.0;
        let px = tx * sx;
        let py = ty * sy;
        [px, py, px + sx, py + sy]
    }
}

impl TileSize {
    /// Tile size from width and height
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self([width, height])
    }
}

impl Dimensions {
    /// Dimensions from width and height in tiles
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self([width, height])
    }

    /// Get width in tiles
    #[inline]
    pub const fn width(self) -> u32 {
        self.0[0]
    }

    /// Get height in tiles
    #[inline]
    pub const fn height(self) -> u32 {
        self.0[1]
    }

    /// Get tiles count
    #[inline]
    pub const fn count(self) -> usize {
        let [w, h] = self.0;
        (w * h) as usize
    }

    /// Return true if the coordinates selected is within bound
    #[inline]
    pub fn contains(self, coords: Coords) -> bool {
        let [w, h] = self.0;
        let [x, y] = coords.0;
        x < w && y < h
    }

    /// Dimensions from width and height of image with tile size
    #[inline]
    pub fn from_img(img_dim: (u32, u32), size: TileSize) -> Self {
        let (ix, iy) = img_dim;
        let [sx, sy] = size.0;
        Self([ix / sx, iy / sy])
    }

    /// Convert index into 2D coordinates
    #[inline]
    pub fn to_coords(self, index: usize) -> Coords {
        let index = index as u32;
        let width = self.0[0];
        Coords([index % width, index / width])
    }
}

/// Convert 2D coordinates into ndarray index
impl From<Coords> for Ix2 {
    #[inline]
    fn from(value: Coords) -> Self {
        let [x, y] = value.0;
        Ix2(x as Ix, y as Ix)
    }
}

/// Convert 2D dimensions into ndarray index
impl From<Dimensions> for Ix2 {
    #[inline]
    fn from(value: Dimensions) -> Self {
        let [x, y] = value.0;
        Ix2(x as Ix, y as Ix)
    }
}

impl fmt::Display for Coords {
    /// Print the tile coordinates
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [x, y] = self.0;
        write!(f, "({x}, {y}) tile²")
    }
}

impl fmt::Display for TileSize {
    /// Print the tile coordinates
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [x, y] = self.0;
        write!(f, "({x}, {y}) pix²")
    }
}

impl fmt::Display for Dimensions {
    /// Print the tile coordinates
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [x, y] = self.0;
        write!(f, "({x}, {y}) tile²")
    }
}
