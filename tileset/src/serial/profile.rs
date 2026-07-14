//! Define hardware profile for serializing the tile data

use std::str::FromStr;

use crate::{
    config::{profile::Hardware, tile::ParseError},
    data::tile::TileSet,
    serial::{
        SerialTile,
        endian::{bits_to_u16_be, bits_to_u16_le, bits_to_u32_be},
        strategy::{SerialLinear, SerialMono, SerialRowIntertwine, SerialSplit, SerialStrategy},
    },
};
use bitvec::{order::Lsb0, vec::BitVec};
use bytes::Bytes;
use serde::Deserialize;
use strum::EnumString;

/// List all hardware profiles available
#[derive(Debug, Clone)]
pub enum Serial {
    /// Famicom / NES
    Famicom(SerialFamicom),

    /// Super Famicom / SNES
    SuperFamicom(SerialSuperFamicom),

    /// Game Boy
    GameBoy(SerialGameBoy),

    /// Virtual Boy
    VirtualBoy(SerialVirtualBoy),

    /// PC-Engine
    PcEngine(SerialPcEngine),

    /// NeoGeo Pocket
    NeoGeoPocket(SerialNeoGeoPocket),

    /// Wonder Swan
    WonderSwan(SerialWonderSwan),

    /// Master System
    MasterSystem(SerialMasterSystem),

    /// MegaDrive / Genesis
    MegaDrive(SerialMegaDrive),
}

impl Serial {
    /// A serializer from the given parameters
    #[rustfmt::skip]
    pub fn new(hardware: Hardware, bit_plane: Option<&str>) -> Result<Self, ParseError> {
        match hardware {
            Hardware::Famicom      => Ok(Self::Famicom     (parse_bit_plane::<SerialFamicom     >(bit_plane)?)),
            Hardware::SuperFamicom => Ok(Self::SuperFamicom(parse_bit_plane::<SerialSuperFamicom>(bit_plane)?)),
            Hardware::GameBoy      => Ok(Self::GameBoy     (SerialGameBoy     ::default())),
            Hardware::VirtualBoy   => Ok(Self::VirtualBoy  (SerialVirtualBoy  ::default())),
            Hardware::PcEngine     => Ok(Self::PcEngine    (SerialPcEngine    ::default())),
            Hardware::NeoGeoPocket => Ok(Self::NeoGeoPocket(SerialNeoGeoPocket::default())),
            Hardware::WonderSwan   => Ok(Self::WonderSwan  (SerialWonderSwan  ::default())),
            Hardware::MasterSystem => Ok(Self::MasterSystem(SerialMasterSystem::default())),
            Hardware::MegaDrive    => Ok(Self::MegaDrive   (SerialMegaDrive   ::default())),
        }
    }
}

/// Define the serialization options available for the Famicom
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, EnumString)]
pub enum SerialFamicom {
    /// Use 1-bit per pixel
    #[serde(rename = "bpp1")]
    #[strum(serialize = "bpp1")]
    Bpp1,

    /// Use 2-bits per pixel
    #[default]
    #[serde(rename = "bpp2")]
    #[strum(serialize = "bpp2")]
    Bpp2,
}

/// Define the serialization options available for the Super Famicom
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, EnumString)]
pub enum SerialSuperFamicom {
    /// Use 2-bits per pixel
    #[serde(rename = "bpp2")]
    #[strum(serialize = "bpp2")]
    Bpp2,

    /// Use 4-bits per pixel
    #[default]
    #[serde(rename = "bpp4")]
    #[strum(serialize = "bpp4")]
    Bpp4,

    /// Use 8-bits per pixel
    #[serde(rename = "bpp8")]
    #[strum(serialize = "bpp8")]
    Bpp8,

    /// Mode7 serialization
    #[serde(rename = "mode7")]
    #[strum(serialize = "mode7")]
    Mode7,
}

/// Parse a bit plane format
fn parse_bit_plane<S>(bit_plane: Option<&str>) -> Result<S, ParseError>
where
    S: Default + FromStr,
{
    if let Some(bpp) = bit_plane {
        if let Ok(s) = S::from_str(bpp) {
            Ok(s)
        } else {
            Err(ParseError(bpp.to_string()))
        }
    } else {
        Ok(S::default())
    }
}

/// Define the serialization for the GameBoy
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialGameBoy;

/// Define the serialization for the VirtualBoy
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialVirtualBoy;

/// Define the serialization for the PC-Engine
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialPcEngine;

/// Define the serialization for the NeoGeo Pocket
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialNeoGeoPocket;

/// Define the serialization for the WonderSwan
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialWonderSwan;

/// Define the serialization for the MasterSystem
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialMasterSystem;

/// Define the serialization for the MegaDrive
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialMegaDrive;

impl SerialTile for Serial {
    #[rustfmt::skip]
    fn serialize(&self, tileset: &TileSet) -> Bytes {
        match self {
            Serial::Famicom     (serial) => serial.serialize(tileset),
            Serial::SuperFamicom(serial) => serial.serialize(tileset),
            Serial::GameBoy     (serial) => serial.serialize(tileset),
            Serial::VirtualBoy  (serial) => serial.serialize(tileset),
            Serial::PcEngine    (serial) => serial.serialize(tileset),
            Serial::NeoGeoPocket(serial) => serial.serialize(tileset),
            Serial::WonderSwan  (serial) => serial.serialize(tileset),
            Serial::MasterSystem(serial) => serial.serialize(tileset),
            Serial::MegaDrive   (serial) => serial.serialize(tileset),
        }
    }
}

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
