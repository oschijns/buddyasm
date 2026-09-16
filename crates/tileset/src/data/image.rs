use image::{DynamicImage, ImageBuffer, Luma, LumaA, Pixel, Rgb, Rgba};
use ndarray::{Array2, Ix, Ix2};
use std::{fmt, ops::Deref};

/// Common image format for processing.
/// Basically a 2D matrix of 32 bits values.
pub type Img = Array2<Color>;

/// Convert a dynamic image format into a simple matrix
#[rustfmt::skip]
pub fn dyn_to_img(dyn_img: &DynamicImage) -> Img {
    match dyn_img {
        DynamicImage::ImageLuma8  (image) => to_img(image),
        DynamicImage::ImageLumaA8 (image) => to_img(image),
        DynamicImage::ImageRgb8   (image) => to_img(image),
        DynamicImage::ImageRgba8  (image) => to_img(image),
        DynamicImage::ImageLuma16 (image) => to_img(image),
        DynamicImage::ImageLumaA16(image) => to_img(image),
        DynamicImage::ImageRgb16  (image) => to_img(image),
        DynamicImage::ImageRgba16 (image) => to_img(image),
        DynamicImage::ImageRgb32F (image) => to_img(image),
        DynamicImage::ImageRgba32F(image) => to_img(image),
        _ => unimplemented!("Unsupported image variant"),
    }
}

/// Convert the image format into a simple matrix
pub fn to_img<P, Q>(image: &ImageBuffer<P, Q>) -> Img
where
    P: Pixel + ToPix,
    Q: Deref<Target = [P::Subpixel]>,
{
    // Allocate a new buffer with the size of the image
    let (w, h) = image.dimensions();
    let mut out_img = Img::zeros((w as usize, h as usize));

    // TODO: check the pixels are in the right order
    // Copy the image into the matrix
    for (in_, out) in image.pixels().zip(out_img.iter_mut()) {
        *out = in_.to_pix();
    }

    out_img
}

/// Wrapper to print the image data
pub struct PrintImg<'i>(pub &'i Img);

impl<'i> fmt::Display for PrintImg<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for col in self.0.rows() {
            write!(f, "[ ")?;
            for &cell in col.iter() {
                write!(f, "{:0>8x}, ", cell)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

/// Convert image coordinates into ndarray coordinates
#[inline]
pub(crate) fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}

/// Define pixel type as a raw 32 bits integer containing the four RGBA
/// channels in a single value for quick equality comparisons.
pub type Color = u32;

/// Convert an arbitrary color type into a generic pixel
pub trait ToPix {
    fn to_pix(self) -> Color;
}

/// Convert the Luma type into an 32 bits integer
impl ToPix for Luma<u8> {
    #[inline]
    fn to_pix(self) -> Color {
        let [l] = self.0;
        Color::from_be_bytes([l, l, l, 0xFF])
    }
}

/// Convert the Luma with Alpha type into an 32 bits integer
impl ToPix for LumaA<u8> {
    #[inline]
    fn to_pix(self) -> Color {
        let [l, a] = self.0;
        if a > 0x7F {
            Color::from_be_bytes([l, l, l, 0xFF])
        } else {
            0
        }
    }
}

/// Convert the RGB type into an 32 bits integer
impl ToPix for Rgb<u8> {
    #[inline]
    fn to_pix(self) -> Color {
        let [r, g, b] = self.0;
        Color::from_be_bytes([r, g, b, 0xFF])
    }
}

/// Convert the RGBA type into an 32 bits integer
impl ToPix for Rgba<u8> {
    #[inline]
    fn to_pix(self) -> Color {
        let [r, g, b, a] = self.0;
        if a > 0x7F {
            Color::from_be_bytes([r, g, b, 0xFF])
        } else {
            0
        }
    }
}

/// Convert the Luma type into an 32 bits integer
impl ToPix for Luma<u16> {
    #[inline]
    fn to_pix(self) -> Color {
        let l = (self.0[0] >> 8) as u8;
        Color::from_be_bytes([l, l, l, 0xFF])
    }
}

/// Convert the Luma with Alpha type into an 32 bits integer
impl ToPix for LumaA<u16> {
    #[inline]
    fn to_pix(self) -> Color {
        let [l, a] = self.0;
        if a > 0x7FFF {
            let l = (l >> 8) as u8;
            Color::from_be_bytes([l, l, l, 0xFF])
        } else {
            0
        }
    }
}

/// Convert the RGB type into an 32 bits integer
impl ToPix for Rgb<u16> {
    #[inline]
    fn to_pix(self) -> Color {
        let [r, g, b] = self.0;
        let r = (r >> 8) as u8;
        let g = (g >> 8) as u8;
        let b = (b >> 8) as u8;
        Color::from_be_bytes([r, g, b, 0xFF])
    }
}

/// Convert the RGBA type into an 32 bits integer
impl ToPix for Rgba<u16> {
    #[inline]
    fn to_pix(self) -> Color {
        let [r, g, b, a] = self.0;
        if a > 0x7FFF {
            let r = (r >> 8) as u8;
            let g = (g >> 8) as u8;
            let b = (b >> 8) as u8;
            Color::from_be_bytes([r, g, b, 0xFF])
        } else {
            0
        }
    }
}

/// Convert the RGB type into an 32 bits integer
impl ToPix for Rgb<f32> {
    #[inline]
    fn to_pix(self) -> Color {
        const F: f32 = u8::MAX as f32;
        let [r, g, b] = self.0;
        let r = (r * F) as u8;
        let g = (g * F) as u8;
        let b = (b * F) as u8;
        Color::from_be_bytes([r, g, b, 0xFF])
    }
}

/// Convert the RGBA type into an 32 bits integer
impl ToPix for Rgba<f32> {
    #[inline]
    fn to_pix(self) -> Color {
        const F: f32 = u8::MAX as f32;
        let [r, g, b, a] = self.0;
        if a >= 0.5 {
            let r = (r * F) as u8;
            let g = (g * F) as u8;
            let b = (b * F) as u8;
            Color::from_be_bytes([r, g, b, 0xFF])
        } else {
            0
        }
    }
}
