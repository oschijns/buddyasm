// reference: https://mrclick.zophar.net/TilEd/download/consolegfx.txt

use super::TileSet;
use bitvec::{order::Msb0, vec::BitVec};

/// Define the serial output format
pub type Serial = BitVec<u8, Msb0>;

/// Define a serialization strategy for the tileset
pub trait SerialTile {
    fn serialize(&self, tileset: &TileSet) -> Serial;
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
pub struct SerialMono;

/// Serialize the tileset linearly
pub struct SerialLinear {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

/// Serialize the tileset as distinct bitplanes
pub struct SerialSplit {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,
}

/// Serialize the tileset by intertwining rows by rows
pub struct SerialRowIntertwine {
    /// How many bits are used to define a pixel
    bits_per_pixel: usize,

    /// Define how many rows are intertwined
    intertwined_rows: usize,
}

// MARK: Implementations

impl SerialTile for SerialFamicom {
    /// Serialize the tileset in a Famicom compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        match self {
            Self::Bpp1 => SerialMono.serialize(tileset),
            Self::Bpp2 => SerialSplit::new(2).serialize(tileset),
        }
    }
}

impl SerialTile for SerialSuperFamicom {
    /// Serialize the tileset in a Super Famicom compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        match self {
            Self::Bpp2 => SerialRowIntertwine::new(2, 2).serialize(tileset),
            Self::Bpp4 => SerialRowIntertwine::new(4, 2).serialize(tileset),
            Self::Bpp8 => SerialRowIntertwine::new(8, 2).serialize(tileset),
            Self::Mode7 => SerialLinear::new(8).serialize(tileset),
        }
    }
}

impl SerialTile for SerialGameBoy {
    /// Serialize the tileset in a GameBoy compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        SerialRowIntertwine::new(2, 2).serialize(tileset)
    }
}

impl SerialTile for SerialVirtualBoy {
    /// Serialize the tileset in a VirtualBoy compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        SerialLinear::new(2).serialize(tileset)
    }
}

impl SerialTile for SerialPcEngine {
    /// Serialize the tileset in a PC-Engine compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        SerialRowIntertwine::new(4, 2).serialize(tileset)
    }
}

impl SerialTile for SerialNeoGeoPocket {
    /// Serialize the tileset in a NeoGeo Pocket compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        // TODO: flipped endianness compared to the VirtualBoy
        SerialLinear::new(2).serialize(tileset)
    }
}

impl SerialTile for SerialWonderSwan {
    /// Serialize the tileset in a WonderSwan compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        SerialRowIntertwine::new(4, 4).serialize(tileset)
    }
}

impl SerialTile for SerialMasterSystem {
    /// Serialize the tileset in a MasterSystem compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        SerialRowIntertwine::new(4, 4).serialize(tileset)
    }
}

impl SerialTile for SerialGenesis {
    /// Serialize the tileset in a Genesis compatible layout
    fn serialize(&self, tileset: &TileSet) -> Serial {
        SerialLinear::new(4).serialize(tileset)
    }
}

// MARK: Generic implementations

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

impl SerialTile for SerialMono {
    /// Serialize the tileset where one pixel corresponds to one bit
    fn serialize(&self, tileset: &TileSet) -> Serial {
        // The serialization to produce
        let mut serial = Serial::with_capacity(tileset.pixel_count());

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

impl SerialTile for SerialLinear {
    /// Serialize the tileset where the bits of a pixel are stored linearly
    fn serialize(&self, tileset: &TileSet) -> Serial {
        // The serialization to produce
        let mut serial = Serial::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

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

impl SerialTile for SerialSplit {
    /// Serialize the tileset as multiple bitplanes
    fn serialize(&self, tileset: &TileSet) -> Serial {
        // The serialization to produce
        let mut serial = Serial::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

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

impl SerialTile for SerialRowIntertwine {
    /// Serialize the tileset as multiple bitplanes
    fn serialize(&self, tileset: &TileSet) -> Serial {
        // The serialization to produce
        let mut serial = Serial::with_capacity(tileset.pixel_count() * self.bits_per_pixel);

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
