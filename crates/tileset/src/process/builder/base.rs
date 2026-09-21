use super::*;
use crate::{
    data::{
        coords::{Coords, ImgTileDim},
        flip::Flip,
        mapping::CharacterMapping,
        palette::Palette,
        tilemap::{TileData, TileMap},
    },
    output_stack::{TileError, TilesError},
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
    ) -> Result<TileMap, TilesError> {
        // Get the dimensions of the input images in tiles.
        let tile_size = self.config.tile_size;
        let dims = ImgTileDim::from_img(img.dimensions(), tile_size);

        // Create a container to store index data
        let mut tile_map = Array2::<TileData>::default(dims.ndarray_dim());

        // Push errors into this list
        let mut errors = Vec::<(Coords, TileError)>::new();

        // Iterate over each tile of the input image
        for index in 0..dims.tiles_count() {
            match self.process_one_tile(img, pal, dims, index) {
                Ok(tile_data) => {
                    let ix2 = dims.index_to_ix2(index);
                    tile_map[ix2] = tile_data;
                }
                Err(tile_error) => {
                    let coords = dims.index_to_coords_16(index);
                    errors.push((coords, tile_error));
                }
            }
        }

        // If we encountered errors, return them
        if errors.is_empty() {
            Ok(TileMap::new(tile_map))
        } else {
            Err(TilesError(errors))
        }
    }

    /// Process one tile and identify its index, palette index, flipping attributes, etc..
    fn process_one_tile(
        &mut self,
        img: &RgbaImage,
        pal: &Palette,
        dims: ImgTileDim,
        index: usize,
    ) -> Result<TileData, TileError> {
        // Extract a sub part of the image
        let [px, py, sx, sy] = dims.index_to_img_view(index);
        let sub_img = img.view(px, py, sx, sy);

        // Try to convert the sub portion of the image into a tile
        let (palette_index, tile) = pal.identify_tile(&sub_img.to_image())?;

        // We identified a tile with the corresponding palette.
        // Now we need to check if said tile already exists in the set.
        if let Some(&(tile_index, flip)) = self.tile_to_index.get(&tile) {
            // The tile already exists in the set,
            // store the corresponding index in the index map.
            Ok(TileData::new(tile_index, palette_index, flip))
        } else if let Some((tile_index, _)) = self.vacancy.iter().find_position(|&&vacant| vacant) {
            // If we cannot find a matching tile in the set,
            // add the new tile to the set at the first available slot.
            self.set_tile(tile_index, tile);
            Ok(TileData::new(tile_index, palette_index, Flip::None))
        } else {
            // We tried to store the new tile in the set, but there are no more room.
            Err(TileError::DistinctOverflow)
        }
    }

    /// Process the given images with the associated palette while enforcing
    /// a target position for each of the tiles.
    pub(super) fn process_fixed(
        &mut self,
        img: &RgbaImage,
        pal: &Palette,
        map: &CharacterMapping,
    ) -> Result<(), TilesError> {
        // Get the dimensions of the input images in tiles.
        let tile_size = self.config.tile_size;
        let dims = ImgTileDim::from_img(img.dimensions(), tile_size);

        // Push errors into this list
        let mut errors = Vec::<(Coords, TileError)>::new();

        // Iterate over the positions provided
        for (&coords, &index) in map.0.iter() {
            if let Err(tile_error) = self.process_one_fixed_tile(img, pal, dims, coords, index) {
                errors.push((coords, tile_error));
            }
        }

        // If we encountered errors, return them
        if errors.is_empty() {
            Ok(())
        } else {
            Err(TilesError(errors))
        }
    }

    /// Process one tile to be placed at a specific location
    fn process_one_fixed_tile(
        &mut self,
        img: &RgbaImage,
        pal: &Palette,
        dims: ImgTileDim,
        coords: Coords,
        index: usize,
    ) -> Result<(), TileError> {
        // The requested index must be valid
        let [ix, iy] = coords;
        if !dims.contains_16(ix, iy) {
            return Err(TileError::OutOfBound);
        }

        // Check that the requested position is valid
        // and that the target index is still free.
        if !self.vacancy[index] {
            return Err(TileError::InvalidIndex(index as u16));
        }

        // Extract a sub part of the image
        let [px, py, sx, sy] = dims.get_img_view(ix as usize, iy as usize);
        let sub_img = img.view(px, py, sx, sy);

        // Try to convert the sub portion of the image into a tile
        let (_, tile) = pal.identify_tile(&sub_img.to_image())?;

        // We identified the tile with it's corresponding palette.
        // Now we can store the tile at the requested index.
        self.set_tile(index, tile);
        Ok(())
    }
}
