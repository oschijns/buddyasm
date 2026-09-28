//! Palette

use crate::data::tileset::{Pix, Tile};
use image::{ImageResult, Pixel, Rgba, RgbaImage};
use ndarray::{Array2, Array3, Ix, Ix2, Ix3};
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

/// Set of palettes to look for in an input image
#[derive(Debug, Clone)]
pub struct Palette(pub(crate) Rc<Inner>);

/// Storage for the processed palette.
#[derive(Debug)]
pub(crate) struct Inner {
    /// Lookup matrix to identify the closest color in the palette for each pixel.
    /// This is based on a voronoid partitioning of the RGB color space.
    /// Any color value with an alpha channel below a certain threshold is
    /// considered transparent and assigned the index 0.
    pub(crate) lookup: Array3<u8>,

    /// Palette definition where each RGBA color has been identified by a unique index.
    pub(crate) palette: Array2<u8>,
}

/// Error encountered when trying to find a palette for a given tile
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("No matching palette for the tile")]
pub struct NoPaletteMatchError;

impl Palette {
    /// Load a palette from a file
    pub fn load_palette(path: &Path) -> ImageResult<Self> {
        let matrix = Inner::load_palette_from_image(path)?;
        let inner = Inner::matrix_to_palette(matrix);
        Ok(Self(Rc::new(inner)))
    }
}

impl Palette {
    /// Check the content of the provided sub image to try to deduce a palette
    /// index and an encoding of the tile. If no palette defined in this set
    /// matches the provided image, return an error.
    pub fn identify_tile(&self, img: &RgbaImage) -> Result<(usize, Tile), NoPaletteMatchError> {
        let inner = self.0.as_ref();
        let matrix = inner.identify_color_indexes(img);
        match inner.find_matching_palette(&matrix) {
            Ok((index, tile)) => Ok((index, Tile::new(tile))),
            Err(_) => Err(NoPaletteMatchError),
        }
    }
}

/// Discard any pixels with an alpha channel below
/// this threshold, treating them as transparent.
const ALPHA_THRESHOLD: u8 = 128;

impl Inner {
    /// Load a palette from an image file on disk.
    /// Returns a simple 2D matrix of RGBA colors.
    fn load_palette_from_image(path: &Path) -> ImageResult<Array2<Rgba<u8>>> {
        // Default color to fill the palette with initialy
        const CLEAR: Rgba<u8> = Rgba([0, 0, 0, 0]);

        // load the image into a RGBA image
        let img = image::ImageReader::open(path)?.decode()?.into_rgba8();

        // Convert it into a 2D matrix
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut matrix = Array2::from_elem((width, height), CLEAR);

        // Fill the matrix with data
        for (x, y, pix) in img.enumerate_pixels() {
            matrix[to_index(x, y)] = *pix;
        }

        // Return the palette
        Ok(matrix)
    }

    /// Convert a 2D matrix of RGBA colors into a usable palette.
    fn matrix_to_palette(matrix: Array2<Rgba<u8>>) -> Self {
        // Identify unique colors in the matrix
        let mut colorset = HashSet::with_capacity(matrix.len());
        for color in matrix.iter() {
            if color.alpha() >= ALPHA_THRESHOLD {
                colorset.insert(color.to_rgb());
            }
        }

        // Assign a unique index to each color encountered by converting the color set into a list
        let colorset = colorset
            .into_iter()
            .map(|rgb| {
                let [r, g, b] = rgb.0;
                let value = u32::from_ne_bytes([r, g, b, 0xFF]);
                ([r as i32, g as i32, b as i32], value)
            })
            .collect::<Vec<_>>();

        // Initialize the lookup table and fill it with color indexes
        const DIM: usize = 0x100;
        let mut lookup = Array3::zeros((DIM, DIM, DIM));
        for ((r, g, b), out) in lookup.indexed_iter_mut() {
            let r1 = r as i32;
            let g1 = g as i32;
            let b1 = b as i32;

            // Find the index of the closest color in the color set.
            let mut selected = (i32::MAX, u8::MAX);
            for (i, ([r0, g0, b0], _)) in colorset.iter().enumerate() {
                // Compute the squared distance between the current color and the lookup color
                #[inline]
                fn pow2(x: i32) -> i32 {
                    x * x
                }
                // 255^2 * 3 is below the maximum value of 2^32 so overflow is not possible
                let sqr_dist = pow2(r0 - r1) + pow2(g0 - g1) + pow2(b0 - b1);

                // If the squared distance is smaller than the current minimum, update the selection
                if sqr_dist < selected.0 {
                    selected = (sqr_dist, i as u8);
                }
            }
            *out = selected.1;
        }

        // Initialize the palette and identify the indexes of the colors
        let mut palette = Array2::zeros(matrix.dim());
        for (out, color) in palette.iter_mut().zip(matrix.iter()) {
            let [r, g, b, a] = color.0;
            if a >= ALPHA_THRESHOLD {
                let item = u32::from_ne_bytes([r, g, b, 0xFF]);

                // Find the index of the color
                if let Some(index) = colorset.iter().position(|(_, value)| *value == item) {
                    *out = index as u8;
                }
            } else {
                // Transparent pixels are assigned the maximum index to avoid conflicts with opaque colors
                *out = u8::MAX;
            }
        }

        Self { lookup, palette }
    }

    /// Given an input image, identify the indexes of the color of each pixel
    fn identify_color_indexes(&self, img: &RgbaImage) -> Array2<u8> {
        let mut matrix = Array2::zeros(to_index(img.width(), img.height()));

        // Assign color indexes to each pixel in the image
        for (out, pixel) in matrix.iter_mut().zip(img.pixels()) {
            let [r, g, b, a] = pixel.0;
            if a >= ALPHA_THRESHOLD {
                let index = Ix3(r as usize, g as usize, b as usize);
                *out = self.lookup[index];
            } else {
                *out = u8::MAX;
            }
        }

        matrix
    }

    /// Once each pixel has been remapped to its index,
    /// try to identify a matching palette for the tile.
    fn find_matching_palette(&self, matrix: &Array2<u8>) -> Result<(usize, Array2<u8>), ()> {
        // Allocate a tile to store the palette index for each pixel.
        // We cannot use the input matrix directly as we will successively
        // try each palette while writing into the tile.
        let mut tile = Array2::zeros(matrix.dim());

        // Try each palette successively
        'palette: for (pal_index, palette) in self.palette.columns().into_iter().enumerate() {
            // iterate over each pixel of the input image
            'pixel: for (out, pixel_id) in tile.iter_mut().zip(matrix.iter()) {
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

        Err(())
    }
}

/// Convert image coordinates into ndarray coordinates
#[inline]
const fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}
