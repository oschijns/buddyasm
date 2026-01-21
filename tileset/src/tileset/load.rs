use super::{Flip, PaletteSet};
use image::{ImageReader, ImageResult, Luma, LumaA, Rgb, Rgba};
use ndarray::{Array2, Ix2};
use std::path::Path;

/// Error encountered when parsing a flipping configuration
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[error("Could not parse \"{0}\" as a flipping configuration.")]
pub struct FlipParseError(String);

impl Flip {
    /// Parse the provided string to deduce a flipping configuration
    #[rustfmt::skip]
    pub fn parse(text: &str) -> Result<Self, FlipParseError> {
        let lower = text.to_ascii_lowercase();
        match lower.as_str() {
            "none"       | "null" | "nil" => Ok(Self::None),
            "horizontal" | "h"    | "x"   => Ok(Self::Horizontal),
            "vertical"   | "v"    | "y"   => Ok(Self::Vertical),
            "both"       | "hv"   | "xy"  => Ok(Self::Both),
            _ => Err(FlipParseError(text.to_string())),
        }
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
