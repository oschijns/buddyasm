use std::str::FromStr;

use crate::{
    data::{coords::TileSize, flip::Flip},
    manifest::{System, TileKind},
};
use strum::EnumString;

// MARK: Traits

/// Get the size of the tiles for the given hardware
pub trait GetTileSize {
    fn tile_size(&self) -> TileSize;
}

/// Get the horizontal and vertical flipping
pub trait GetTileFlip {
    fn tile_flip(&self) -> Flip;
}

// MARK: Profile

/// List all hardware profiles available
#[derive(Debug, Clone, Copy)]
pub enum Profile {
    /// Famicom / NES
    Famicom(ProfileFamicom),

    /// Super Famicom / SNES
    SuperFamicom(ProfileSuperFamicom),

    /// Game Boy
    GameBoy(ProfileGameBoy),

    /// Game Boy Color
    GameBoyColor(ProfileGameBoyColor),

    /// Virtual Boy
    VirtualBoy(ProfileVirtualBoy),

    /// PC-Engine
    PcEngine(ProfilePcEngine),

    /// Wonder Swan
    WonderSwan(ProfileWonderSwan),

    /// Master System
    MasterSystem(ProfileMasterSystem),

    /// MegaDrive / Genesis
    MegaDrive(ProfileMegaDrive),

    /// NeoGeo Pocket
    NeoGeoPocket(ProfileNeoGeoPocket),

    /// NeoGeo
    NeoGeo(ProfileNeoGeo),
}

/// Error encountered when identifying the target hardware
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IdentError {
    #[error("Unknown bitplane: {0}")]
    Strum(#[from] strum::ParseError),
}

impl Profile {
    /// Build a profile given the parameters
    #[rustfmt::skip]
    pub fn new(
        system: System,
        kind: TileKind,
        bitplane: Option<&str>,
        sprite_size: Option<&str>,
    ) -> Result<Self, IdentError> {
        match kind {
            TileKind::Background => match system {
                System::Famicom      => Ok(Self::Famicom     (ProfileFamicom     { mode: BgFg::default_bg(), bitplane: parse_bitplane(bitplane)? })),
                System::SuperFamicom => Ok(Self::SuperFamicom(ProfileSuperFamicom{ mode: BgFg::default_bg(), bitplane: parse_bitplane(bitplane)? })),
                System::GameBoy      => Ok(Self::GameBoy     (ProfileGameBoy     { mode: BgFg::default_bg() })),
                System::GameBoyColor => Ok(Self::GameBoyColor(ProfileGameBoyColor{ mode: BgFg::default_bg() })),
                System::VirtualBoy   => Ok(Self::VirtualBoy  (ProfileVirtualBoy  { mode: BgFg::default_bg() })),
                System::PcEngine     => Ok(Self::PcEngine    (ProfilePcEngine    { mode: BgFg::default_bg() })),
                System::WonderSwan   => Ok(Self::WonderSwan  (ProfileWonderSwan  { mode: BgFg::default_bg() })),
                System::MasterSystem => Ok(Self::MasterSystem(ProfileMasterSystem{ mode: BgFg::default_bg() })),
                System::MegaDrive    => Ok(Self::MegaDrive   (ProfileMegaDrive   { mode: BgFg::default_bg() })),
                System::NeoGeoPocket => Ok(Self::NeoGeoPocket(ProfileNeoGeoPocket{ mode: BgFg::default_bg() })),
                System::NeoGeo       => Ok(Self::NeoGeo      (ProfileNeoGeo      { mode: Default::default() })),
            },
            TileKind::Foreground => match system {
                System::Famicom      => Ok(Self::Famicom     (ProfileFamicom     { mode: parse_sprite_size(sprite_size)?, bitplane: parse_bitplane(bitplane)? })),
                System::SuperFamicom => Ok(Self::SuperFamicom(ProfileSuperFamicom{ mode: parse_sprite_size(sprite_size)?, bitplane: parse_bitplane(bitplane)? })),
                System::GameBoy      => Ok(Self::GameBoy     (ProfileGameBoy     { mode: parse_sprite_size(sprite_size)? })),
                System::GameBoyColor => Ok(Self::GameBoyColor(ProfileGameBoyColor{ mode: parse_sprite_size(sprite_size)? })),
                System::VirtualBoy   => Ok(Self::VirtualBoy  (ProfileVirtualBoy  { mode: parse_sprite_size(sprite_size)? })),
                System::PcEngine     => Ok(Self::PcEngine    (ProfilePcEngine    { mode: parse_sprite_size(sprite_size)? })),
                System::WonderSwan   => Ok(Self::WonderSwan  (ProfileWonderSwan  { mode: BgFg::default_fg() })),
                System::MasterSystem => Ok(Self::MasterSystem(ProfileMasterSystem{ mode: parse_sprite_size(sprite_size)? })),
                System::MegaDrive    => Ok(Self::MegaDrive   (ProfileMegaDrive   { mode: parse_sprite_size(sprite_size)? })),
                System::NeoGeoPocket => Ok(Self::NeoGeoPocket(ProfileNeoGeoPocket{ mode: parse_sprite_size(sprite_size)? })),
                System::NeoGeo       => Ok(Self::NeoGeo      (ProfileNeoGeo      { mode: Default::default() })),
            },
        }
    }
}

impl GetTileSize for Profile {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Profile::Famicom     (profile) => profile.tile_size(),
            Profile::SuperFamicom(profile) => profile.tile_size(),
            Profile::GameBoy     (profile) => profile.tile_size(),
            Profile::GameBoyColor(profile) => profile.tile_size(),
            Profile::VirtualBoy  (profile) => profile.tile_size(),
            Profile::PcEngine    (profile) => profile.tile_size(),
            Profile::WonderSwan  (profile) => profile.tile_size(),
            Profile::MasterSystem(profile) => profile.tile_size(),
            Profile::MegaDrive   (profile) => profile.tile_size(),
            Profile::NeoGeoPocket(profile) => profile.tile_size(),
            Profile::NeoGeo      (profile) => profile.tile_size(),
        }
    }
}

