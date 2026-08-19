//! Define method for creating attributes bytes for Tera templates

use crate::{config::output::IndexTile, data::flip::Flip};

/// Attribute value for background tile on NES
/// Basically the palette index of the tile
pub fn bg_nes(tile: IndexTile) -> u8 {
    tile.palette as u8
}

/// Attribute value for foreground tile on NES
/// Combine palette index and flip flags
pub fn fg_nes(tile: IndexTile) -> u8 {
    #[rustfmt::skip]
    let flip = match tile.flip {
        Flip::None       => 0b00_000000,
        Flip::Horizontal => 0b01_000000,
        Flip::Vertical   => 0b10_000000,
        Flip::Both       => 0b11_000000,
    };
    flip | (tile.palette as u8)
}

/// Attribute value for background tile on SNES
pub fn bg_snes(tile: IndexTile) -> u8 {
    0
}
