//! Serialize a tileset in the format expected by the target hardware

pub mod profile;
pub mod strategy;

use crate::data::tile::TileSet;
use bytes::Bytes;

/// Define a serialization strategy for the tileset
pub trait SerialTile {
    fn serialize(&self, tileset: &TileSet) -> Bytes;
}
