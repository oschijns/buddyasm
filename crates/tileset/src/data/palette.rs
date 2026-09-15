//! Palette

use crate::data::{
    image::{Color, Img, to_img},
    tileset::{Pix, Tile},
};
use image::{ImageError, ImageReader};
use ndarray::{Array2, ArrayRef2, Ix2};
use std::{path::Path, rc::Rc};

/// Set of palettes to look for in an input image
#[derive(Debug, Clone)]
pub struct Palette(pub(crate) Rc<Img>);

/// Error encountered when trying to find a palette for a given tile
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("No matching palette for the tile")]
pub struct NoPaletteMatchError;

impl Palette {
    /// Create a new palette set
    #[inline]
    pub fn new(data: Img) -> Self {
        Self(Rc::new(data))
    }
}

impl Palette {
    /// Check the content of the provided sub image to try to deduce a palette
    /// index and an encoding of the tile. If no palette defined in this set
    /// matches the provided image, return an error.
    pub fn identify_tile(
        &self,
        img: &ArrayRef2<Color>,
    ) -> Result<(usize, bool, Tile), NoPaletteMatchError> {
        // Create a tile to store the result
        let mut tile = Array2::zeros(img.dim());

        // Has one pixel been set at the very least
        let mut pixel_set = false;

        // Try each palette successively
        'pal: for (i, palette) in self.0.rows().into_iter().enumerate() {
            // iterate over each pixel of the input image
            'pix: for ((x, y), &pixel) in img.indexed_iter() {
                let index = Ix2(x, y);

                // Check if the pixel is part of the palette selected
                for (j, &color) in palette.iter().enumerate() {
                    // Pixel of the image matches color from the selected palette
                    if pixel == color {
                        // Store the corresponding index in the tile we are making
                        tile[index] = j as Pix;
                        pixel_set = true;

                        // We can move on to the next pixel
                        continue 'pix;
                    }
                }

                // We have iterated over each color of the current palette.
                // We'll try the next palette.
                continue 'pal;
            }

            // We have filled the tile with indexes
            // the palette we used is a full match.
            return Ok((i, !pixel_set, Tile::new(tile)));
        }

        // We've look into each color of the palette selected but could
        // not find a match. We'll try again with the next palette.
        Err(NoPaletteMatchError)
    }
}

impl Palette {
    /// Load a palette from a file
    pub fn load_palette(path: &Path) -> Result<Self, ImageError> {
        // load the image into a RGBA image
        let image = ImageReader::open(path)?.decode()?.into_rgba8();
        Ok(Self::new(to_img(&image)))
    }
}
