//! Palette

use crate::{
    data::{
        coords::TileSize,
        tileset::{Pix, Tile},
    },
    manifest::{self, Manifest},
};
use buddyasm_common::{
    List, Str,
    color::{ColorError, rgba_from_hex},
    image::{
        ImageError, ImageResult, Pixel, Rgba, RgbaImage,
        imageops::{ColorMap, dither},
    },
    manifest::Manifest as _,
};
use buddyasm_common::{
    color_space::{ColorSpace, ColorSpaceBuilder, LockError},
    image,
};
use ndarray::{Array2, Ix, Ix2};
use std::{path::Path, sync::Arc};

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

    /// If dithering pass is needed, populate this vector with sub palettes.
    /// Otherwise it can remain empty.
    sub_palettes: Vec<SubPalette>,
}

/// Error encountered when trying to load a palette from the manifest file
#[derive(thiserror::Error, Debug)]
pub enum PaletteLoadError {
    /// No palette specified
    #[error("No palette specified")]
    Missing,

    /// Palette is empty
    #[error("Palette is empty")]
    Empty,

    /// Palette has a variable amout of color per rows
    #[error("Uneven palette at {0}")]
    Uneven(usize),

    /// Parsing error
    #[error("Failed to parse color value: {0}")]
    ColorParsing(#[from] ColorError),

    /// Failed loading a palette from a file
    #[error("Failed to load palette at path: {0}")]
    Image(#[from] ImageError),
}

/// Error encountered when trying to find a palette for a given tile
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteUseError {
    #[error("No matching palette found")]
    NoMatch,

    #[error("{0}")]
    Lock(#[from] LockError),
}

/// Work buffer for identifying
pub type WorkBuffer = Array2<u8>;

/// Allocate a work buffer to process tiles
#[inline]
pub fn make_workbuffer(tile_size: TileSize) -> WorkBuffer {
    Array2::zeros(tile_size.ndarray_dim())
}

impl Palette {
    /// Load from an entry in the manifest (either an image or a direct declaration)
    pub fn load_from_entry(
        manifest: &Manifest,
        palette: &manifest::Palette,
        alpha_threshold: u8,
        use_dithering: bool,
    ) -> Result<Self, PaletteLoadError> {
        let matrix = match palette {
            manifest::Palette::Path(path) => {
                let path = manifest.evaluate_path(path);
                load_matrix_from_image(&path)?
            }
            manifest::Palette::Direct(lists) => load_matrix_from_manifest(lists)?,
        };
        Ok(Self::from_matrix(matrix, alpha_threshold, use_dithering))
    }

    /// Load a palette from a file
    pub fn load_from_disk(
        path: &Path,
        alpha_threshold: u8,
        use_dithering: bool,
    ) -> Result<Self, PaletteLoadError> {
        let matrix = load_matrix_from_image(path)?;
        Ok(Self::from_matrix(matrix, alpha_threshold, use_dithering))
    }

    /// Load a palette from the manifest directly
    pub fn load_from_manifest(
        lists: &List<List<Str>>,
        alpha_threshold: u8,
        use_dithering: bool,
    ) -> Result<Self, PaletteLoadError> {
        let matrix = load_matrix_from_manifest(lists)?;
        Ok(Self::from_matrix(matrix, alpha_threshold, use_dithering))
    }

    /// Convert a 2D matrix of RGBA colors into a usable palette.
    pub fn from_matrix(matrix: Array2<Rgba<u8>>, alpha_threshold: u8, use_dithering: bool) -> Self {
        // Identify unique colors in the matrix
        let mut builder = ColorSpaceBuilder::with_capacity(matrix.len());
        builder.alpha_threshold = alpha_threshold;
        for color in matrix.iter() {
            builder.add_a(*color);
        }
        let partition = builder.finish();

        // If we apply a dithering pass, generate sub palettes
        let sub_palettes = if use_dithering {
            SubPalette::create_sub_palettes(&matrix, alpha_threshold)
        } else {
            Vec::new()
        };

        // Initialize the palette and identify the indexes of the colors
        let mut palette = Array2::zeros(matrix.dim());
        for (out, color) in palette.iter_mut().zip(matrix.iter()) {
            // Transparent pixels are assigned the maximum index to avoid conflicts with opaque colors
            if color.alpha() >= alpha_threshold
                && let Ok((index, _)) = partition.identify_color_index(color.to_rgb())
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
            sub_palettes,
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
    ) -> Result<(usize, Tile), PaletteUseError> {
        // Identify the indexes of each color in the input image.
        self.identify_color_indexes(img, workbuffer)?;

        // Should we apply dithering or not ?
        if self.sub_palettes.is_empty() {
            // Then find a matching palette for the tile being processed.
            let (index, tile) = self.find_matching_palette(workbuffer)?;
            Ok((index, Tile::new(tile)))
        } else {
            let index = self.find_best_matching_palette(workbuffer);
            let tile = self.apply_dithering(img, index);
            Ok((index, Tile::new(tile)))
        }
    }
}

/// Load a palette matrix from an image file on disk.
/// Returns a simple 2D matrix of RGBA colors.
fn load_matrix_from_image(path: &Path) -> ImageResult<Array2<Rgba<u8>>> {
    // load the image into a RGBA image
    let img = image::ImageReader::open(path)?.decode()?.into_rgba8();

    // Convert it into a 2D matrix
    let width = img.width() as usize;
    let height = img.height() as usize;
    let mut matrix = Array2::from_elem((width, height), Rgba([0, 0, 0, 0]));

    // Fill the matrix with data
    for (x, y, &pix) in img.enumerate_pixels() {
        matrix[to_index(x, y)] = pix;
    }

    // Return the palette
    Ok(matrix)
}

/// Load a palette matrix from an image file on disk.
/// Returns a simple 2D matrix of RGBA colors.
fn load_matrix_from_manifest(
    lists: &List<List<Str>>,
) -> Result<Array2<Rgba<u8>>, PaletteLoadError> {
    // Check list is not empty
    if lists.is_empty() {
        Err(PaletteLoadError::Empty)
    } else {
        // We first must check that the palette is even
        let expect = lists[0].len();
        for (index, list) in lists.iter().enumerate() {
            if list.len() != expect {
                return Err(PaletteLoadError::Uneven(index));
            }
        }

        // The palette is even, generate the matrix
        let mut matrix = Array2::from_elem((lists.len(), expect), Rgba([0, 0, 0, 0]));
        for (i, list) in lists.iter().enumerate() {
            for (j, val) in list.iter().enumerate() {
                matrix[Ix2(j, i)] = rgba_from_hex(val.as_ref())?;
            }
        }

        Ok(matrix)
    }
}

impl Palette {
    /// Given an input image, identify the indexes of the color of each pixel
    fn identify_color_indexes(
        &self,
        img: &RgbaImage,
        workbuffer: &mut WorkBuffer,
    ) -> Result<(), PaletteUseError> {
        // Assign color indexes to each pixel in the image
        for (out, pixel) in workbuffer.iter_mut().zip(img.pixels()) {
            if pixel.alpha() >= self.alpha_threshold {
                *out = self.partition.identify_color_index(pixel.to_rgb())?.0 as u8;
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
    ) -> Result<(usize, Array2<u8>), PaletteUseError> {
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

        Err(PaletteUseError::NoMatch)
    }

    /// Once each pixel has been remapped to its index,
    /// try to identify the best matching palette for the tile.
    /// This method is to be followed by applying dithering afterward.
    fn find_best_matching_palette(&self, workbuffer: &WorkBuffer) -> usize {
        // Palette index selected and the number of matching pixels found
        let mut selected = 0usize;
        let mut best_count = 0usize;

        // Try each palette successively
        for (pal_index, palette) in self.palette.columns().into_iter().enumerate() {
            let mut count = 0usize;

            // iterate over each pixel of the input image
            'pixel: for pixel_id in workbuffer.iter() {
                // Check if the pixel is part of the palette selected
                for color_id in palette.iter() {
                    // Pixel of the image matches color from the selected palette
                    if *pixel_id == *color_id {
                        count += 1;

                        // We can move on to the next pixel
                        continue 'pixel;
                    }
                }
            }

            // If we found a better matching palette, switch to it
            if count > best_count {
                selected = pal_index;
                best_count = count;
            }
        }
        selected
    }

    /// Given the initial image and the best matching palette,
    /// apply a dithering algorithm to output a tile.
    fn apply_dithering(&self, img: &RgbaImage, palette_index: usize) -> Array2<u8> {
        // Pick the color palette to use for indexing the pixels
        let palette = &self.sub_palettes[palette_index];

        // Apply the dithering to the image
        let mut img = img.clone();
        dither(&mut img, palette);

        // Allocate a new tile
        let (w, h) = img.dimensions();
        let mut tile = Array2::zeros((w as Ix, h as Ix));

        // iterate over each pixel of the input image
        for (out, pixel) in tile.iter_mut().zip(img.pixels()) {
            *out = palette.index_of(pixel) as u8;
        }
        tile
    }
}

/// Convert image coordinates into ndarray coordinates
#[inline]
const fn to_index(x: u32, y: u32) -> Ix2 {
    Ix2(x as Ix, y as Ix)
}

/// Sub palette for applying the color dithering
#[derive(Debug, Clone)]
struct SubPalette {
    /// Color space partitioning
    partition: ColorSpace,

    /// threshold for discarding colors based on their alpha channel
    pub alpha_threshold: u8,
}

impl SubPalette {
    /// process the matrix of color to generate the list of sub-palettes
    fn create_sub_palettes(matrix: &Array2<Rgba<u8>>, alpha_threshold: u8) -> Vec<Self> {
        // List of sub palettes to return
        let mut out = Vec::with_capacity(matrix.ncols());

        // Buffer to store colors and their assigned index
        let mut buffer = Vec::with_capacity(matrix.nrows());

        // For each palette defined, generate a color space for it
        for palette in matrix.columns() {
            buffer.clear();

            for (index, color) in palette.iter().enumerate() {
                buffer.push((color.to_rgb(), index));
            }

            // Generate the color space
            out.push(Self {
                partition: ColorSpace::load(&buffer),
                alpha_threshold,
            });
        }

        out
    }
}

impl ColorMap for SubPalette {
    type Color = Rgba<u8>;

    /// Returns the index of the closest match of `color` in the color map.
    fn index_of(&self, color: &Self::Color) -> usize {
        if color.alpha() >= self.alpha_threshold
            && let Ok((index, _)) = self.partition.identify_color_index(color.to_rgb())
        {
            index
        } else {
            u8::MAX as usize
        }
    }

    /// Maps `color` to the closest color in the color map.
    fn map_color(&self, color: &mut Self::Color) {
        if color.alpha() >= self.alpha_threshold
            && let Ok((_, rgb)) = self.partition.identify_color_index(color.to_rgb())
        {
            *color = rgb.to_rgba();
        } else {
            *color = Rgba([0, 0, 0, 0])
        }
    }

    /// Looks up color by index in the color map.
    fn lookup(&self, index: usize) -> Option<Self::Color> {
        self.partition.by_index(index).map(|rgb| rgb.to_rgba())
    }

    /// Enable lookup
    #[inline]
    fn has_lookup(&self) -> bool {
        true
    }
}
