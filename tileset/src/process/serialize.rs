//! Define generic strategies for serializing a tile.

use crate::{data::tileset::TileSet, profile::*};
use bitvec::{
    order::{BitOrder, Lsb0, Msb0},
    store::BitStore,
    vec::BitVec,
};
use bytemuck::cast_slice;
use bytes::{BufMut, Bytes, BytesMut};

/// Serialize the generated tileset for the target system profile
pub trait SerializeTileSet {
    /// Serialize the generated tileset for the target system profile
    fn serialize(&self, tileset: &TileSet) -> Bytes;
}

// MARK: System

impl SerializeTileSet for Profile {
    #[rustfmt::skip]
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        match self {
            Self::Famicom     (serial) => serial.serialize(tileset),
            Self::SuperFamicom(serial) => serial.serialize(tileset),
            Self::GameBoy     (serial) => serial.serialize(tileset),
            Self::GameBoyColor(serial) => serial.serialize(tileset),
            Self::VirtualBoy  (serial) => serial.serialize(tileset),
            Self::PcEngine    (serial) => serial.serialize(tileset),
            Self::WonderSwan  (serial) => serial.serialize(tileset),
            Self::MasterSystem(serial) => serial.serialize(tileset),
            Self::MegaDrive   (serial) => serial.serialize(tileset),
            Self::NeoGeoPocket(serial) => serial.serialize(tileset),
            Self::NeoGeo      (serial) => serial.serialize(tileset),
        }
    }
}

impl SerializeTileSet for ProfileFamicom {
    /// Serialize the tileset in a Famicom compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = match self.bitplane {
            BitplaneFamicom::Bpp1 => SerialMono.serialize(tileset),
            BitplaneFamicom::Bpp2 => SerialTileSplit::new(2).serialize(tileset),
        };
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerializeTileSet for ProfileSuperFamicom {
    /// Serialize the tileset in a Super Famicom compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = match self.bitplane {
            BitplaneSuperFamicom::Bpp2 => SerialTileSplit::new(2).serialize(tileset),
            BitplaneSuperFamicom::Bpp4 => SerialInterleave::new(4).serialize(tileset),
            BitplaneSuperFamicom::Bpp8 => SerialInterleave::new(8).serialize(tileset),
            BitplaneSuperFamicom::Mode7 => SerialLinear::new(8).serialize(tileset),
        };
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerializeTileSet for ProfileGameBoy {
    /// Serialize the tileset in a GameBoy compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = SerialRowSplit::new(2).serialize(tileset);
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerializeTileSet for ProfileGameBoyColor {
    /// Serialize the tileset in a GameBoy compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = SerialRowSplit::new(2).serialize(tileset);
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerializeTileSet for ProfileVirtualBoy {
    /// Serialize the tileset in a VirtualBoy compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u16, Msb0> = SerialLinear::new(2).serialize(tileset);
        Bytes::copy_from_slice(cast_slice(bits.as_raw_slice()))
    }
}

impl SerializeTileSet for ProfilePcEngine {
    /// Serialize the tileset in a PC-Engine compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        match self.mode {
            BgFg::Bg(_) => {
                let bits: BitVec<u8, Lsb0> = SerialInterleave::new(4).serialize(tileset);
                Bytes::copy_from_slice(bits.as_raw_slice())
            }
            BgFg::Fg(_) => {
                let bits: BitVec<u16, Lsb0> = SerialTileSplit::new(4).serialize(tileset);
                Bytes::copy_from_slice(cast_slice(bits.as_raw_slice()))
            }
        }
    }
}

impl SerializeTileSet for ProfileWonderSwan {
    /// Serialize the tileset in a WonderSwan compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u32, Msb0> = SerialLinear::new(4).serialize(tileset);
        let buffer = Bytes::copy_from_slice(cast_slice(bits.as_raw_slice()));

        // Swap the nybbles of each byte
        let mut wbuffer = BytesMut::with_capacity(buffer.len());
        for &byte in buffer.iter() {
            // swap the two nybbles
            let wbyte = (byte >> 4) | (byte << 4);
            wbuffer.put_u8(wbyte);
        }

        wbuffer.freeze()
    }
}

impl SerializeTileSet for ProfileMasterSystem {
    /// Serialize the tileset in a MasterSystem compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Msb0> = SerialLinear::new(4).serialize(tileset);
        Bytes::copy_from_slice(cast_slice(bits.as_raw_slice()))
    }
}

