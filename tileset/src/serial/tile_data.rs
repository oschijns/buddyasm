//! Define how to encode tile data for a given platform

use crate::{
    config::profile::{
        Profile, ProfileFamicom, ProfileGameBoy, ProfileGameBoyColor, ProfileMasterSystem,
        ProfileMegaDrive, ProfileNeoGeo, ProfileNeoGeoPocket, ProfilePcEngine, ProfileSuperFamicom,
        ProfileVirtualBoy, ProfileWonderSwan,
    },
    process::output::{EncodedTile, IndexMap, OutTile},
    serial::flip::FlipEncoder,
};

/// Encode tile data for the target hardware
pub fn encode_tiles(encoder: &dyn TileData, map: &mut IndexMap) {
    for tile in map.0.borrow_mut().iter_mut() {
        tile.encoded = encoder.encode_tile(*tile);
    }
}

/// Trait for encoding a tile for a given platform.
/// Depending on the platform, the words should be read as 8-bits or 16-bits values.
pub trait TileData {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile;
}

impl TileData for Profile {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
        #[cfg_attr(cfg, rustfmt::skip)]
        match self {
            Profile::Famicom     (profile) => profile.encode_tile(tile),
            Profile::SuperFamicom(profile) => profile.encode_tile(tile),
            Profile::GameBoy     (profile) => profile.encode_tile(tile),
            Profile::GameBoyColor(profile) => profile.encode_tile(tile),
            Profile::VirtualBoy  (profile) => profile.encode_tile(tile),
            Profile::PcEngine    (profile) => profile.encode_tile(tile),
            Profile::WonderSwan  (profile) => profile.encode_tile(tile),
            Profile::MasterSystem(profile) => profile.encode_tile(tile),
            Profile::MegaDrive   (profile) => profile.encode_tile(tile),
            Profile::NeoGeoPocket(profile) => profile.encode_tile(tile),
            Profile::NeoGeo      (profile) => profile.encode_tile(tile),
        }
    }
}

/// Encode tile data for the Famicom platform
impl TileData for ProfileFamicom {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
        /*
         * attributes layout:
         * [ VHP. ..CC ]
         * V: vertical flip
         * H: horizontal flip
         * P: priority
         * C: color palette index
         */
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = (tile.palette_index & 0b11) as u16;
        let attr = flip | palette;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the Super Famicom platform
impl TileData for ProfileSuperFamicom {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
        /*
         * attributes layout:
         * [ VHPP CCCI ]
         * V: vertical flip
         * H: horizontal flip
         * P: priority
         * C: color palette index
         * I: tile index (high bit) (aka name select)
         */
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = ((tile.palette_index & 0b111) << 1) as u16;
        let high = if tile.tile_index < 0x100 { 0u16 } else { 1u16 };
        let attr = flip | palette | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the Game Boy platform
impl TileData for ProfileGameBoy {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
        /*
         * attributes layout:
         * [ PVHC .... ]
         * P: priority
         * V: vertical flip
         * H: horizontal flip
         * C: DMG palette index
         */
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = if tile.palette_index == 0 {
            0u16
        } else {
            0b1_0000
        };
        let attr = flip | palette;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the Game Boy Color platform
impl TileData for ProfileGameBoyColor {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
        /*
         * attributes layout:
         * [ PVH. BCCC ]
         * P: priority
         * V: vertical flip
         * H: horizontal flip
         * B: bank
         * C: color palette index
         */
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = (tile.palette_index & 0b111) as u16;
        let attr = flip | palette;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the Virtual Boy platform
impl TileData for ProfileVirtualBoy {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
        /*
         * attributes layout:
         * [ CCHV .III ]
         * C: color palette index
         * H: horizontal flip
         * V: vertical flip
         * I: tile index (high bits)
         */
        let index = (tile.tile_index & 0xFF) as u16;
        let palette = ((tile.palette_index & 0b11) << 14) as u16;
        let flip = self.encode_flip(tile.flip);
        let high = ((tile.tile_index >> 16) & 0b111) as u16;
        let attr = palette | flip | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the PC Engine platform
impl TileData for ProfilePcEngine {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
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
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = (tile.palette_index & 0b1111) as u16;
        let attr = palette | flip;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the WonderSwan platform
impl TileData for ProfileWonderSwan {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
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
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = ((tile.palette_index & 0b111) << 1) as u16;
        let high = if tile.tile_index < 0x100 { 0u16 } else { 1u16 };
        let attr = palette | flip | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the SEGA Master System platform
impl TileData for ProfileMasterSystem {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
        /* no attributes */
        let index = (tile.tile_index & 0xFF) as u16;
        EncodedTile { index, attr: 0u16 }
    }
}

/// Encode tile data for the SEGA Mega Drive platform
impl TileData for ProfileMegaDrive {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
        /*
         * attributes layout:
         * [ PCCV HIII ]
         * P: priority
         * C: color palette index
         * V: vertical flip
         * H: horizontal flip
         * I: tile index (high bits)
         */
        let index = (tile.tile_index & 0xFF) as u16;
        let palette = ((tile.palette_index & 0b111) << 1) as u16;
        let flip = self.encode_flip(tile.flip);
        let high = ((tile.tile_index >> 8) & 0b111) as u16;
        let attr = palette | flip | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the NeoGeo Pocket platform
impl TileData for ProfileNeoGeoPocket {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
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
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = self.encode_flip(tile.flip);
        let palette = if tile.palette_index == 0 {
            0u16
        } else {
            0b10_0000
        };
        let high = if tile.tile_index < 0x100 { 0u16 } else { 1u16 };
        let attr = palette | flip | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the NeoGeo platform
impl TileData for ProfileNeoGeo {
    fn encode_tile(&self, tile: OutTile) -> EncodedTile {
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
        let index = (tile.tile_index & 0xFFFF) as u16;
        let palette = ((tile.palette_index & 0xFF) << 8) as u16;
        let high = ((tile.tile_index & 0xF_0000) >> 4) as u16;
        let flip = self.encode_flip(tile.flip);
        let attr = palette | high | flip;
        EncodedTile { index, attr }
    }
}