impl GetTileFlip for Profile {
    #[rustfmt::skip]
    fn tile_flip(&self) -> Flip {
        match self {
            Profile::Famicom     (profile) => profile.tile_flip(),
            Profile::SuperFamicom(profile) => profile.tile_flip(),
            Profile::GameBoy     (profile) => profile.tile_flip(),
            Profile::GameBoyColor(profile) => profile.tile_flip(),
            Profile::VirtualBoy  (profile) => profile.tile_flip(),
            Profile::PcEngine    (profile) => profile.tile_flip(),
            Profile::WonderSwan  (profile) => profile.tile_flip(),
            Profile::MasterSystem(profile) => profile.tile_flip(),
            Profile::MegaDrive   (profile) => profile.tile_flip(),
            Profile::NeoGeoPocket(profile) => profile.tile_flip(),
            Profile::NeoGeo      (profile) => profile.tile_flip(),
        }
    }
}

/// Implement the traits for each hardware profile
macro_rules! impl_traits {
    ($profile:ty) => {
        impl GetTileSize for $profile {
            #[inline]
            fn tile_size(&self) -> TileSize {
                self.mode.tile_size()
            }
        }

        impl GetTileFlip for $profile {
            #[inline]
            fn tile_flip(&self) -> Flip {
                self.mode.tile_flip()
            }
        }
    };
}

/// Implement flip trait
macro_rules! impl_flip {
    ($type:ty => $flip:ident) => {
        impl GetTileFlip for $type {
            #[inline]
            fn tile_flip(&self) -> Flip {
                Flip::$flip
            }
        }
    };
}

// MARK: Nintendo

/// Famicom Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileFamicom {
    /// Specify bitplane mode
    pub bitplane: BitplaneFamicom,

    /// Specify background or foreground mode
    pub mode: BgFg<Const<false>, SpriteNintendo1>,
}

impl_traits!(ProfileFamicom);

/// Define the serialization options available for the Famicom
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum BitplaneFamicom {
    /// Use 1-bit per pixel
    #[strum(serialize = "bpp1")]
    Bpp1,

    /// Use 2-bits per pixel
    #[default]
    #[strum(serialize = "bpp2")]
    Bpp2,
}

// <TODO>
// The SNES supports two sprites size at the same time.
// But then it will be really hard to design an algorithm
// that will pick the most appropriate sprite size.
// This will require a custom variation of the algorithm
// just for the SNES.
// </TODO>

/// Super Famicom Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileSuperFamicom {
    /// Specify bitplane mode
    pub bitplane: BitplaneSuperFamicom,

    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteNintendo2>,
}

impl_traits!(ProfileSuperFamicom);

/// Define the serialization options available for the Super Famicom
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum BitplaneSuperFamicom {
    /// Use 2-bits per pixel
    #[strum(serialize = "bpp2")]
    Bpp2,

    /// Use 4-bits per pixel
    #[default]
    #[strum(serialize = "bpp4")]
    Bpp4,

    /// Use 8-bits per pixel
    #[strum(serialize = "bpp8")]
    Bpp8,

    /// Mode7 serialization
    #[strum(serialize = "mode7")]
    Mode7,
}

/// Game Boy Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileGameBoy {
    /// Specify background or foreground mode
    pub mode: BgFg<Const<false>, SpriteNintendo1>,
}

impl_traits!(ProfileGameBoy);

/// Game Boy Color Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileGameBoyColor {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteNintendo1>,
}

impl_traits!(ProfileGameBoyColor);

/// Virtual Boy Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileVirtualBoy {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteNintendo2>,
}

