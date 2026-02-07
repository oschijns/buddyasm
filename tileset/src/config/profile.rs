//! Define hardware profiles

use crate::config::{
    input::BuilderConfig,
    tile::{
        BgTile, SpriteNeoGeoPocket, SpriteNintendo1, SpriteNintendo2, SpritePcEngine, TileConfig,
        TileOrSprite,
    },
};
use serde::Deserialize;

/// Get a builder configuration from the selected hardware profile
pub trait ToConfig {
    fn to_config(&self) -> BuilderConfig;
}

/// List all hardware profiles available
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Profile {
    /// Famicom / NES
    Famicom(ProfileFamicom),

    /// Super Famicom / SNES
    SuperFamicom(ProfileSuperFamicom),

    /// Game Boy
    GameBoy(ProfileGameBoy),

    /// Virtual Boy
    VirtualBoy(ProfileVirtualBoy),

    /// PC-Engine
    PcEngine(ProfilePcEngine),

    /// NeoGeo Pocket
    NeoGeoPocket(ProfileNeoGeoPocket),

    /// Wonder Swan
    WonderSwan,

    /// Master System
    MasterSystem,

    /// MegaDrive / Genesis
    MegaDrive,
}

/// Famicom Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileFamicom(pub(crate) TileOrSprite<BgTile, SpriteNintendo1>);

// <TODO>
// The SNES supports two sprites size at the same time.
// But then it will be really hard to design an algorithm
// that will pick the most appropriate sprite size.
// This will require a custom variation of the algorithm
// just for the SNES.
// </TODO>

/// Super Famicom Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileSuperFamicom(pub(crate) TileOrSprite<BgTile<true>, SpriteNintendo2>);

/// Game Boy Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileGameBoy(pub(crate) TileOrSprite<BgTile, SpriteNintendo1>);

/// Virtual Boy Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileVirtualBoy(pub(crate) TileOrSprite<BgTile<true>, SpriteNintendo2>);

/// PC-Engine Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfilePcEngine(pub(crate) TileOrSprite<BgTile<true>, SpritePcEngine>);

/// NeoGeo Pocket Profile
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct ProfileNeoGeoPocket(pub(crate) TileOrSprite<BgTile<true>, SpriteNeoGeoPocket>);

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

/// Virtual Boy Config
impl ToConfig for ProfileVirtualBoy {
    fn to_config(&self) -> BuilderConfig {
        build_config(&self.0)
    }
}

/// PC-Engine Config
impl ToConfig for ProfilePcEngine {
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

/// Build a config from the provided tile config and some standard values
fn build_config<T>(tile_config: &T) -> BuilderConfig
where
    T: TileConfig,
{
    // base tile configuration is 256 8x8 tiles
    const TILE_COUNT: usize = 256;
    const TILE_SIZE: u32 = 8;

    let [flip_h, flip_v] = tile_config.flipping();
    let [px, py] = tile_config.tile_size().0;
    let tx = (px / TILE_SIZE) as usize;
    let ty = (py / TILE_SIZE) as usize;

    BuilderConfig::new(TILE_COUNT / (tx * ty), px, py, flip_h, flip_v)
}
