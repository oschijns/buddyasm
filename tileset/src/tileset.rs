use image::{EncodableLayout, GenericImage, ImageBuffer, Pixel, SubImage};
use ndarray::{Array2, Dim, Ix, Ix2};
use std::{ops::Deref, usize};

/// Define a pixel in a tile
pub type Pix = u8;

/// Tile to serialize in a given binary layout
pub struct Tile(Array2<Pix>);

/// Tileset containing tiles
pub struct TileSet(Vec<Tile>);

/// Set of palettes to look for in an input image
pub struct Palettes<C>(Array2<C>);

impl<P> Palettes<P>
where
    P: Pixel,
{
    /// Check the content of the provided sub image to try to deduce a palette
    /// index and an encoding of the tile. If no palette defined in this set
    /// matches the provided image, return an error.
    pub fn identify<Q>(&self, img: &ImageBuffer<P, Q>) -> Result<(usize, Tile), ()>
    where
        P: PartialEq,
        [P::Subpixel]: EncodableLayout,
        Q: Deref<Target = [P::Subpixel]>,
    {
        // Figure out the dimensions of the input image
        let (w, h) = img.dimensions();

        // Create a tile to store the result
        let mut tile = Tile(Array2::zeros(to_index(w, h)));

        // Try each palette successively
        'pal: for (i, palette) in self.0.columns().into_iter().enumerate() {
            // iterate over each pixel of the input image
            'pix: for (x, y, pixel) in img.enumerate_pixels() {
                // Check if the pixel is part of the palette selected
                for (j, color) in palette.iter().enumerate() {
                    // Pixel of the image matches color from the selected palette
                    if *pixel == *color {
                        // Store the corresponding index in the tile we are making
                        tile.0[to_index(x, y)] = j as Pix;

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
            return Ok((i, tile));
        }

        // We've look into each color of the palette selected but could
        // not find a match. We'll try again with the next palette.

        Err(())
    }
}

#[inline]
fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}

// Define a chunk of an image
// pub struct ImageChunk<T>(Array2<Rgba<T>>);

/*
impl<T, const S: usize> Palette<T, S> {
    /// Check if the provided color is part of the palette
    pub fn contains(&self, color: Rgba<T>) -> bool
    where
        T: PartialEq,
    {
        for c in &self.0 {
            if color == *c {
                return true;
            }
        }
        false
    }
}
*/
