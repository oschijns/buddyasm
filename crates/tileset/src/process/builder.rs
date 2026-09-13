//! Read the input data and generate tiles.

/// Handle simple static images
pub mod base;

/// Handle animated sprites
pub mod animation;

use crate::{
    data::{
        flip::Flip,
        tileset::{Tile, TileSet},
    },
    input_stack::{InputConfig, InputEntry, InputImage, InputStack},
    output_stack::{OutError, OutputEntry, OutputImage, OutputStack, OutputStackError},
    process::encode::encode_tiles,
    profile::Profile,
};
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
        match builder.process_entry(&input.profile, entry) {
            Ok(Some(out_entry)) => {
                out_entries.push(out_entry);
            }
            Ok(None) => { /* do nothing */ }
            Err(err) => {
                errors.insert(entry.path.clone(), err);
            }
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
            empty_tile: Tile::new_empty(config.tile_size),
            index_to_tile: HashMap::with_capacity(capacity),
            tile_to_index: HashMap::with_capacity(capacity),
            vacancy,
        }
    }

    /// Store a given tile at the specified index in the tileset
    /// This method assume the tile is in its default orientation
    #[inline]
    fn set_tile(&mut self, index: usize, tile: Tile) {
        // Associate the index with the provided tile
        self.index_to_tile.insert(index, tile.clone());

        // Make it a bidirectional relationship
        self.tile_to_index.insert(tile.clone(), (index, Flip::None));

        #[cfg_attr(cfg, rustfmt::skip)]
        match self.config.flip {
            Flip::None       => { /* nothing to do */ }
            Flip::Horizontal => { self.tile_to_index.insert(tile.flip_horizontal(), (index, Flip::Horizontal)); }
            Flip::Vertical   => { self.tile_to_index.insert(tile.flip_vertical  (), (index, Flip::Vertical  )); }
            Flip::Both       => {
                // Flip the tile along both axes
                self.tile_to_index.insert(tile.flip_horizontal(), (index, Flip::Horizontal));
                self.tile_to_index.insert(tile.flip_vertical  (), (index, Flip::Vertical  ));
                self.tile_to_index.insert(tile.flip_both      (), (index, Flip::Both      ));
            }
        }

        // The slot is no longer vacant
        self.vacancy[index] = false;
    }

    /// Get the list of vacant slots in the tileset
    #[allow(unused)]
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

    /// Process one entry of the input stack and generate an output entry
    fn process_entry(
        &mut self,
        profile: &Profile,
        entry: &InputEntry,
    ) -> Result<Option<OutputEntry>, Vec<OutError>> {
        match &entry.image {
            // Input is a common static image
            InputImage::Static(image) => {
                let tile_map = self.process(image, &entry.palette)?;
                // Encode the tiles for the target system
                let out_map = encode_tiles(profile, &tile_map);
                Ok(Some(OutputEntry {
                    name: entry.name.clone(),
                    image: OutputImage::Static(out_map),
                    output_json: entry.output_json,
                    template: entry.template.clone(),
                }))
            }
            // Input is a character set (or similar)
            InputImage::FixedPosition { image, mapping } => {
                let _ = self.process_fixed(image, &entry.palette, mapping)?;
                Ok(None)
            }
            InputImage::Aseprite(aseprite) => {
                /*
                // Collect animations
                let mut animations = Vec::with_capacity(aseprite.num_frames() as usize);

                // TODO
                // read animations metadata to associate actual animations with
                // frames that have been processed.

                // Iterate over all the frames defined in the Aseprite file
                for i in 0..aseprite.num_frames() {
                    let image = aseprite.frame(i).image();
                    match builder.process(&image, &entry.palette) {
                        Ok(tile_map) => {
                            // Encode the tiles for the target system
                            let out_map = encode_tiles(&input.profile, &tile_map);
                            animations.push(out_map);
                        }
                        Err(err) => {
                            errors.insert(entry.path.clone(), err);
                            continue;
                        }
                    }
                }

                // Encode the tiles for the target system
                out_entries.push(OutputEntry {
                    name: entry.name.clone(),
                    image: OutputImage::Animated(animations),
                    output_json: entry.output_json,
                    template: entry.template.clone(),
                });
                */
                todo!("Aseprite not yet supported")
            }
            InputImage::TiledTileset(tileset) => todo!("Tiled files not yet supported"),
            InputImage::TiledMap(map) => todo!("Tiled files not yet supported"),
        }
    }
}

/// Check if a tile only contains zeroes
#[inline]
pub fn is_empty_tile(tile: &Tile) -> bool {
    tile.0.as_ref().iter().all(|&x| x == 0)
}
