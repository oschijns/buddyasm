//! Define how to encode tile data for a given platform

use crate::{
    config::profile::{
        ProfileFamicom, ProfileGameBoy, ProfileGameBoyColor, ProfileMasterSystem, ProfileMegaDrive,
        ProfileNeoGeo, ProfileNeoGeoPocket, ProfilePcEngine, ProfileSuperFamicom,
        ProfileVirtualBoy, ProfileWonderSwan,
    },
    process::output::IndexTile,
    serial::flip::FlipEncoder,
};

/// Trait for encoding a tile for a given platform.
/// Depending on the platform, the words should be read as 8-bits or 16-bits values.
pub trait TileData {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2];
}

/// Encode tile data for the Famicom platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileFamicom {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ VHP. ..CC ]
         * V: vertical flip
         * H: horizontal flip
         * P: priority
         * C: color palette index
         */
        let index = (tile.tile & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = (tile.palette & 0b11) as u16;
        let attr = flip | palette;
        [index, attr]
    }
}

/// Encode tile data for the Super Famicom platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileSuperFamicom {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ VHPP CCCI ]
         * V: vertical flip
         * H: horizontal flip
         * P: priority
         * C: color palette index
         * I: tile index (high bit) (aka name select)
         */
        let index = (tile.tile & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = ((tile.palette & 0b111) << 1) as u16;
        let high = if tile.tile < 0x100 { 0u16 } else { 1u16 };
        let attr = flip | palette | high;
        [index, attr]
    }
}

/// Encode tile data for the Game Boy platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileGameBoy {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ PVHC .... ]
         * P: priority
         * V: vertical flip
         * H: horizontal flip
         * C: DMG palette index
         */
        let index = (tile.tile & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = if tile.palette == 0 { 0u16 } else { 0b1_0000 };
        let attr = flip | palette;
        [index, attr]
    }
}

/// Encode tile data for the Game Boy Color platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileGameBoyColor {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ PVH. BCCC ]
         * P: priority
         * V: vertical flip
         * H: horizontal flip
         * B: bank
         * C: color palette index
         */
        let index = (tile.tile & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = (tile.palette & 0b111) as u16;
        let attr = flip | palette;
        [index, attr]
    }
}

/// Encode tile data for the Virtual Boy platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileVirtualBoy {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ CCHV .III ]
         * C: color palette index
         * H: horizontal flip
         * V: vertical flip
         * I: tile index (high bits)
         */
        let index = (tile.tile & 0xFF) as u16;
        let palette = ((tile.palette & 0b11) << 14) as u16;
        let flip = self.encode_flip(tile.flip);
        let high = ((tile.tile >> 16) & 0b111) as u16;
        let attr = palette | flip | high;
        [index, attr]
    }
}

/// Encode tile data for the PC Engine platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfilePcEngine {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ V.yy H..x P... CCCC ]
         * V: vertical flip
         * y: Y scale
         * H: horizontal flip
         * x: X scale
         * P: priority
         * C: color palette index
         */
        let index = (tile.tile & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = (tile.palette & 0b1111) as u16;
        let attr = palette | flip;
        [index, attr]
    }
}

/// Encode tile data for the WonderSwan platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileWonderSwan {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ VHPW CCCI ]
         * V: vertical flip
         * H: horizontal flip
         * P: priority
         * W: window
         * C: color palette index
         * I: tile index (high bits)
         */
        let index = (tile.tile & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = ((tile.palette & 0b111) << 1) as u16;
        let high = if tile.tile < 0x100 { 0u16 } else { 1u16 };
        let attr = palette | flip | high;
        [index, attr]
    }
}

/// Encode tile data for the SEGA Master System platform
/// Word 0: tile index
impl TileData for ProfileMasterSystem {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /* no attributes */
        let index = (tile.tile & 0xFF) as u16;
        [index, 0u16]
    }
}

/// Encode tile data for the SEGA Mega Drive platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileMegaDrive {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ PCCV HIII ]
         * P: priority
         * C: color palette index
         * V: vertical flip
         * H: horizontal flip
         * I: tile index (high bits)
         */
        let index = (tile.tile & 0xFF) as u16;
        let palette = ((tile.palette & 0b111) << 1) as u16;
        let flip = self.encode_flip(tile.flip);
        let high = ((tile.tile >> 8) & 0b111) as u16;
        let attr = palette | flip | high;
        [index, attr]
    }
}

/// Encode tile data for the NeoGeo Pocket platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileNeoGeoPocket {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ HVCp PhvI ]
         * H: horizontal flip
         * V: vertical flip
         * C: color palette index
         * P: priority
         * h: horizontal position chain
         * v: vertical position chain
         * I: tile index (high bits) (aka character code)
         */
        let index = (tile.tile & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = if tile.palette == 0 { 0u16 } else { 0b10_0000 };
        let high = if tile.tile < 0x100 { 0u16 } else { 1u16 };
        let attr = palette | flip | high;
        [index, attr]
    }
}

/// Encode tile data for the NeoGeo platform
/// Word 0: tile index
/// Word 1: attributes
impl TileData for ProfileNeoGeo {
    fn encode_tile(&self, tile: IndexTile) -> [u16; 2] {
        /*
         * attributes layout:
         * [ CCCC CCCC IIII AaVH ]
         * C: color palette index
         * I: tile index (high bits)
         * A: 3bits auto-animation
         * a: 2bits auto-animation
         * V: vertical flip
         * H: horizontal flip
         */
        let index = (tile.tile & 0xFFFF) as u16;
        let palette = ((tile.palette & 0xFF) << 8) as u16;
        let high = ((tile.tile & 0xF_0000) >> 4) as u16;
        let flip = self.encode_flip(tile.flip);
        let attr = palette | high | flip;
        [index, attr]
    }
}
