use super::{Flip, IndexMap, IndexTile, PaletteSet, Pix, Tile, TileSet, to_index};
use image::{EncodableLayout, GenericImageView, ImageBuffer, Pixel};
use itertools::Itertools;
use ndarray::Array2;
use serde::Deserialize;
use std::{fmt, ops::Deref, usize};

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

    /// Tiles to generate
    tiles: Vec<Option<Tile>>,
}

impl TileMapBuilder {
    /// Create a builder with the provided config
    #[inline]
    pub fn new(config: BuilderConfig) -> Self {
        let mut tiles = Vec::with_capacity(config.tile_count);
        tiles.resize(config.tile_count, None);
        Self { config, tiles }
    }

    /// Store a given tile at the specified index in the tileset
    #[inline]
    pub fn set_tile(&mut self, index: usize, tile: Tile) {
        self.tiles[index] = Some(tile);
    }

    /// Complete the tileset to be usable
    pub fn complete(&self) -> Result<TileSet, Vec<usize>> {
        // Unwrap the options to get a filled tileset
        let mut tileset = Vec::with_capacity(self.config.tile_count);

        // Report missing tiles by index
        let mut undef = Vec::new();

        // Default empty tile to add to the tileset
        let empty = Tile::default();

        // Check that all the tiles have been set
        for (idx, tile) in self.tiles.iter().enumerate() {
            if let Some(tile) = tile {
                tileset.push(tile.clone());
            } else {
                tileset.push(empty.clone());
                undef.push(idx);
            }
        }

        // Check if
        if undef.is_empty() {
            Ok(TileSet::new(tileset))
        } else {
            Err(undef)
        }
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
        'tile: for (tx, ty) in (0..idx_w).cartesian_product(0..idx_h) {
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
                    for (tile_idx, other) in self.tiles.iter().enumerate() {
                        if let Some(other) = other
                            && let Some(flip) =
                                other.similarity(&tile, cfg.flip_horizontal, cfg.flip_vertical)
                        {
                            // The two tiles are actually the same.
                            // Store the corresponding index in the index map.
                            index_map[to_index(tx, ty)] = IndexTile::new(tile_idx, pal_idx, flip);
                            continue 'tile;
                        }
                    }

                    // We could not find a matching tile in the stack.
                    // Add the new tile to the set at the first available slot.
                    for (tile_idx, other) in self.tiles.iter_mut().enumerate() {
                        if other.is_none() {
                            // Store the tile at that location
                            *other = Some(tile);

                            // And store the corresponding index in the index map
                            index_map[to_index(tx, ty)] =
                                IndexTile::new(tile_idx, pal_idx, Flip::None);
                            continue 'tile;
                        }
                    }

                    // We tried to store the new tile in the set, but there are no more room.
                    errors.push(Error::TooManyTile(pos));
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