impl_traits!(ProfileVirtualBoy);

/// Sprite modes supported by the Famicom / GameBoy
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum SpriteNintendo1 {
    /// Small 8x8 sprite
    #[default]
    #[strum(serialize = "8x8")]
    S8x8,

    /// Larger 8x16 sprite
    #[strum(serialize = "8x16")]
    S8x16,
}

impl GetTileSize for SpriteNintendo1 {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8  => TileSize::new(8,  8),
            Self::S8x16 => TileSize::new(8, 16),
        }
    }
}

impl_flip!(SpriteNintendo1 => Both);

/// Sprite modes supported by the Super Famicom / Virtual Boy
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum SpriteNintendo2 {
    /// 8x8 sprite
    #[default]
    #[strum(serialize = "8x8")]
    S8x8,

    /// 16x16 sprite
    #[strum(serialize = "16x16")]
    S16x16,

    /// 32x32 sprite
    #[strum(serialize = "32x32")]
    S32x32,

    /// 64x64 sprite
    #[strum(serialize = "64x64")]
    S64x64,
}

impl GetTileSize for SpriteNintendo2 {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8   => TileSize::new( 8,  8),
            Self::S16x16 => TileSize::new(16, 16),
            Self::S32x32 => TileSize::new(32, 32),
            Self::S64x64 => TileSize::new(64, 64),
        }
    }
}

impl_flip!(SpriteNintendo2 => Both);

// MARK: NEC

/// PC-Engine Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfilePcEngine {
    /*
     * Bitplane encoding vary between background and foreground tiles for the PC-Engine.
     */
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpritePcEngine>,
}

impl_traits!(ProfilePcEngine);

/// Sprite modes supported by the PC-Engine
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum SpritePcEngine {
    /// 16x16 sprite
    #[default]
    #[strum(serialize = "16x16")]
    S16x16,

    /// 16x32 sprite
    #[strum(serialize = "16x32")]
    S16x32,

    /// 16x48 sprite
    #[strum(serialize = "16x48")]
    S16x48,

    /// 16x64 sprite
    #[strum(serialize = "16x64")]
    S16x64,

    /// 32x16 sprite
    #[strum(serialize = "32x16")]
    S32x16,

    /// 32x32 sprite
    #[strum(serialize = "32x32")]
    S32x32,

    /// 32x48 sprite
    #[strum(serialize = "32x48")]
    S32x48,

    /// 32x64 sprite
    #[strum(serialize = "32x64")]
    S32x64,
}

impl GetTileSize for SpritePcEngine {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S16x16 => TileSize::new(16, 16),
            Self::S16x32 => TileSize::new(16, 32),
            Self::S16x48 => TileSize::new(16, 48),
            Self::S16x64 => TileSize::new(16, 64),
            Self::S32x16 => TileSize::new(32, 16),
            Self::S32x32 => TileSize::new(32, 32),
            Self::S32x48 => TileSize::new(32, 48),
            Self::S32x64 => TileSize::new(32, 64),
        }
    }
}

impl_flip!(SpritePcEngine => Both);

// MARK: Bandai

/// WonderSwan Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileWonderSwan {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, Const<true, 8>>,
}

impl_traits!(ProfileWonderSwan);

// MARK: SEGA

/// Master System Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileMasterSystem {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteSegaMS>,
}

impl_traits!(ProfileMasterSystem);

/// Sprite modes supported by the SEGA Master System
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum SpriteSegaMS {
    /// Small 8x8 sprite
    #[default]
    #[strum(serialize = "8x8")]
    S8x8,

    /// Larger 8x16 sprite
    #[strum(serialize = "8x16")]
    S8x16,
}

impl GetTileSize for SpriteSegaMS {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8  => TileSize::new(8,  8),
            Self::S8x16 => TileSize::new(8, 16),
        }
    }
}

impl_flip!(SpriteSegaMS => None);

/// MegaDrive Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileMegaDrive {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteSegaMD>,
}

impl_traits!(ProfileMegaDrive);

/// Sprite modes supported by the SEGA MegaDrive
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum SpriteSegaMD {
    /// 8x8 sprite
    #[default]
    #[strum(serialize = "8x8")]
    S8x8,

    /// 16x16 sprite
    #[strum(serialize = "16x16")]
    S16x16,

    /// 24x24 sprite
    #[strum(serialize = "24x24")]
    S24x24,

    /// 32x32 sprite
    #[strum(serialize = "32x32")]
    S32x32,
}

impl GetTileSize for SpriteSegaMD {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8   => TileSize::new( 8,  8),
            Self::S16x16 => TileSize::new(16, 16),
            Self::S24x24 => TileSize::new(24, 24),
            Self::S32x32 => TileSize::new(32, 32),
        }
    }
}

impl_flip!(SpriteSegaMD => Both);

