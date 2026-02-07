//! Define hardware profile for serializing the tile data

use crate::{
    data::tile::TileSet,
    serial::{
        SerialTile,
        endian::{bits_to_u16_be, bits_to_u16_le, bits_to_u32_be},
        strategy::{SerialLinear, SerialMono, SerialRowIntertwine, SerialSplit, SerialStrategy},
    },
};
use bitvec::{order::Lsb0, vec::BitVec};
use bytes::Bytes;

/// Define the serialization options available for the Famicom
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SerialFamicom {
    /// Use 1-bit per pixel
    Bpp1,

    /// Use 2-bits per pixel
    #[default]
    Bpp2,
}

/// Define the serialization options available for the Super Famicom
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SerialSuperFamicom {
    /// Use 2-bits per pixel
    Bpp2,

    /// Use 4-bits per pixel
    #[default]
    Bpp4,

    /// Use 8-bits per pixel
    Bpp8,

    /// Mode7 serialization
    Mode7,
}

/// Define the serialization for the GameBoy
pub struct SerialGameBoy;

/// Define the serialization for the VirtualBoy
pub struct SerialVirtualBoy;

/// Define the serialization for the PC-Engine
pub struct SerialPcEngine;

/// Define the serialization for the NeoGeo Pocket
pub struct SerialNeoGeoPocket;

/// Define the serialization for the WonderSwan
pub struct SerialWonderSwan;

/// Define the serialization for the MasterSystem
pub struct SerialMasterSystem;

/// Define the serialization for the MegaDrive
pub struct SerialMegaDrive;

impl SerialTile for SerialFamicom {
    /// Serialize the tileset in a Famicom compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = match self {
            Self::Bpp1 => SerialMono.serialize(tileset),
            Self::Bpp2 => SerialSplit::new(2).serialize(tileset),
        };
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerialTile for SerialSuperFamicom {
    /// Serialize the tileset in a Super Famicom compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = match self {
            Self::Bpp2 => SerialRowIntertwine::new(2, 2).serialize(tileset),
            Self::Bpp4 => SerialRowIntertwine::new(4, 2).serialize(tileset),
            Self::Bpp8 => SerialRowIntertwine::new(8, 2).serialize(tileset),
            Self::Mode7 => SerialLinear::new(8).serialize(tileset),
        };
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerialTile for SerialGameBoy {
    /// Serialize the tileset in a GameBoy compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = SerialRowIntertwine::new(2, 2).serialize(tileset);
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerialTile for SerialVirtualBoy {
    /// Serialize the tileset in a VirtualBoy compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u16, Lsb0> = SerialLinear::new(2).serialize(tileset);
        bits_to_u16_be(&bits)
    }
}

impl SerialTile for SerialPcEngine {
    /// Serialize the tileset in a PC-Engine compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = SerialRowIntertwine::new(4, 2).serialize(tileset);
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerialTile for SerialNeoGeoPocket {
    /// Serialize the tileset in a NeoGeo Pocket compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u16, Lsb0> = SerialLinear::new(2).serialize(tileset);
        bits_to_u16_le(&bits)
    }
}

impl SerialTile for SerialWonderSwan {
    /// Serialize the tileset in a WonderSwan compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = SerialRowIntertwine::new(4, 4).serialize(tileset);
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerialTile for SerialMasterSystem {
    /// Serialize the tileset in a MasterSystem compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u8, Lsb0> = SerialRowIntertwine::new(4, 4).serialize(tileset);
        Bytes::copy_from_slice(bits.as_raw_slice())
    }
}

impl SerialTile for SerialMegaDrive {
    /// Serialize the tileset in a MegaDrive compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u32, Lsb0> = SerialLinear::new(4).serialize(tileset);
        bits_to_u32_be(&bits)
    }
}
