use crate::{
    data::tilemap::{TileData, TileMap},
    output_stack::{EncodedTile, OutMap, OutTile},
    profile::{
        BgFg, Profile, ProfileFamicom, ProfileGameBoy, ProfileGameBoyColor, ProfileMasterSystem,
        ProfileMegaDrive, ProfileNeoGeo, ProfileNeoGeoPocket, ProfilePcEngine, ProfileSuperFamicom,
        ProfileVirtualBoy, ProfileWonderSwan,
    },
};
use ndarray::Array2;

/// Encode tile data for the target hardware
pub fn encode_tiles(encoder: &dyn EncodeTileData, map: &TileMap) -> OutMap {
    // Prepare new buffer to add the encoded tile data
    let mut out_map = Array2::<OutTile>::default(map.0.dim());

    // Encode the tile data
    for (out, tile) in out_map.iter_mut().zip(map.0.iter()) {
        out.data = *tile;
        out.encoded = encoder.encode_tile_data(*tile);
    }

    OutMap::new(out_map)
}

/// Trait for encoding a tile for a given platform.
/// Depending on the platform, the words should be read as 8-bits or 16-bits values.
pub trait EncodeTileData {
    /// Encode the tile data for the target hardware
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile;

    /// Word size of the target hardware (8-bits or 16-bits)
    #[inline]
    fn long_word(&self) -> bool {
        // defaults to 8-bits words
        false
    }
}

impl EncodeTileData for Profile {
    #[rustfmt::skip]
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
        match self {
            Profile::Famicom     (profile) => profile.encode_tile_data(tile),
            Profile::SuperFamicom(profile) => profile.encode_tile_data(tile),
            Profile::GameBoy     (profile) => profile.encode_tile_data(tile),
            Profile::GameBoyColor(profile) => profile.encode_tile_data(tile),
            Profile::VirtualBoy  (profile) => profile.encode_tile_data(tile),
            Profile::PcEngine    (profile) => profile.encode_tile_data(tile),
            Profile::WonderSwan  (profile) => profile.encode_tile_data(tile),
            Profile::MasterSystem(profile) => profile.encode_tile_data(tile),
            Profile::MegaDrive   (profile) => profile.encode_tile_data(tile),
            Profile::NeoGeoPocket(profile) => profile.encode_tile_data(tile),
            Profile::NeoGeo      (profile) => profile.encode_tile_data(tile),
        }
    }
}

/// Encode tile data for the Famicom platform
impl EncodeTileData for ProfileFamicom {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
        /*
         * attributes layout:
         * [ VHP. ..CC ]
         * V: vertical flip
         * H: horizontal flip
         * P: priority
         * C: color palette index
         */
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = match self.mode {
            BgFg::Bg(_) => flip::NONE,
            BgFg::Fg(_) => flip::shift::<6>(tile.flip),
        };
        let palette = (tile.palette_index & 0b11) as u16;
        let attr = flip | palette;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the Super Famicom platform
impl EncodeTileData for ProfileSuperFamicom {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
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
        let flip = match self.mode {
            BgFg::Bg(_) => flip::shift::<6>(tile.flip),
            BgFg::Fg(_) => flip::shift::<6>(tile.flip),
        };
        let palette = ((tile.palette_index & 0b111) << 1) as u16;
        let high = if tile.tile_index < 0x100 { 0u16 } else { 1u16 };
        let attr = flip | palette | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the Game Boy platform
impl EncodeTileData for ProfileGameBoy {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
        /*
         * attributes layout:
         * [ PVHC .... ]
         * P: priority
         * V: vertical flip
         * H: horizontal flip
         * C: DMG palette index
         */
        let index = (tile.tile_index & 0xFF) as u16;
        let flip = match self.mode {
            BgFg::Bg(_) => flip::NONE,
            BgFg::Fg(_) => flip::shift::<5>(tile.flip),
        };
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
impl EncodeTileData for ProfileGameBoyColor {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
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
        let flip = match self.mode {
            BgFg::Bg(_) => flip::shift::<5>(tile.flip),
            BgFg::Fg(_) => flip::shift::<5>(tile.flip),
        };
        let palette = (tile.palette_index & 0b111) as u16;
        let attr = flip | palette;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the Virtual Boy platform
impl EncodeTileData for ProfileVirtualBoy {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
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
        let flip = match self.mode {
            BgFg::Bg(_) => flip::pos::<5, 4>(tile.flip),
            BgFg::Fg(_) => flip::pos::<5, 4>(tile.flip),
        };
        let high = ((tile.tile_index >> 16) & 0b111) as u16;
        let attr = palette | flip | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the PC Engine platform
impl EncodeTileData for ProfilePcEngine {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
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
        let flip = match self.mode {
            BgFg::Bg(_) => flip::NONE,
            BgFg::Fg(_) => flip::pos::<11, 15>(tile.flip),
        };
        let palette = (tile.palette_index & 0b1111) as u16;
        let attr = palette | flip;
        EncodedTile { index, attr }
    }

