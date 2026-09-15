use image::{DynamicImage, ImageBuffer, Luma, LumaA, Pixel, Rgb, Rgba};
use ndarray::{Array2, Ix, Ix2};
use std::ops::Deref;

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
        DynamicImage::ImageLuma16 (image) => todo!(),
        DynamicImage::ImageLumaA16(image) => todo!(),
        DynamicImage::ImageRgb16  (image) => todo!(),
        DynamicImage::ImageRgba16 (image) => todo!(),
        DynamicImage::ImageRgb32F (image) => todo!(),
        DynamicImage::ImageRgba32F(image) => todo!(),
        _ => todo!(),
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

/// Convert the RGBA type into an 32 bits integer
impl ToPix for Rgba<u8> {
    #[inline]
    fn to_pix(self) -> Color {
        Color::from_ne_bytes(self.0)
    }
}
/// Convert the RGB type into an 32 bits integer
impl ToPix for Rgb<u8> {
    #[inline]
    fn to_pix(self) -> Color {
        let [r, g, b] = self.0;
        Color::from_ne_bytes([r, g, b, 0xFF])
    }
}

/// Convert the Luma with Alpha type into an 32 bits integer
impl ToPix for LumaA<u8> {
    #[inline]
    fn to_pix(self) -> Color {
        let [l, a] = self.0;
        Color::from_ne_bytes([l, l, l, a])
    }
}

/// Convert the Luma type into an 32 bits integer
impl ToPix for Luma<u8> {
    #[inline]
    fn to_pix(self) -> Color {
        let [l] = self.0;
        Color::from_ne_bytes([l, l, l, 0xFF])
    }
}
