//! Palette

use crate::data::{
    coords::TileSize,
    tileset::{Pix, Tile},
};
use buddyasm_common::color_space::{ColorA, ColorSpace, ColorSpaceBuilder, LockError};
use image::{ImageResult, Rgb, RgbaImage};
use ndarray::{Array2, Ix, Ix2};
use std::{
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
};

/// Set of palettes to look for in an input image
#[derive(Debug, Clone)]
pub struct Palette {
    /// Color space partitioning
    partition: ColorSpace,

    /// Palette definition where each RGBA color
    /// has been identified by a unique index.
    palette: Arc<Array2<u8>>,

    /// threshold for discarding colors based on their alpha channel
    pub alpha_threshold: u8,
}

/// Work buffer for identifying
pub type WorkBuffer = Array2<u8>;

/// Allocate a work buffer to process tiles
#[inline]
pub fn make_workbuffer(tile_size: TileSize) -> WorkBuffer {
    Array2::zeros(tile_size.ndarray_dim())
}

/// Error encountered when trying to find a palette for a given tile
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteError {
    #[error("No matching palette found")]
    NoMatch,

    #[error("{0}")]
    Lock(#[from] LockError),
}

impl Palette {
    /// Load a palette from a file
    pub fn load_from_disk(path: &Path, alpha_threshold: u8) -> ImageResult<Self> {
        let matrix = load_matrix_from_image(path)?;
        Ok(Self::from_matrix(matrix, alpha_threshold))
    }

    /// Convert a 2D matrix of RGBA colors into a usable palette.
    pub fn from_matrix(matrix: Array2<ColorA>, alpha_threshold: u8) -> Self {
        // Identify unique colors in the matrix
        let mut builder = ColorSpaceBuilder::with_capacity(matrix.len());
        builder.alpha_threshold = alpha_threshold;
        for color in matrix.iter() {
            builder.add_a(*color);
        }
        let partition = builder.finish();

        // Initialize the palette and identify the indexes of the colors
        let mut palette = Array2::zeros(matrix.dim());
        for (out, &[r, g, b, a]) in palette.iter_mut().zip(matrix.iter()) {
            // Transparent pixels are assigned the maximum index to avoid conflicts with opaque colors
            if a >= alpha_threshold
                && let Some(index) = partition.get_color_index([r, g, b])
            {
                *out = index as u8;
            } else {
                *out = u8::MAX;
            }
        }

        Self {
            partition,
            palette: Arc::new(palette),
            alpha_threshold,
        }
    }
}

impl Palette {
    /// Check the content of the provided sub image to try to deduce a palette
    /// index and an encoding of the tile. If no palette defined in this set
    /// matches the provided image, return an error.
    pub fn identify_tile(
        &mut self,
        img: &RgbaImage,
        workbuffer: &mut WorkBuffer,
    ) -> Result<(usize, Tile), PaletteError> {
        // Identify the indexes of each color in the input image.
        self.identify_color_indexes(img, workbuffer)?;

        // Then find a matching palette for the tile being processed.
        let (index, tile) = self.find_matching_palette(workbuffer)?;
        Ok((index, Tile::new(tile)))
    }
}

/// Load a palette matrix from an image file on disk.
/// Returns a simple 2D matrix of RGBA colors.
fn load_matrix_from_image(path: &Path) -> ImageResult<Array2<ColorA>> {
    // load the image into a RGBA image
    let img = image::ImageReader::open(path)?.decode()?.into_rgba8();

    // Convert it into a 2D matrix
    let width = img.width() as usize;
    let height = img.height() as usize;
    let mut matrix = Array2::default((width, height));

    // Fill the matrix with data
    for (x, y, pix) in img.enumerate_pixels() {
        matrix[to_index(x, y)] = pix.0;
    }

    // Return the palette
    Ok(matrix)
}

impl Palette {
    /// Given an input image, identify the indexes of the color of each pixel
    fn identify_color_indexes(
        &self,
        img: &RgbaImage,
        workbuffer: &mut WorkBuffer,
    ) -> Result<(), PaletteError> {
        // Assign color indexes to each pixel in the image
        for (out, &pixel) in workbuffer.iter_mut().zip(img.pixels()) {
            let [r, g, b, a] = pixel.0;
            if a >= self.alpha_threshold {
                *out = self.partition.identify_color_index([r, g, b])? as u8;
            } else {
                *out = u8::MAX;
            }
        }
        Ok(())
    }

    /// Once each pixel has been remapped to its index,
    /// try to identify a matching palette for the tile.
    fn find_matching_palette(
        &self,
        workbuffer: &WorkBuffer,
    ) -> Result<(usize, Array2<u8>), PaletteError> {
        // Allocate a tile to store the palette index for each pixel.
        // We cannot use the input matrix directly as we will successively
        // try each palette while writing into the tile.
        let mut tile = Array2::zeros(workbuffer.dim());

        // Try each palette successively
        'palette: for (pal_index, palette) in self.palette.columns().into_iter().enumerate() {
            // iterate over each pixel of the input image
            'pixel: for (out, pixel_id) in tile.iter_mut().zip(workbuffer.iter()) {
                // Check if the pixel is part of the palette selected
                for (col_index, color_id) in palette.iter().enumerate() {
                    // Pixel of the image matches color from the selected palette
                    if *pixel_id == *color_id {
                        // Store the corresponding index in the tile we are making
                        *out = col_index as Pix;

                        // We can move on to the next pixel
                        continue 'pixel;
                    }
                }

                // We have iterated over each color of the current palette.
                // We'll try the next palette.
                continue 'palette;
            }

            // We have filled the tile with indexes
            // the palette we used is a full match.
            return Ok((pal_index, tile));
        }

        Err(PaletteError::NoMatch)
    }
}

/// Convert image coordinates into ndarray coordinates
#[inline]
const fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}
