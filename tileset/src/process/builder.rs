//! Read the input data and generate tiles.

use crate::{
    data::{
        coords::Dimensions,
        flip::Flip,
        mapping::CharacterMapping,
        palette::PaletteSet,
        tilemap::{TileData, TileMap},
        tileset::{Tile, TileSet},
    },
    input_stack::{InputConfig, InputImage, InputStack},
    output_stack::{OutError, OutputEntry, OutputImage, OutputStack, OutputStackError},
    process::encode::encode_tiles,
};
use core::ops::Deref;
use image::{EncodableLayout, GenericImageView, ImageBuffer, Pixel};
use itertools::Itertools;
use ndarray::{Array2, Ix2};
use std::collections::{BTreeMap, HashMap};

/// Process the input stack and generate an output stack
pub fn process_stack(input: &InputStack) -> Result<OutputStack, OutputStackError> {
    // Create a builder for the tilemap
    let mut builder = Builder::new(input.config);

    // Store the index maps if some are generated
    let mut out_entries = Vec::with_capacity(input.stack.len());

    // Store errors encountered while processing the stack
    let mut errors = BTreeMap::new();

    // Process each element in the input stack
    for entry in input.stack.iter() {
        match &entry.image {
            // Input is a common static image
            InputImage::Static(image) => match builder.process(image, &entry.palette) {
                Ok(tile_map) => {
                    // Encode the tiles for the target system
                    let out_map = encode_tiles(&input.profile, &tile_map);

                    out_entries.push(OutputEntry {
                        name: entry.name.clone(),
                        image: OutputImage::Static(out_map),
                        output_json: entry.output_json,
                        template: entry.template.clone(),
                    });
                }
                Err(err) => {
                    errors.insert(entry.path.clone(), err);
                }
            },
            // Input is a character set (or similar)
            InputImage::FixedPosition { image, mapping } => {
                match builder.process_fixed(image, &entry.palette, mapping) {
                    Ok(()) => {}
                    Err(err) => {
                        errors.insert(entry.path.clone(), err);
                    }
                }
            }
            InputImage::Aseprite(aseprite) => todo!("Aseprite files not yet supported"),
            InputImage::TiledTileset(tileset) => todo!("Tiled files not yet supported"),
            InputImage::TiledMap(map) => todo!("Tiled files not yet supported"),
        }
    }

    // Check if there are any errors and return the appropriate result
    if errors.is_empty() {
        Ok(OutputStack {
            tileset: builder.complete(),
            entries: out_entries,
        })
    } else {
        Err(OutputStackError { errors })
    }
}

// MARK: Builder

/// Builder that will process multiple input images to compose a tileset
#[derive(Debug)]
struct Builder {
    /// Configuration of this builder
    config: InputConfig,

    /// Empty tile to complete the tileset
    empty_tile: Tile,

    /// For a given index, keep track of the tile associated with it
    index_to_tile: HashMap<usize, Tile>,

    /// For a given tile, keep track of the associated index
    tile_to_index: HashMap<Tile, (usize, Flip)>,

    /// Keep track of the slots that are still vacant
    vacancy: Vec<bool>,
}

impl Builder {
    /// Create a builder with the provided config
    #[inline]
    fn new(config: InputConfig) -> Self {
        let capacity = config.tile_count;
        let mut vacancy = Vec::with_capacity(capacity);
        vacancy.resize(capacity, true);
        Self {
            config,
            empty_tile: Tile::new_empty(config.tile_width, config.tile_height),
            index_to_tile: HashMap::with_capacity(capacity),
            tile_to_index: HashMap::with_capacity(capacity),
            vacancy,
        }
    }

