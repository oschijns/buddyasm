use super::*;
use crate::tileset::TileSet;
use bitvec::{order::BitOrder, store::BitStore, vec::BitVec};

impl<T, O> SerialInner<T, O> for SerialMono
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

impl<T, O> SerialInner<T, O> for SerialLinear
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

impl<T, O> SerialInner<T, O> for SerialSplit
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

impl<T, O> SerialInner<T, O> for SerialRowIntertwine
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
