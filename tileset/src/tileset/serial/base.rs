use super::*;
use crate::tileset::{
    TileSet,
    serial::{endian::*, inner::*},
};
use bitvec::{order::Lsb0, vec::BitVec};
use bytes::Bytes;

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

impl SerialTile for SerialGenesis {
    /// Serialize the tileset in a Genesis compatible layout
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        let bits: BitVec<u32, Lsb0> = SerialLinear::new(4).serialize(tileset);
        bits_to_u32_be(&bits)
    }
}