impl SerializeTileSet for ProfileMegaDrive {
    /// Serialize the tileset in a MegaDrive compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u32, Msb0> = SerialLinear::new(4).serialize(tileset);
        let buffer = Bytes::copy_from_slice(cast_slice(bits.as_raw_slice()));

        // Swap the nybbles of each byte
        let mut wbuffer = BytesMut::with_capacity(buffer.len());
        for &byte in buffer.iter() {
            // swap the two nybbles
            let wbyte = (byte >> 4) | (byte << 4);
            wbuffer.put_u8(wbyte);
        }

        wbuffer.freeze()
    }
}

impl SerializeTileSet for ProfileNeoGeoPocket {
    /// Serialize the tileset in a NeoGeo Pocket compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u16, Lsb0> = SerialLinear::new(2).serialize(tileset);
        Bytes::copy_from_slice(cast_slice(bits.as_raw_slice()))
    }
}

impl SerializeTileSet for ProfileNeoGeo {
    /// Serialize the tileset in a NeoGeo compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        todo!()
    }
}

// MARK: Strategies

/// Define a serialization strategy for the tileset
trait SerialStrategy<T, O>
where
    T: BitStore,
    O: BitOrder,
{
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O>;
}

/// Serialize the tileset where one pixel corresponds to one bit
struct SerialMono;

/// Serialize the tileset linearly
struct SerialLinear {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

/// Serialize the tileset as distinct bitplanes
struct SerialPlaneSplit {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

/// Serialize the tileset by splitting each tile
struct SerialTileSplit {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

/// Serialize the tileset by splitting each row
struct SerialRowSplit {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

/// Serialize the tileset by serializing pairs of bit planes
struct SerialInterleave {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

impl SerialLinear {
    /// Define linear layout serialization
    #[inline]
    const fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
    }
}

impl SerialPlaneSplit {
    /// Define splitted bitplane layout serialization
    #[inline]
    const fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
    }
}

impl SerialTileSplit {
    /// Define splitted tiles layout serialization
    #[inline]
    const fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
    }
}

impl SerialRowSplit {
    /// Define splitted rows layout serialization
    #[inline]
    const fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
    }
}

impl SerialInterleave {
    /// Define interleave layout serialization
    #[inline]
    const fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
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

impl<T, O> SerialStrategy<T, O> for SerialPlaneSplit
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

impl<T, O> SerialStrategy<T, O> for SerialTileSplit
where
    T: BitStore,
    O: BitOrder,
{
    /// Serialize the tileset as multiple bitplanes
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O> {
        // The serialization to produce
        let mut serial = BitVec::<T, O>::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

        // iterate over each tile
        for tile in tileset.0.iter() {
            // for each bitplane to serialize
            for plane in 0..self.bits_per_pixel {
                let mask = 1 << plane;

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

impl<T, O> SerialStrategy<T, O> for SerialRowSplit
where
    T: BitStore,
    O: BitOrder,
{
    /// Serialize the tileset as multiple bitplanes
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O> {
        // The serialization to produce
        let mut serial = BitVec::<T, O>::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

        // iterate over each tile
        for tile in tileset.0.iter() {
            for row in tile.0.rows() {
                // for each bitplane to serialize
                for plane in 0..self.bits_per_pixel {
                    let mask = 1 << plane;

                    // iterate over each pixel of the tile
                    for &pix in row.iter() {
                        // Store the bit of the bitplane
                        serial.push(pix & mask != 0);
                    }
                }
            }
        }

        serial
    }
}

impl<T, O> SerialStrategy<T, O> for SerialInterleave
where
    T: BitStore,
    O: BitOrder,
{
    /// Serialize the tileset as multiple bitplanes
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O> {
        // The serialization to produce
        let mut serial = BitVec::<T, O>::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

        // iterate over each tile
        for tile in tileset.0.iter() {
            // for each bitplane to serialize
            for plane_pair in (0..self.bits_per_pixel).step_by(2) {
                for row in tile.0.rows() {
                    let mask0 = 0b01 << plane_pair;
                    let mask1 = 0b10 << plane_pair;

                    // iterate over each pixel of the tile
                    for &pix in row.iter() {
                        // Store the bit of the bitplane
                        serial.push(pix & mask0 != 0);
                    }

                    // iterate over each pixel of the tile
                    for &pix in row.iter() {
                        // Store the bit of the bitplane
                        serial.push(pix & mask1 != 0);
                    }
                }
            }
        }

        serial
    }
}
