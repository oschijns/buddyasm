use super::{Flip, IndexMap, IndexTile, PaletteSet, Pix, Tile, TileSet, to_index};
use core::{fmt, ops::Deref};
use image::{EncodableLayout, GenericImageView, ImageBuffer, Pixel};
use itertools::Itertools;
use ndarray::Array2;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("No matching palette for tile at {0}")]
    NoPaletteMatch(TilePos),

    #[error("Too many distinct tiles, tile at {0}")]
    TooManyTile(TilePos),
}

/// Define the position of a tile in an image in pixels coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilePos {
    /// X coordinate of the tile
    x: u32,

    /// Y coordinate of the tile
    y: u32,
}

/// Define a configuration to process the input images
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct BuilderConfig {
    /// Size of the tileset to produce
    tile_count: usize,

    /// With of a tile in pixels
    tile_width: u32,

    /// Height of a tile in pixels
    tile_height: u32,

    /// Specify wherever tiles can be flipped horizontally
    flip_horizontal: bool,

    /// Specify wherever tiles can be flipped vertically
    flip_vertical: bool,
}

/// Builder that will process multiple input images to compose a tileset
#[derive(Debug)]
pub struct TileMapBuilder {
    /// Configuration of this builder
    config: BuilderConfig,

    /// For a given index, keep track of the tile associated with it
    index_to_tile: HashMap<usize, Tile>,

    /// For a given tile, keep track of the associated index
    tile_to_index: HashMap<Tile, (usize, Flip)>,

    /// Keep track of the slots that are still vacant
    vacancy: Vec<bool>,
}

impl TileMapBuilder {
    /// Create a builder with the provided config
    #[inline]
    pub fn new(config: BuilderConfig) -> Self {
        let capacity = config.tile_count;
        let mut vacancy = Vec::with_capacity(capacity);
        vacancy.resize(capacity, true);
        Self {
            config,
            index_to_tile: HashMap::with_capacity(capacity),
            tile_to_index: HashMap::with_capacity(capacity),
            vacancy,
        }
    }

    /// Store a given tile at the specified index in the tileset
    /// This method assume the tile is in its default orientation
    #[inline]
    pub fn set_tile(&mut self, index: usize, tile: Tile) {
        let flip_h = self.config.flip_horizontal;
        let flip_v = self.config.flip_vertical;

        // Associate the index with the provided tile
        self.index_to_tile.insert(index, tile.clone());

        // Make it a bidirectional relationship
        self.tile_to_index.insert(tile.clone(), (index, Flip::None));
        if flip_h {
            self.tile_to_index
                .insert(tile.flip_horizontal(), (index, Flip::Horizontal));
        }
        if flip_v {
            self.tile_to_index
                .insert(tile.flip_vertical(), (index, Flip::Vertical));
        }
        if flip_h && flip_v {
            self.tile_to_index
                .insert(tile.flip_both(), (index, Flip::Both));
        }

        // The slot is no longer vacant
        self.vacancy[index] = false;
    }

    /// Get the list of vacant slots in the tileset
    pub fn get_vacant_slots(&self) -> Vec<usize> {
        let count = self.config.tile_count;

        // Check that all the tiles have been set
        let mut undef = Vec::with_capacity(count);
        for (index, &vacant) in self.vacancy.iter().enumerate() {
            if vacant {
                undef.push(index);
            }
        }
        undef
    }

    /// Complete the tileset to be usable
    pub fn complete(&self) -> TileSet {
        let empty = Tile::default();
        let count = self.config.tile_count;

        // Fill the final tileset
        let mut tileset = Vec::with_capacity(count);
        for index in 0..count {
            let tile = self.index_to_tile.get(&index).unwrap_or(&empty);
            tileset.push(tile.clone());
        }
        TileSet::new(tileset)
    }

    /// Process the given images with the associated palette
    pub fn process<P, Q>(
        &mut self,
        img: &ImageBuffer<P, Q>,
        pal: &PaletteSet<P>,
    ) -> Result<IndexMap, Vec<Error>>
    where
        P: 'static + Pixel + PartialEq,
        [P::Subpixel]: EncodableLayout,
        Q: 'static + Deref<Target = [P::Subpixel]>,
    {
        let cfg = self.config;

        // Figure out the dimensions of the input image
        let (img_w, img_h) = img.dimensions();

        // Define the dimensions of the input images in terms of tiles
        let idx_w = img_w / cfg.tile_width;
        let idx_h = img_h / cfg.tile_height;

        // Create a container to store index data
        let mut index_map = Array2::<IndexTile>::default(to_index(idx_w, idx_h));

        // Push errors into this list
        let mut errors = Vec::<Error>::new();

        // Iterate over each tile of the input image
        for (tx, ty) in (0..idx_w).cartesian_product(0..idx_h) {
            // Define the limits of the tile in pixels
            let pos = TilePos::new(tx * cfg.tile_width, ty * cfg.tile_height);
            let sub_img = img.view(
                pos.x,
                pos.y,
                pos.x + cfg.tile_width,
                pos.y + cfg.tile_height,
            );

            // Try to convert the sub portion of the image into a tile
            let res = pal.identify_tile(&sub_img.to_image());
            match res {
                Ok((pal_idx, tile)) => {
                    // We identified a tile with the corresponding palette.
                    // Now we need to check if said tile already exists in the set.
                    if let Some((tile_idx, flip)) = self.tile_to_index.get(&tile) {
                        // The tile already exists in the set,
                        // store the corresponding index in the index map.
                        index_map[to_index(tx, ty)] = IndexTile::new(*tile_idx, pal_idx, *flip);
                    } else if let Some((tile_idx, _)) =
                        self.vacancy.iter().find_position(|&&vacant| vacant)
                    {
                        // If we cannot find a matching tile in the set,
                        // add the new tile to the set at the first available slot.
                        self.set_tile(tile_idx, tile);
                        index_map[to_index(tx, ty)] = IndexTile::new(tile_idx, pal_idx, Flip::None);
                    } else {
                        // We tried to store the new tile in the set, but there are no more room.
                        errors.push(Error::TooManyTile(pos));
                    }
                }
                Err(_) => {
                    errors.push(Error::NoPaletteMatch(pos));
                }
            }
        }

        // If we encountered errors, return them
        if errors.is_empty() {
            Ok(IndexMap::new(index_map))
        } else {
            Err(errors)
        }
    }
}

impl<P> PaletteSet<P>
where
    P: Pixel,
{
    /// Check the content of the provided sub image to try to deduce a palette
    /// index and an encoding of the tile. If no palette defined in this set
    /// matches the provided image, return an error.
    pub fn identify_tile<Q>(&self, img: &ImageBuffer<P, Q>) -> Result<(usize, Tile), ()>
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
        Err(())
    }
}

impl TilePos {
    /// Create position for the tile
    #[inline]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for TilePos {
    /// Print the tile coordinates
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) pix²", self.x, self.y)
    }
}
