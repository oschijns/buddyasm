//! Specify how to encode flipping attributes for each target hardware

/*
NES
    BG:       no flip
    Sprite:   H = bit 6, V = bit 7

SNES
    BG:       H = bit 14, V = bit 15
    Sprite:   H = bit 6, V = bit 7

Game Boy
    BG DMG:   no flip
    BG GBC:   H = bit 5, V = bit 6
    Sprite:   H = bit 5, V = bit 6

WonderSwan
    BG:       H = bit 14, V = bit 15

PC-Engine
    BG:       H = bit ?, V = bit ?

Master System
    BG:       H = bit 9, V = bit 10
    Sprite:   no flip

Mega Drive
    BG:       H = bit 11, V = bit 12
    Sprite:   H/V are represented in the sprite's tile attribute word

Neo Geo Pocket
    BG:       H = bit 15, V = bit 14   // considering whole 16-bit entry
    Sprite:   H = bit 15, V = bit 14

Neo Geo
    Sprite tile: H = bit 0, V = bit 1
*/

use crate::{
    config::{
        profile::{
            ProfileFamicom, ProfileGameBoy, ProfileGameBoyColor, ProfileMasterSystem,
            ProfileMegaDrive, ProfileNeoGeo, ProfileNeoGeoPocket, ProfilePcEngine,
            ProfileSuperFamicom, ProfileVirtualBoy, ProfileWonderSwan,
        },
        tile::TileOrSprite,
    },
    data::flip::Flip,
};

/// Define how to encode a flip value for a given target
pub trait FlipEncoder {
    /// Encode a flip value as a 16-bit value
    fn encode_flip(&self, flip: Flip) -> u16;
}

/// Encodes a flip value for the Famicom target
impl FlipEncoder for ProfileFamicom {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => NO_FLIP,
            TileOrSprite::Sprite(_) => flip_h6_v7(flip),
        }
    }
}

/// Encodes a flip value for the Super Famicom target
impl FlipEncoder for ProfileSuperFamicom {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_h14_v15(flip),
            TileOrSprite::Sprite(_) => flip_h6_v7(flip),
        }
    }
}

/// Encodes a flip value for the Game Boy target
impl FlipEncoder for ProfileGameBoy {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => NO_FLIP,
            TileOrSprite::Sprite(_) => flip_h5_v6(flip),
        }
    }
}

/// Encodes a flip value for the Game Boy Color target
impl FlipEncoder for ProfileGameBoyColor {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        flip_h5_v6(flip)
    }
}

/// Encodes a flip value for the Virtual Boy target
impl FlipEncoder for ProfileVirtualBoy {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        flip_h14_v15(flip)
    }
}

/// Encodes a flip value for the PC Engine target
impl FlipEncoder for ProfilePcEngine {
    #[inline]
    fn encode_flip(&self, _flip: Flip) -> u16 {
        NO_FLIP
    }
}

/// Encodes a flip value for the WonderSwan target
impl FlipEncoder for ProfileWonderSwan {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        flip_h6_v7(flip)
    }
}

/// Encodes a flip value for the Master System target
impl FlipEncoder for ProfileMasterSystem {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_h9_v10(flip) as u16,
            TileOrSprite::Sprite(_) => NO_FLIP,
        }
    }
}

/// Encodes a flip value for the Mega Drive target
impl FlipEncoder for ProfileMegaDrive {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        flip_h11_v12(flip)
    }
}

/// Encodes a flip value for the Neo Geo Pocket target
impl FlipEncoder for ProfileNeoGeoPocket {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        flip_h15_v14(flip)
    }
}

/// Encodes a flip value for the Neo Geo target
impl FlipEncoder for ProfileNeoGeo {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        flip_h0_v1(flip)
    }
}

/// Flipping tiles is not supported for the target
/// For NES, MasterSystem, PC-Engine
const NO_FLIP: u16 = 0;

/// Flipping bits for NES, SNES, WonderSwan
#[inline]
fn flip_h6_v7(flip: Flip) -> u16 {
    (flip as u16) << 6
}

/// Flipping bits for GameBoy
#[inline]
fn flip_h5_v6(flip: Flip) -> u16 {
    (flip as u16) << 5
}

/// Flipping bits for Master System
#[inline]
fn flip_h9_v10(flip: Flip) -> u16 {
    (flip as u16) << 9
}

/// Flipping bits for SNES, VirtualBoy, WonderSwan
#[inline]
fn flip_h14_v15(flip: Flip) -> u16 {
    (flip as u16) << 14
}

/// Flipping bits for NeoGeo Pocket
#[inline]
fn flip_h15_v14(flip: Flip) -> u16 {
    #[cfg_attr(cfg, rustfmt::skip)]
    match flip {
        Flip::None       => 0b00_000000_00000000,
        Flip::Horizontal => 0b10_000000_00000000,
        Flip::Vertical   => 0b01_000000_00000000,
        Flip::Both       => 0b11_000000_00000000,
    }
}

/// Flipping bits for MegaDrive
#[inline]
fn flip_h11_v12(flip: Flip) -> u16 {
    (flip as u16) << 11
}

/// Flipping bits for NeoGeo
#[inline]
fn flip_h0_v1(flip: Flip) -> u16 {
    flip as u16
}
