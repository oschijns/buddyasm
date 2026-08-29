use strum::EnumString;

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

// MARK: Nintendo

/// Famicom Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileFamicom {
    /// Specify bitplane mode
    pub bitplane: BitplaneFamicom,

    /// Specify background or foreground mode
    pub mode: BgFg<Const<false>, SpriteNintendo1>,
}

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

/// Game Boy Color Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileGameBoyColor {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteNintendo1>,
}

/// Virtual Boy Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileVirtualBoy {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteNintendo2>,
}

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

// MARK: Bandai

/// WonderSwan Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileWonderSwan {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, Const<true, 8>>,
}

// MARK: SEGA

/// Master System Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileMasterSystem {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteSegaMS>,
}

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

/// MegaDrive Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileMegaDrive {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteSegaMD>,
}

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

// MARK: SNK

/// NeoGeo Pocket Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileNeoGeoPocket {
    /// Specify background or foreground mode
    pub mode: BgFg<Const, SpriteNeoGeoPocket>,
}

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

/// NeoGeo Profile
#[derive(Debug, Clone, Copy)]
pub struct ProfileNeoGeo {
    /// NeoGeo only supports sprites
    pub mode: Const<true, 16>,
}

/// Specify if we are processing background or foreground tiles
#[derive(Debug, Clone, Copy)]
pub enum BgFg<Bg, Fg> {
    /// Background data
    Bg(Bg),

    /// Foreground data
    Fg(Fg),
}

/// Non-configurable tile size and mode.
/// Used for hardware which only supports one type of tile.
#[derive(Debug, Default, Clone, Copy)]
pub struct Const<const FLIP: bool = true, const SIZE: u32 = 8>;
