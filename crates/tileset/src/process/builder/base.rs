use super::*;
use crate::{
    data::{
        coords::ImgTileDim,
        flip::Flip,
        mapping::CharacterMapping,
        palette::Palette,
        tilemap::{TileData, TileMap},
    },
    output_stack::OutError,
};
use image::{GenericImageView, RgbaImage};
use itertools::Itertools;
use ndarray::Array2;

impl Builder {
    /// Process the given images with the associated palette
    pub(super) fn process(
        &mut self,
        img: &RgbaImage,
        pal: &Palette,
    ) -> Result<TileMap, Vec<OutError>> {
        // Get the dimensions of the input images in tiles.
        let tile_size = self.config.tile_size;
        let dims = ImgTileDim::from_img(img.dimensions(), tile_size);

        // Create a container to store index data
        let mut tile_map = Array2::<TileData>::default(dims.ndarray_dim());

        // Push errors into this list
        let mut errors = Vec::<OutError>::new();

        // Iterate over each tile of the input image
        for index in 0..dims.tiles_count() {
            // Define the limits of the tile in pixels
            let coords = dims.index_to_coords_16(index);
            let ix2 = dims.index_to_ix2(index);

            // Extract a sub part of the image
            let [px, py, sx, sy] = dims.index_to_img_view(index);
            let sub_img = img.view(px, py, sx, sy);

            // Try to convert the sub portion of the image into a tile
            let res = pal.identify_tile(&sub_img.to_image());
            match res {
                Ok((palette_index, empty_tile, tile)) => {
                    // We identified a tile with the corresponding palette.
                    // Now we need to check if said tile already exists in the set.
                    if let Some(&(tile_index, flip)) = self.tile_to_index.get(&tile) {
                        // The tile already exists in the set,
                        // store the corresponding index in the index map.
                        tile_map[ix2] =
                            TileData::new(tile_index, palette_index, coords, flip, empty_tile);
                    } else if let Some((tile_index, _)) =
                        self.vacancy.iter().find_position(|&&vacant| vacant)
                    {
                        // If we cannot find a matching tile in the set,
                        // add the new tile to the set at the first available slot.
                        self.set_tile(tile_index, tile);
                        tile_map[ix2] = TileData::new(
                            tile_index,
                            palette_index,
                            coords,
                            Flip::None,
                            empty_tile,
                        );
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
    pub(super) fn process_fixed(
        &mut self,
        img: &RgbaImage,
        pal: &Palette,
        map: &CharacterMapping,
    ) -> Result<(), Vec<OutError>> {
        // Get the dimensions of the input images in tiles.
        let tile_size = self.config.tile_size;
        let dims = ImgTileDim::from_img(img.dimensions(), tile_size);

        // Push errors into this list
        let mut errors = Vec::<OutError>::new();

        // Iterate over the positions provided
        for (&[ix, iy], &tile_idx) in map.0.iter() {
            // Check that the requested position is valid
            // and that the target index is still free.
            if !self.vacancy[tile_idx] {
                errors.push(OutError::InvalidIndex(tile_idx));
                continue;
            }

            if !dims.contains_16(ix, iy) {
                errors.push(OutError::OutOfBound([ix, iy]));
                continue;
            }

            // Extract a sub part of the image
            let [px, py, sx, sy] = dims.get_img_view(ix as usize, iy as usize);
            let sub_img = img.view(px, py, sx, sy);

            // Try to convert the sub portion of the image into a tile
            let res = pal.identify_tile(&sub_img.to_image());
            match res {
                Ok((_, _, tile)) => {
                    // We identified the tile with it's corresponding palette.
                    // Now we can store the tile at the requested index.
                    self.set_tile(tile_idx, tile);
                }
                Err(_) => {
                    errors.push(OutError::NoPaletteMatch([ix, iy]));
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
