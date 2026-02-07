//! Palette

use crate::data::tile::{Pix, Tile};
use core::ops::Deref;
use image::{
    EncodableLayout, ImageBuffer, ImageReader, ImageResult, Luma, LumaA, Pixel, Rgb, Rgba,
};
use ndarray::{Array2, Ix, Ix2};
use std::path::Path;
use std::rc::Rc;

/// Set of palettes to look for in an input image
#[derive(Debug, Clone)]
pub struct PaletteSet<C>(pub(crate) Rc<Array2<C>>);

/// RGBA palette set
pub type PaletteSetRgba = PaletteSet<Rgba<u8>>;

/// RGB palette set
pub type PaletteSetRgb = PaletteSet<Rgb<u8>>;

/// Grayscale + Alpha palette set
pub type PaletteSetLumaA = PaletteSet<LumaA<u8>>;

/// Grayscale palette set
pub type PaletteSetLuma = PaletteSet<Luma<u8>>;

/// Error encountered when trying to find a palette for a given tile
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("No matching palette for the tile")]
pub struct NoPaletteMatchError;

impl<C> PaletteSet<C> {
    /// Create a new palette set
    #[inline]
    pub fn new(data: Array2<C>) -> Self {
        Self(Rc::new(data))
    }
}

impl<P> PaletteSet<P>
where
    P: Pixel,
{
    /// Check the content of the provided sub image to try to deduce a palette
    /// index and an encoding of the tile. If no palette defined in this set
    /// matches the provided image, return an error.
    pub fn identify_tile<Q>(
        &self,
        img: &ImageBuffer<P, Q>,
    ) -> Result<(usize, Tile), NoPaletteMatchError>
    where
        P: PartialEq,
        [P::Subpixel]: EncodableLayout,
        Q: Deref<Target = [P::Subpixel]>,
    {
        // Figure out the dimensions of the input image
        let (w, h) = img.dimensions();

        // Create a tile to store the result
        let mut tile = Array2::zeros(to_index(w, h));

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
            return Ok((i, Tile::new(tile)));
        }

        // We've look into each color of the palette selected but could
        // not find a match. We'll try again with the next palette.
        Err(NoPaletteMatchError)
    }
}

/// Implement load_palette for the various palette variants
macro_rules! impl_load_palette {
    ( $color:ty, $into:ident, $black:ident ) => {
        impl PaletteSet<$color> {
            /// Load a palette from a file
            pub fn load_palette(path: &Path) -> ImageResult<Self> {
                // Default color to fill the palette with initialy
                const BLACK: $color = $black();

                // load the image into a RGBA image
                let img = ImageReader::open(path)?.decode()?.$into();

                // Convert it into a 2D matrix
                let width = img.width() as usize;
                let height = img.height() as usize;
                let mut matrix = Array2::from_elem((width, height), BLACK);

                // Fill the matrix with data
                for (x, y, pix) in img.enumerate_pixels() {
                    matrix[Ix2(x as usize, y as usize)] = *pix;
                }

                Ok(Self::new(matrix))
            }
        }
    };
}

impl_load_palette! { Rgba  <u8>, into_rgba8      , black_rgba8      }
impl_load_palette! { Rgb   <u8>, into_rgb8       , black_rgb8       }
impl_load_palette! { LumaA <u8>, into_luma_alpha8, black_luma_alpha8}
impl_load_palette! { Luma  <u8>, into_luma8      , black_luma8      }

#[inline]
const fn black_rgba8() -> Rgba<u8> {
    Rgba([0, 0, 0, 0xff])
}

#[inline]
const fn black_rgb8() -> Rgb<u8> {
    Rgb([0, 0, 0])
}

#[inline]
const fn black_luma_alpha8() -> LumaA<u8> {
    LumaA([0, 0xff])
}

#[inline]
const fn black_luma8() -> Luma<u8> {
    Luma([0])
}

/// Convert image coordinates into ndarray coordinates
#[inline]
fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}
