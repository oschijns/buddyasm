//! Palette

use crate::data::{
    coords::TileSize,
    tileset::{Pix, Tile},
};
use image::{ImageResult, Pixel, Rgb, Rgba, RgbaImage};
use ndarray::{Array2, Ix, Ix2};
use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hash, Hasher},
    path::Path,
    sync::{Arc, Mutex, RwLock},
};

/// Set of palettes to look for in an input image
#[derive(Debug, Clone)]
pub struct Palette {
    /// Read-only part of the palette
    read: Arc<InnerRead>,

    /// Writable part of the palette
    write: Arc<RwLock<InnerWrite>>,
}

/// Storage for the processed palette.
#[derive(Debug)]
struct InnerRead {
    /// Set of unique colors extracted from the palette
    colorset: Vec<(ColorHash, [i32; 3])>,

    /// Palette definition where each RGBA color has been identified by a unique index.
    palette: Array2<u8>,
}

/// Storage for the processed palette.
#[derive(Debug)]
struct InnerWrite {
    /// Store already identified mapping between RGB colors and corresponding index.
    /// This map is initially filled with colors from the palette but may be completed
    /// overtime if we encounter colors that are close but not perfect match.
    lookup: HashMap<ColorHash, u8, BuildHasherDefault<ColorHasher>>,
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
#[error("No matching palette for the tile")]
pub enum PaletteError {
    #[error("No matching palette found")]
    NoMatch,

    #[error("Failed to lock the shared map for reading")]
    ReadLock,

    #[error("Failed to lock the shared map for writing")]
    WriteLock,
}

impl Palette {
    /// Load a palette from a file
    pub fn load_palette(path: &Path) -> ImageResult<Self> {
        let matrix = load_palette_from_image(path)?;
        let (read, write) = matrix_to_palette(matrix);
        Ok(Self {
            read: Arc::new(read),
            write: Arc::new(RwLock::new(write)),
        })
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

/// Discard any pixels with an alpha channel below
/// this threshold, treating them as transparent.
const ALPHA_THRESHOLD: u8 = 128;

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
fn matrix_to_palette(matrix: Array2<Rgba<u8>>) -> (InnerRead, InnerWrite) {
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
        .map(|rgb| (ColorHash::from(rgb), to_vec(rgb)))
        .collect::<Vec<_>>();

    // Initialize the lookup table and fill it with color indexes
    let mut lookup =
        HashMap::with_capacity_and_hasher(colorset.len(), BuildHasherDefault::<ColorHasher>::new());
    for (index, (hash, _)) in colorset.iter().enumerate() {
        lookup.insert(*hash, index as u8);
    }

    // Initialize the palette and identify the indexes of the colors
    let mut palette = Array2::zeros(matrix.dim());
    for (out, color) in palette.iter_mut().zip(matrix.iter()) {
        if color.alpha() >= ALPHA_THRESHOLD {
            let item_hash = ColorHash::from(color.to_rgb());

            // Find the index of the color
            if let Some(index) = colorset.iter().position(|(hash, _)| *hash == item_hash) {
                *out = index as u8;
            }
        } else {
            // Transparent pixels are assigned the maximum index to avoid conflicts with opaque colors
            *out = u8::MAX;
        }
    }

    (InnerRead { colorset, palette }, InnerWrite { lookup })
}

impl Palette {
    /// Given an input image, identify the indexes of the color of each pixel
    fn identify_color_indexes(
        &self,
        img: &RgbaImage,
        workbuffer: &mut WorkBuffer,
    ) -> Result<(), PaletteError> {
        // Assign color indexes to each pixel in the image
        for (out, color) in workbuffer.iter_mut().zip(img.pixels()) {
            if color.alpha() >= ALPHA_THRESHOLD {
                // Check if the color has already been identified before
                let rgb = color.to_rgb();
                let hash = ColorHash::from(rgb);

                // Lock the lookup map for reading
                let Ok(lookup) = self.write.read() else {
                    return Err(PaletteError::ReadLock);
                };

                // Check if an index has already be identified for this color
                if let Some(index) = lookup.lookup.get(&hash) {
                    *out = *index;
                } else {
                    // Drop the lock on the lookup map while we search
                    // for the index of the closest color.
                    drop(lookup);

                    // Otherwise look for an index based on the proximity
                    // to the colors in the initial colorset.
                    let p1 = to_vec(rgb);

                    // Find the index of the closest color in the color set.
                    let mut selected = (u32::MAX, u8::MAX);
                    for (i, &(_, p0)) in self.read.colorset.iter().enumerate() {
                        let sqr_dist = squared_distance(p0, p1);

                        // If the squared distance is smaller than
                        // the current minimum, update the selection.
                        if sqr_dist < selected.0 {
                            selected = (sqr_dist, i as u8);
                        }
                    }

                    // We found an index for the new color
                    *out = selected.1;

                    // Lock the lookup map in writing to insert the new entry
                    let Ok(mut lookup) = self.write.write() else {
                        return Err(PaletteError::WriteLock);
                    };
                    lookup.lookup.insert(hash, selected.1);
                }
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
        'palette: for (pal_index, palette) in self.read.palette.columns().into_iter().enumerate() {
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

/// Convert a RGB value into a array of signed integers
#[inline]
const fn to_vec(rgb: Rgb<u8>) -> [i32; 3] {
    let [r, g, b] = rgb.0;
    [r as i32, g as i32, b as i32]
}

/// Compute the squared distance between two points in 3D space
#[inline]
const fn squared_distance(p0: [i32; 3], p1: [i32; 3]) -> u32 {
    let [x0, y0, z0] = p0;
    let [x1, y1, z1] = p1;

    #[inline]
    const fn pow2(x: i32) -> u32 {
        (x * x) as u32
    }

    // 255^2 * 3 is below the maximum value of 2^32 so overflow is not possible
    pow2(x0 - x1) + pow2(y0 - y1) + pow2(z0 - z1)
}

/// Wrapper to quickly hash the RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorHash(u32);

/// Hasher for colors
#[derive(Debug, Default, Clone, Copy)]
struct ColorHasher(u32);

impl From<Rgb<u8>> for ColorHash {
    fn from(rgb: Rgb<u8>) -> Self {
        let [r, g, b] = rgb.0;
        Self(u32::from_ne_bytes([r, g, b, 0x00]))
    }
}

impl Hash for ColorHash {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}

impl Hasher for ColorHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0 = i;
    }

    /// Should not be used
    fn write(&mut self, _: &[u8]) {
        panic!("Invalid use")
    }
}
