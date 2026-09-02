use serde::Deserialize;

/// List all hardware profiles available
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum System {
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

    /// Game Boy Advance
    #[serde(alias = "gameboy-advance", alias = "gba")]
    GameBoyAdvance,

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
