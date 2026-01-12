// reference: https://mrclick.zophar.net/TilEd/download/consolegfx.txt

/// Hardware serialization
mod base;

/// Generic serialization
mod inner;

use super::TileSet;
use bitvec::{order::BitOrder, store::BitStore, vec::BitVec};
use bytes::Bytes;

/// Define a serialization strategy for the tileset
pub trait SerialTile {
    fn serialize(&self, tileset: &TileSet) -> Bytes;
}

/// Define a serialization strategy for the tileset
trait SerialInner<T, O>
where
    T: BitStore,
    O: BitOrder,
{
    fn serialize(&self, tileset: &TileSet) -> BitVec<T, O>;
}

// MARK: Hardware

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

// MARK: Generics

/// Serialize the tileset where one pixel corresponds to one bit
struct SerialMono;

/// Serialize the tileset linearly
struct SerialLinear {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

/// Serialize the tileset as distinct bitplanes
struct SerialSplit {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

/// Serialize the tileset by intertwining rows by rows
struct SerialRowIntertwine {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,

    /// Define how many rows are intertwined
    intertwined_rows: usize,
}

impl SerialLinear {
    /// Define linear layout serialization
    #[inline]
    fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
    }
}

impl SerialSplit {
    /// Define splitted bitplane layout serialization
    #[inline]
    fn new(bits_per_pixel: usize) -> Self {
        Self { bits_per_pixel }
    }
}

impl SerialRowIntertwine {
    /// Define intertwined rows layout serialization
    #[inline]
    fn new(bits_per_pixel: usize, intertwined_rows: usize) -> Self {
        Self {
            bits_per_pixel,
            intertwined_rows,
        }
    }
}
