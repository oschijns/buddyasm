//! Serialize a tileset in the format expected by the target hardware

pub mod flip;
pub mod profile;
pub mod strategy;

use crate::data::{flip::Flip, tile::TileSet};
use bytes::Bytes;

/// Define a serialization strategy for the tileset
pub trait SerialTile {
    fn serialize(&self, tileset: &TileSet) -> Bytes;
}

/// Define how to encode flipping attributes for each target hardware
pub trait EncodeFlip {
    fn encode_flip(&self, flip: Flip) -> u8;
}
