//! Define generic strategies for serializing a tile.

use crate::data::tile::TileSet;
use bitvec::{order::BitOrder, store::BitStore, vec::BitVec};

/// Define a serialization strategy for the tileset
pub(crate) trait SerialStrategy<T, O>
where
    T: BitStore,
    O: BitOrder,
{
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O>;
}

/// Serialize the tileset where one pixel corresponds to one bit
pub struct SerialMono;

/// Serialize the tileset linearly
pub struct SerialLinear {
    /// How many bits are used to define a pixel
    pub(crate) bits_per_pixel: usize,
}

/// Serialize the tileset as distinct bitplanes
pub struct SerialSplit {
    /// How many bits are used to define a pixel
    pub(crate) bits_per_pixel: usize,
}

/// Serialize the tileset by intertwining rows by rows
pub struct SerialRowIntertwine {
    /// How many bits are used to define a pixel
    pub(crate) bits_per_pixel: usize,

    /// Define how many rows are intertwined
    pub(crate) intertwined_rows: usize,
}

impl SerialLinear {
    /// Define linear layout serialization
    #[inline]
    pub fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
    }
}

impl SerialSplit {
    /// Define splitted bitplane layout serialization
    #[inline]
    pub fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
    }
}

impl SerialRowIntertwine {
    /// Define intertwined rows layout serialization
    #[inline]
    pub fn new(bits_per_pixel: usize, intertwined_rows: usize) -> Self {
        Self {
            bits_per_pixel,
            intertwined_rows,
        }
    }
}

impl<T, O> SerialStrategy<T, O> for SerialMono
where
    T: BitStore,
    O: BitOrder,
{
    /// Serialize the tileset where one pixel corresponds to one bit
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O> {
        // The serialization to produce
        let mut serial = BitVec::<T, O>::with_capacity(tileset.pixel_count());

        // iterate over each tile
        for tile in tileset.0.iter() {
            // iterate over each pixel of the tile
            for &pix in tile.0.iter() {
                // Simply store whenever the pixel is OFF or ON
                serial.push(pix != 0);
            }
        }

        serial
    }
}

impl<T, O> SerialStrategy<T, O> for SerialLinear
where
    T: BitStore,
    O: BitOrder,
{
    /// Serialize the tileset where the bits of a pixel are stored linearly
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O> {
        // The serialization to produce
        let mut serial = BitVec::<T, O>::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

        // iterate over each tile
        for tile in tileset.0.iter() {
            // iterate over each pixel of the tile
            for &pix in tile.0.iter() {
                // for each bit composing the pixel store it
                for bit in 0..self.bits_per_pixel {
                    serial.push(pix & (1 << bit) != 0);
                }
            }
        }

        serial
    }
}

impl<T, O> SerialStrategy<T, O> for SerialSplit
where
    T: BitStore,
    O: BitOrder,
{
    /// Serialize the tileset as multiple bitplanes
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O> {
        // The serialization to produce
        let mut serial = BitVec::<T, O>::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

        // for each bitplane to serialize
        for plane in 0..self.bits_per_pixel {
            let mask = 1 << plane;

            // iterate over each tile
            for tile in tileset.0.iter() {
                // iterate over each pixel of the tile
                for &pix in tile.0.iter() {
                    // Store the bit of the bitplane
                    serial.push(pix & mask != 0);
                }
            }
        }

        serial
    }
}

impl<T, O> SerialStrategy<T, O> for SerialRowIntertwine
where
    T: BitStore,
    O: BitOrder,
{
    /// Serialize the tileset as multiple bitplanes
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O> {
        // The serialization to produce
        let mut serial = BitVec::<T, O>::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

        // for each bitplane to serialize
        let planes = self.bits_per_pixel / self.intertwined_rows;
        for plane in 0..planes {
            // Index of the first bitplane to serialize
            let bitplane = plane * self.intertwined_rows;

            // iterate over each tile
            for tile in tileset.0.iter() {
                // iterate over each row of the tile
                for row in tile.0.rows() {
                    // intertwine the bits of the row
                    for inter in 0..self.intertwined_rows {
                        // Pick the bit to serialize
                        let mask = 1 << (bitplane + inter);

                        // serialize one bit of the pixel in this row
                        for pix in row {
                            serial.push(pix & mask != 0);
                        }
                    }
                }
            }
        }

        serial
    }
}