    #[inline]
    fn long_word(&self) -> bool {
        true
    }
}

/// Encode tile data for the WonderSwan platform
impl EncodeTileData for ProfileWonderSwan {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
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
        let flip = match self.mode {
            BgFg::Bg(_) => flip::shift::<6>(tile.flip),
            BgFg::Fg(_) => flip::shift::<6>(tile.flip),
        };
        let palette = ((tile.palette_index & 0b111) << 1) as u16;
        let high = if tile.tile_index < 0x100 { 0u16 } else { 1u16 };
        let attr = palette | flip | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the SEGA Master System platform
impl EncodeTileData for ProfileMasterSystem {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
        /* no attributes */
        let flip = match self.mode {
            BgFg::Bg(_) => flip::shift::<9>(tile.flip),
            BgFg::Fg(_) => flip::NONE,
        };
        let index = (tile.tile_index & 0xFF) as u16;
        EncodedTile { index, attr: 0u16 }
    }
}

/// Encode tile data for the SEGA Mega Drive platform
impl EncodeTileData for ProfileMegaDrive {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
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
        let flip = match self.mode {
            BgFg::Bg(_) => flip::shift::<3>(tile.flip),
            BgFg::Fg(_) => flip::shift::<3>(tile.flip),
        };
        let high = ((tile.tile_index >> 8) & 0b111) as u16;
        let attr = palette | flip | high;
        EncodedTile { index, attr }
    }
}

/// Encode tile data for the NeoGeo Pocket platform
impl EncodeTileData for ProfileNeoGeoPocket {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
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
        let flip = match self.mode {
            BgFg::Bg(_) => flip::pos::<7, 6>(tile.flip),
            BgFg::Fg(_) => flip::pos::<7, 6>(tile.flip),
        };
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
impl EncodeTileData for ProfileNeoGeo {
    fn encode_tile_data(&self, tile: TileData) -> EncodedTile {
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
        let flip = tile.flip as u16;
        let attr = palette | high | flip;
        EncodedTile { index, attr }
    }

    #[inline]
    fn long_word(&self) -> bool {
        true
    }
}

mod flip {
    use crate::data::flip::Flip;

    /// Flipping tiles is not supported for the target
    /// For NES, MasterSystem, PC-Engine
    pub(super) const NONE: u16 = 0;

    /// Encodes a flip value for a target by shifting the flip bits into the
    /// appropriate position. This works for targets where horizontal flipping
    /// is at bit position `N` and vertical flipping is at bit position `N + 1`.
    #[inline]
    pub(super) fn shift<const N: usize>(flip: Flip) -> u16 {
        ((flip as usize) << N) as u16
    }

    /// Encodes a flip value for a target with arbitrary bit positions for
    /// horizontal and vertical flipping.
    #[inline]
    pub(super) fn pos<const H: usize, const V: usize>(flip: Flip) -> u16 {
        match flip {
            Flip::None => 0,
            Flip::Horizontal => 1 << H,
            Flip::Vertical => 1 << V,
            Flip::Both => (1 << H) | (1 << V),
        }
    }
}
