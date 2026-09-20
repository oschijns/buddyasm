//! Define coordinates

use core::fmt;
use ndarray::{Ix, Ix2};

/// 2D coordinates
pub type Coords = [u16; 2];

/// Size of a tile in pixels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileSize {
    /// Width in pixels
    width: u16,

    /// Height in pixels
    height: u16,
}

/// Dimensions of an input image in tiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImgTileDim {
    /// Width in tiles
    width_tiles: u16,

    /// Height in tiles
    height_tiles: u16,

    /// Size of an individual tile
    tile_size: TileSize,
}

impl TileSize {
    /// Tile size from width and height
    #[inline]
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            width: width as u16,
            height: height as u16,
        }
    }

    /// Get the matrix dimension to use to store the tile pixel data
    #[inline]
    pub fn ndarray_dim(self) -> Ix2 {
        Ix2(self.width as Ix, self.height as Ix)
    }
}

impl ImgTileDim {
    /// Evaludate the number of tiles to process into an image.
    /// Provide the width and height of the image in pixels as well as the size of an individual tile.
    #[inline]
    pub const fn new(width: usize, height: usize, tile_size: TileSize) -> Self {
        Self {
            width_tiles: (width / tile_size.width as usize) as u16,
            height_tiles: (height / tile_size.height as usize) as u16,
            tile_size,
        }
    }

    /// Evaludate the number of tiles to process into an image.
    /// Provide the width and height of the image in pixels as well as the size of an individual tile.
    #[inline]
    pub const fn from_img(img_dim: (u32, u32), tile_size: TileSize) -> Self {
        Self {
            width_tiles: (img_dim.0 / tile_size.width as u32) as u16,
            height_tiles: (img_dim.1 / tile_size.height as u32) as u16,
            tile_size,
        }
    }

    /// Get width in tiles
    #[inline]
    pub const fn width_in_tiles(self) -> usize {
        self.width_tiles as usize
    }

    /// Get height in tiles
    #[inline]
    pub const fn height_in_tiles(self) -> usize {
        self.height_tiles as usize
    }

    /// Get the number of tiles to iterate over in this image
    #[inline]
    pub const fn tiles_count(self) -> usize {
        self.width_tiles as usize * self.height_tiles as usize
    }

    /// Return true if the coordinates selected is within bound
    #[inline]
    pub fn contains(self, x: usize, y: usize) -> bool {
        (x < self.width_tiles as usize) && (y < self.height_tiles as usize)
    }

    /// Return true if the coordinates selected is within bound
    #[inline]
    pub fn contains_16(self, x: u16, y: u16) -> bool {
        (x < self.width_tiles) && (y < self.height_tiles)
    }

    /// Convert index into 2D coordinates
    #[inline]
    pub fn index_to_coords(self, index: usize) -> [usize; 2] {
        let width = self.width_tiles as usize;
        [index % width, index / width]
    }

    /// Convert index into 2D coordinates for ndarray matrix
    #[inline]
    pub fn index_to_ix2(self, index: usize) -> Ix2 {
        let width = self.width_tiles as usize;
        Ix2(index % width, index / width)
    }

    /// Convert index into 2D coordinates encoded over 16-bits
    #[inline]
    pub fn index_to_coords_16(self, index: usize) -> Coords {
        let width = self.width_tiles as usize;
        [(index % width) as u16, (index / width) as u16]
    }

    /// Get the matrix dimension to use to store the tile index data
    #[inline]
    pub fn ndarray_dim(self) -> Ix2 {
        Ix2(self.width_tiles as Ix, self.height_tiles as Ix)
    }

    /// Given the index of the tile along the X and Y axes, get the bounding box
    /// covering the tile in pixels so that it can be used directly with
    /// ImageBuffer::view method.
    pub fn get_img_view(self, ix: usize, iy: usize) -> [u32; 4] {
        let sx = self.tile_size.width as u32;
        let sy = self.tile_size.height as u32;
        let px = ix as u32 * sx;
        let py = iy as u32 * sy;
        [px, py, sx, sy]
    }

    /// While iterating over the image by index, get the coordinates
    /// to use to extract an image view.
    pub fn index_to_img_view(self, index: usize) -> [u32; 4] {
        let [ix, iy] = self.index_to_coords(index);
        self.get_img_view(ix, iy)
    }
}

impl fmt::Display for TileSize {
    /// Print the tile coordinates
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) pix²", self.width, self.height)
    }
}

impl fmt::Display for ImgTileDim {
    /// Print the tile coordinates
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) tiles² of {}",
            self.width_tiles, self.height_tiles, self.tile_size
        )
    }
}
