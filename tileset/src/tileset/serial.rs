// reference: https://mrclick.zophar.net/TilEd/download/consolegfx.txt

/// Hardware serialization
mod base;

/// Generic serialization
mod inner;

/// Endianness handling
mod endian;

use super::TileSet;
use bytes::Bytes;

/// Define a serialization strategy for the tileset
pub trait SerialTile {
    fn serialize(&self, tileset: &TileSet) -> Bytes;
}

/// Define the serialization options available for the Famicom
pub enum SerialFamicom {
    /// Use 1-bit per pixel
    Bpp1,

    /// Use 2-bits per pixel
    Bpp2,
}

/// Define the serialization options available for the Super Famicom
pub enum SerialSuperFamicom {
    /// Use 2-bits per pixel
    Bpp2,

    /// Use 4-bits per pixel
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

/// Define the serialization for the Genesis
pub struct SerialGenesis;
