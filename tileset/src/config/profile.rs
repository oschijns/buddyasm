//! Define hardware profiles

use crate::config::{
    input::BuilderConfig,
    tile::{
        FixedTile, ParseError, SpriteNeoGeoPocket, SpriteNintendo1, SpriteNintendo2,
        SpritePcEngine, SpriteSegaMD, SpriteSegaMS, TileConfig, TileOrSprite,
    },
};
use serde::Deserialize;

/// Get a builder configuration from the selected hardware profile
pub trait ToConfig {
    fn to_config(&self) -> BuilderConfig;
}

/// List all hardware profiles available
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Hardware {
    /// Famicom / NES
    #[serde(alias = "nes")]
    Famicom,

    /// Super Famicom / SNES
    #[serde(alias = "snes")]
    SuperFamicom,

    /// Game Boy
    #[serde(alias = "gameboy", alias = "gb")]
    GameBoy,

    /// Game Boy Color
    #[serde(alias = "gameboy-color", alias = "gbc")]
    GameBoyColor,

    /// Virtual Boy
    #[serde(alias = "virtualboy", alias = "vb")]
    VirtualBoy,

    /// PC-Engine
    #[serde(alias = "pc-engine", alias = "pce")]
    PcEngine,

    /// Wonder Swan
    #[serde(alias = "wonderswan", alias = "ws")]
    WonderSwan,

    /// Master System
    #[serde(alias = "mastersystem", alias = "sms")]
    MasterSystem,

    /// MegaDrive / Genesis
    #[serde(alias = "megadrive", alias = "md")]
    MegaDrive,

    /// NeoGeo Pocket
    #[serde(alias = "neogeo-pocket", alias = "ngp")]
    NeoGeoPocket,

    /// NeoGeo
    #[serde(alias = "neogeo", alias = "ng")]
    NeoGeo,
}

/// Are we generating background tiles or foreground tiles (sprites)
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum TileKind {
    /// Background tiles
    #[serde(alias = "bg", alias = "nametable")]
    Background,

    /// Foreground tiles
    #[serde(alias = "fg", alias = "sprite")]
    Foreground,
}

/// List all hardware profiles available
#[derive(Debug, Clone)]
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

impl Profile {
    /// Build a profile given the parameters
    #[rustfmt::skip]
    pub fn new(
        hardware: Hardware,
        kind: TileKind,
        sprite_size: Option<&str>,
    ) -> Result<Self, ParseError> {
        match kind {
            TileKind::Background => match hardware {
                Hardware::Famicom      => Ok(Self::Famicom     (ProfileFamicom     (TileOrSprite::default_tile()))),
                Hardware::SuperFamicom => Ok(Self::SuperFamicom(ProfileSuperFamicom(TileOrSprite::default_tile()))),
                Hardware::GameBoy      => Ok(Self::GameBoy     (ProfileGameBoy     (TileOrSprite::default_tile()))),
                Hardware::GameBoyColor => Ok(Self::GameBoyColor(ProfileGameBoyColor(TileOrSprite::default_tile()))),
                Hardware::VirtualBoy   => Ok(Self::VirtualBoy  (ProfileVirtualBoy  (TileOrSprite::default_tile()))),
                Hardware::PcEngine     => Ok(Self::PcEngine    (ProfilePcEngine    (TileOrSprite::default_tile()))),
                Hardware::WonderSwan   => Ok(Self::WonderSwan  (ProfileWonderSwan  (TileOrSprite::default_tile()))),
                Hardware::MasterSystem => Ok(Self::MasterSystem(ProfileMasterSystem(TileOrSprite::default_tile()))),
                Hardware::MegaDrive    => Ok(Self::MegaDrive   (ProfileMegaDrive   (TileOrSprite::default_tile()))),
                Hardware::NeoGeoPocket => Ok(Self::NeoGeoPocket(ProfileNeoGeoPocket(TileOrSprite::default_tile()))),
                Hardware::NeoGeo       => Ok(Self::NeoGeo      (ProfileNeoGeo      (TileOrSprite::default_tile()))),
            },
            TileKind::Foreground => match hardware {
                Hardware::Famicom      => Ok(Self::Famicom     (ProfileFamicom     (TileOrSprite::sprite(sprite_size)?))),
                Hardware::SuperFamicom => Ok(Self::SuperFamicom(ProfileSuperFamicom(TileOrSprite::sprite(sprite_size)?))),
                Hardware::GameBoy      => Ok(Self::GameBoy     (ProfileGameBoy     (TileOrSprite::sprite(sprite_size)?))),
                Hardware::GameBoyColor => Ok(Self::GameBoyColor(ProfileGameBoyColor(TileOrSprite::sprite(sprite_size)?))),
                Hardware::VirtualBoy   => Ok(Self::VirtualBoy  (ProfileVirtualBoy  (TileOrSprite::sprite(sprite_size)?))),
                Hardware::PcEngine     => Ok(Self::PcEngine    (ProfilePcEngine    (TileOrSprite::sprite(sprite_size)?))),
                Hardware::WonderSwan   => Ok(Self::WonderSwan  (ProfileWonderSwan  (TileOrSprite::default_sprite()))),
                Hardware::MasterSystem => Ok(Self::MasterSystem(ProfileMasterSystem(TileOrSprite::sprite(sprite_size)?))),
                Hardware::MegaDrive    => Ok(Self::MegaDrive   (ProfileMegaDrive   (TileOrSprite::sprite(sprite_size)?))),
                Hardware::NeoGeoPocket => Ok(Self::NeoGeoPocket(ProfileNeoGeoPocket(TileOrSprite::sprite(sprite_size)?))),
                Hardware::NeoGeo       => Ok(Self::NeoGeo      (ProfileNeoGeo      (TileOrSprite::default_sprite()))),
            },
        }
    }
}