    /// Store a given tile at the specified index in the tileset
    /// This method assume the tile is in its default orientation
    #[inline]
    fn set_tile(&mut self, index: usize, tile: Tile) {
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
    fn get_vacant_slots(&self) -> Vec<usize> {
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
    fn complete(&self) -> TileSet {
        let count = self.config.tile_count;

        // Fill the final tileset
        let mut tileset = Vec::with_capacity(count);
        for index in 0..count {
            let tile = self.index_to_tile.get(&index).unwrap_or(&self.empty_tile);
            tileset.push(tile.clone());
        }

        TileSet::new(tileset)
    }

    /// Process the given images with the associated palette
    fn process<P, Q>(
        &mut self,
        img: &ImageBuffer<P, Q>,
        pal: &PaletteSet<P>,
    ) -> Result<TileMap, Vec<OutError>>
    where
        P: 'static + Pixel + PartialEq,
        [P::Subpixel]: EncodableLayout,
        Q: 'static + Deref<Target = [P::Subpixel]>,
    {
        // Get the dimensions of the input images in tiles.
        let tile_size = self.config.tile_size();
        let dims = Dimensions::from_img(img.dimensions(), tile_size);

        // Create a container to store index data
        let mut tile_map = Array2::<TileData>::default(Ix2::from(dims));

        // Push errors into this list
        let mut errors = Vec::<OutError>::new();

        // Iterate over each tile of the input image
        for index in 0..dims.count() {
            // Define the limits of the tile in pixels
            let coords = dims.to_coords(index);
            let ix2 = Ix2::from(coords);

            // Extract a sub part of the image
            let [px0, py0, px1, py1] = coords.bounds(tile_size);
            let sub_img = img.view(px0, py0, px1, py1);

            // Try to convert the sub portion of the image into a tile
            let res = pal.identify_tile(&sub_img.to_image());
            match res {
                Ok((palette_index, tile)) => {
                    // We identified a tile with the corresponding palette.
                    // Now we need to check if said tile already exists in the set.
                    if let Some(&(tile_index, flip)) = self.tile_to_index.get(&tile) {
                        // The tile already exists in the set,
                        // store the corresponding index in the index map.
                        tile_map[ix2] = TileData {
                            tile_index,
                            palette_index,
                            flip,
                        };
                    } else if let Some((tile_index, _)) =
                        self.vacancy.iter().find_position(|&&vacant| vacant)
                    {
                        // If we cannot find a matching tile in the set,
                        // add the new tile to the set at the first available slot.
                        self.set_tile(tile_index, tile);
                        tile_map[ix2] = TileData {
                            tile_index,
                            palette_index,
                            flip: Flip::None,
                        };
                    } else {
                        // We tried to store the new tile in the set, but there are no more room.
                        errors.push(OutError::DistinctOverflow(coords));
                    }
                }
                Err(_) => {
                    errors.push(OutError::NoPaletteMatch(coords));
                }
            }
        }

        // If we encountered errors, return them
        if errors.is_empty() {
            Ok(TileMap::new(tile_map))
        } else {
            Err(errors)
        }
    }

    /// Process the given images with the associated palette while enforcing
    /// a target position for each of the tiles.
    fn process_fixed<P, Q>(
        &mut self,
        img: &ImageBuffer<P, Q>,
        pal: &PaletteSet<P>,
        map: &CharacterMapping,
    ) -> Result<(), Vec<OutError>>
    where
        P: 'static + Pixel + PartialEq,
        [P::Subpixel]: EncodableLayout,
        Q: 'static + Deref<Target = [P::Subpixel]>,
    {
        // Get the dimensions of the input images in tiles.
        let tile_size = self.config.tile_size();
        let dims = Dimensions::from_img(img.dimensions(), tile_size);

        // Push errors into this list
        let mut errors = Vec::<OutError>::new();

        // Iterate over the positions provided
        for (&coords, &tile_idx) in map.0.iter() {
            // Check that the requested position is valid
            // and that the target index is still free.
            if !self.vacancy[tile_idx] {
                errors.push(OutError::InvalidIndex(tile_idx));
                continue;
            }

            if !dims.contains(coords) {
                errors.push(OutError::OutOfBound(coords));
                continue;
            }

            // Extract a sub part of the image
            let [px0, py0, px1, py1] = coords.bounds(tile_size);
            let sub_img = img.view(px0, py0, px1, py1);

            // Try to convert the sub portion of the image into a tile
            let res = pal.identify_tile(&sub_img.to_image());
            match res {
                Ok((_, tile)) => {
                    // We identified the tile with it's corresponding palette.
                    // Now we can store the tile at the requested index.
                    self.set_tile(tile_idx, tile);
                }
                Err(_) => {
                    errors.push(OutError::NoPaletteMatch(coords));
                }
            }
        }

        // If we encountered errors, return them
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
