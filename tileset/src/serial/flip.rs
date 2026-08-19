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
            TileOrSprite::Sprite(_) => flip_shift::<6>(flip),
        }
    }
}

/// Encodes a flip value for the Super Famicom target
impl FlipEncoder for ProfileSuperFamicom {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_shift::<6>(flip),
            TileOrSprite::Sprite(_) => flip_shift::<6>(flip),
        }
    }
}

/// Encodes a flip value for the Game Boy target
impl FlipEncoder for ProfileGameBoy {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => NO_FLIP,
            TileOrSprite::Sprite(_) => flip_shift::<5>(flip),
        }
    }
}

/// Encodes a flip value for the Game Boy Color target
impl FlipEncoder for ProfileGameBoyColor {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_shift::<5>(flip),
            TileOrSprite::Sprite(_) => flip_shift::<5>(flip),
        }
    }
}

/// Encodes a flip value for the Virtual Boy target
impl FlipEncoder for ProfileVirtualBoy {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_pos::<5, 4>(flip),
            TileOrSprite::Sprite(_) => flip_pos::<5, 4>(flip),
        }
    }
}

/// Encodes a flip value for the PC Engine target
impl FlipEncoder for ProfilePcEngine {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => NO_FLIP,
            TileOrSprite::Sprite(_) => flip_pos::<11, 15>(flip),
        }
    }
}

/// Encodes a flip value for the WonderSwan target
impl FlipEncoder for ProfileWonderSwan {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_shift::<6>(flip),
            TileOrSprite::Sprite(_) => flip_shift::<6>(flip),
        }
    }
}

/// Encodes a flip value for the Master System target
impl FlipEncoder for ProfileMasterSystem {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_shift::<9>(flip),
            TileOrSprite::Sprite(_) => NO_FLIP,
        }
    }
}

/// Encodes a flip value for the Mega Drive target
impl FlipEncoder for ProfileMegaDrive {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_shift::<3>(flip),
            TileOrSprite::Sprite(_) => flip_shift::<3>(flip),
        }
    }
}

/// Encodes a flip value for the Neo Geo Pocket target
impl FlipEncoder for ProfileNeoGeoPocket {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        match self.0 {
            TileOrSprite::Tile(_) => flip_pos::<7, 6>(flip),
            TileOrSprite::Sprite(_) => flip_pos::<7, 6>(flip),
        }
    }
}

/// Encodes a flip value for the Neo Geo target
impl FlipEncoder for ProfileNeoGeo {
    #[inline]
    fn encode_flip(&self, flip: Flip) -> u16 {
        flip as u16
    }
}

/// Flipping tiles is not supported for the target
/// For NES, MasterSystem, PC-Engine
const NO_FLIP: u16 = 0;

/// Encodes a flip value for a target by shifting the flip bits into the
/// appropriate position. This works for targets where horizontal flipping
/// is at bit position `N` and vertical flipping is at bit position `N + 1`.
#[inline]
fn flip_shift<const N: usize>(flip: Flip) -> u16 {
    ((flip as usize) << N) as u16
}

/// Encodes a flip value for a target with arbitrary bit positions for
/// horizontal and vertical flipping.
#[inline]
fn flip_pos<const H: usize, const V: usize>(flip: Flip) -> u16 {
    #[cfg_attr(cfg, rustfmt::skip)]
    match flip {
        Flip::None       =>  0,
        Flip::Horizontal =>  1 << H,
        Flip::Vertical   =>  1 << V,
        Flip::Both       => (1 << H) | (1 << V),
    }
}