/// Famicom Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileFamicom(pub(crate) TileOrSprite<FixedTile<false>, SpriteNintendo1>);

// <TODO>
// The SNES supports two sprites size at the same time.
// But then it will be really hard to design an algorithm
// that will pick the most appropriate sprite size.
// This will require a custom variation of the algorithm
// just for the SNES.
// </TODO>

/// Super Famicom Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileSuperFamicom(pub(crate) TileOrSprite<FixedTile, SpriteNintendo2>);

/// Game Boy Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileGameBoy(pub(crate) TileOrSprite<FixedTile<false>, SpriteNintendo1>);

/// Game Boy Color Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileGameBoyColor(pub(crate) TileOrSprite<FixedTile, SpriteNintendo1>);

/// Virtual Boy Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileVirtualBoy(pub(crate) TileOrSprite<FixedTile, SpriteNintendo2>);

/// PC-Engine Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfilePcEngine(pub(crate) TileOrSprite<FixedTile, SpritePcEngine>);

/// WonderSwan Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileWonderSwan(pub(crate) TileOrSprite<FixedTile, FixedTile<true, 8>>);

/// Master System Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileMasterSystem(pub(crate) TileOrSprite<FixedTile, SpriteSegaMS>);

/// MegaDrive Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileMegaDrive(pub(crate) TileOrSprite<FixedTile, SpriteSegaMD>);

/// NeoGeo Pocket Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileNeoGeoPocket(pub(crate) TileOrSprite<FixedTile, SpriteNeoGeoPocket>);

/// NeoGeo Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileNeoGeo(pub(crate) TileOrSprite<FixedTile<true, 16>, FixedTile<true, 16>>);

impl ToConfig for Profile {
    #[rustfmt::skip]
    fn to_config(&self) -> BuilderConfig {
        match self {
            Self::Famicom     (profile) => profile.to_config(),
            Self::SuperFamicom(profile) => profile.to_config(),
            Self::GameBoy     (profile) => profile.to_config(),
            Self::GameBoyColor(profile) => profile.to_config(),
            Self::VirtualBoy  (profile) => profile.to_config(),
            Self::PcEngine    (profile) => profile.to_config(),
            Self::WonderSwan  (profile) => profile.to_config(),
            Self::MasterSystem(profile) => profile.to_config(),
            Self::MegaDrive   (profile) => profile.to_config(),
            Self::NeoGeoPocket(profile) => profile.to_config(),
            Self::NeoGeo      (profile) => profile.to_config(),
        }
    }
}

/// Famicom Config
impl ToConfig for ProfileFamicom {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// Super Famicom Config
impl ToConfig for ProfileSuperFamicom {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// Game Boy Config
impl ToConfig for ProfileGameBoy {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// Game Boy Color Config
impl ToConfig for ProfileGameBoyColor {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// Virtual Boy Config
impl ToConfig for ProfileVirtualBoy {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// PC-Engine Config
impl ToConfig for ProfilePcEngine {
    fn to_config(&self) -> BuilderConfig {
        match self.0 {
            TileOrSprite::Tile(_) => build_config(&self.0),
            TileOrSprite::Sprite(_) => build_config_params(&self.0, 16, 256),
        }
    }
}

/// WonderSwan Config
impl ToConfig for ProfileWonderSwan {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// Master System Config
impl ToConfig for ProfileMasterSystem {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// MegaDrive Config
impl ToConfig for ProfileMegaDrive {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// NeoGeo Pocket Config
impl ToConfig for ProfileNeoGeoPocket {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// NeoGeo Config
impl ToConfig for ProfileNeoGeo {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// Build a config from the provided tile config and some standard values
fn build_config<T>(tile_config: &T) -> BuilderConfig
where
    T: TileConfig,
{
    // base tile configuration is 256 8x8 tiles
    const TILE_COUNT: usize = 256;
    const TILE_SIZE: u32 = 8;

    build_config_params(tile_config, TILE_SIZE, TILE_COUNT)
}

/// Build a config from the provided tile config and some standard values
fn build_config_params<T>(tile_config: &T, tile_size: u32, tile_count: usize) -> BuilderConfig
where
    T: TileConfig,
{
    let [flip_h, flip_v] = tile_config.flipping();
    let [px, py] = tile_config.tile_size().0;
    let tx = (px / tile_size) as usize;
    let ty = (py / tile_size) as usize;

    BuilderConfig::new(tile_count / (tx * ty), px, py, flip_h, flip_v)
}