// MARK: SNK

/// NeoGeo Pocket Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileNeoGeoPocket {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteNeoGeoPocket>,
}

impl_traits!(ProfileNeoGeoPocket);

/// Sprite modes supported by the NeoGeo Pocket
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum SpriteNeoGeoPocket {
    /// 8x8 sprite
    #[default]
    #[strum(serialize = "8x8")]
    S8x8,

    /// 8x16 sprite
    #[strum(serialize = "8x16")]
    S8x16,

    /// 8x32 sprite
    #[strum(serialize = "8x32")]
    S8x32,

    /// 16x8 sprite
    #[strum(serialize = "16x8")]
    S16x8,

    /// 16x16 sprite
    #[strum(serialize = "16x16")]
    S16x16,

    /// 16x32 sprite
    #[strum(serialize = "16x32")]
    S16x32,

    /// 32x8 sprite
    #[strum(serialize = "32x8")]
    S32x8,

    /// 32x16 sprite
    #[strum(serialize = "32x16")]
    S32x16,

    /// 32x32 sprite
    #[strum(serialize = "32x32")]
    S32x32,
}

impl GetTileSize for SpriteNeoGeoPocket {
    #[rustfmt::skip]
    fn tile_size(&self) -> TileSize {
        match self {
            Self::S8x8   => TileSize::new( 8,  8),
            Self::S8x16  => TileSize::new( 8, 16),
            Self::S8x32  => TileSize::new( 8, 16),
            Self::S16x8  => TileSize::new(16,  8),
            Self::S16x16 => TileSize::new(16, 16),
            Self::S16x32 => TileSize::new(16, 32),
            Self::S32x8  => TileSize::new(32,  8),
            Self::S32x16 => TileSize::new(32, 16),
            Self::S32x32 => TileSize::new(32, 32),
        }
    }
}

impl_flip!(SpriteNeoGeoPocket => Both);

/// NeoGeo Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileNeoGeo {
    /// NeoGeo only supports sprites
    pub mode: Const<true, 16>,
}

impl_traits!(ProfileNeoGeo);

/// Specify if we are processing background or foreground tiles
#[derive(Debug, Clone, Copy)]
pub enum BgFg<Bg, Fg> {
    /// Background data
    Bg(Bg),

    /// Foreground data
    Fg(Fg),
}

impl<Bg: Default, Fg> BgFg<Bg, Fg> {
    #[inline]
    fn default_bg() -> Self {
        Self::Bg(Default::default())
    }
}

impl<Bg, Fg: Default> BgFg<Bg, Fg> {
    #[inline]
    fn default_fg() -> Self {
        Self::Fg(Default::default())
    }
}

impl<Bg: GetTileSize, Fg: GetTileSize> GetTileSize for BgFg<Bg, Fg> {
    #[inline]
    fn tile_size(&self) -> TileSize {
        match self {
            BgFg::Bg(bg) => bg.tile_size(),
            BgFg::Fg(fg) => fg.tile_size(),
        }
    }
}

impl<Bg: GetTileFlip, Fg: GetTileFlip> GetTileFlip for BgFg<Bg, Fg> {
    #[inline]
    fn tile_flip(&self) -> Flip {
        match self {
            BgFg::Bg(bg) => bg.tile_flip(),
            BgFg::Fg(fg) => fg.tile_flip(),
        }
    }
}

/// Non-configurable tile size and mode.
/// Used for hardware which only supports one type of tile.
#[derive(Debug, Default, Clone, Copy)]
pub struct Const<const FLIP: bool = true, const SIZE: u32 = 8>;

impl<const FLIP: bool, const SIZE: u32> GetTileSize for Const<FLIP, SIZE> {
    #[inline]
    fn tile_size(&self) -> TileSize {
        TileSize::new(SIZE, SIZE)
    }
}

impl<const FLIP: bool, const SIZE: u32> GetTileFlip for Const<FLIP, SIZE> {
    #[inline]
    fn tile_flip(&self) -> Flip {
        if FLIP { Flip::Both } else { Flip::None }
    }
}

// MARK: Parse

/// Parse a sprite size format
#[inline]
fn parse_sprite_size<Bg, Fg>(sprite_size: Option<&str>) -> Result<BgFg<Bg, Fg>, Fg::Err>
where
    Fg: Default + FromStr,
{
    if let Some(sprite_size) = sprite_size {
        Ok(BgFg::Fg(Fg::from_str(sprite_size)?))
    } else {
        Ok(BgFg::default_fg())
    }
}

/// Parse a bit plane format
#[inline]
fn parse_bitplane<S>(bitplane: Option<&str>) -> Result<S, S::Err>
where
    S: Default + FromStr,
{
    if let Some(bpp) = bitplane {
        S::from_str(bpp)
    } else {
        Ok(S::default())
    }
}
