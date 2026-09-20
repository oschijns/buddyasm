//! Palette

use crate::data::tileset::{Pix, Tile};
use image::{ImageResult, Rgba, RgbaImage};
use ndarray::{Array2, Ix, Ix2};
use std::path::Path;
use std::rc::Rc;

/// Set of palettes to look for in an input image
#[derive(Debug, Clone)]
pub struct Palette(pub(crate) Rc<Array2<Rgba<u8>>>);

/// Error encountered when trying to find a palette for a given tile
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("No matching palette for the tile")]
pub struct NoPaletteMatchError;

impl Palette {
    /// Load a palette from a file
    pub fn load_palette(path: &Path) -> ImageResult<Self> {
        // Default color to fill the palette with initialy
        const BLACK: Rgba<u8> = Rgba([0, 0, 0, 0xFF]);

        // load the image into a RGBA image
        let img = image::ImageReader::open(path)?.decode()?.into_rgba8();

        // Convert it into a 2D matrix
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut matrix = Array2::from_elem((width, height), BLACK);

        // Fill the matrix with data
        for (x, y, pix) in img.enumerate_pixels() {
            matrix[Ix2(x as usize, y as usize)] = *pix;
        }

        // Return the palette
        Ok(Self(Rc::new(matrix)))
    }
}

impl Palette {
    /// Check the content of the provided sub image to try to deduce a palette
    /// index and an encoding of the tile. If no palette defined in this set
    /// matches the provided image, return an error.
    pub fn identify_tile(
        &self,
        img: &RgbaImage,
    ) -> Result<(usize, bool, Tile), NoPaletteMatchError> {
        // Figure out the dimensions of the input image
        let (w, h) = img.dimensions();

        // Create a tile to store the result
        let mut tile = Array2::zeros(to_index(w, h));
        let mut pixel_set = false;

        // Try each palette successively
        'pal: for (i, palette) in self.0.columns().into_iter().enumerate() {
            // iterate over each pixel of the input image
            'pix: for (x, y, pixel) in img.enumerate_pixels() {
                // Check if the pixel is part of the palette selected
                for (j, color) in palette.iter().enumerate() {
                    // Pixel of the image matches color from the selected palette
                    if *pixel == *color {
                        // Store the corresponding index in the tile we are making
                        tile[to_index(x, y)] = j as Pix;
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

/// Convert image coordinates into ndarray coordinates
#[inline]
fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}
