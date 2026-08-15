//! Flip

use serde::{Deserialize, Serialize};

/// Flipping axes
#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Flip {
    /// No flip
    #[default]
    None = 0b00,

    /// Horizontal flip
    Horizontal = 0b01,

    /// Vertical flip
    Vertical = 0b10,

    /// Flip horizontally and vertically
    Both = 0b11,
}

impl Flip {
    /// Return true if the tile is flipped horizontally
    #[inline]
    pub const fn horizontal(self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }

    /// Return true if the tile is flipped vertically
    #[inline]
    pub const fn vertical(self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
    }
}

/// Error encountered when parsing a flipping configuration
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[error("Could not parse \"{0}\" as a flipping configuration.")]
pub struct FlipParseError(String);

impl Flip {
    /// Parse the provided string to deduce a flipping configuration
    #[rustfmt::skip]
    pub fn parse(text: &str) -> Result<Self, FlipParseError> {
        let lower = text.to_ascii_lowercase();
        match lower.as_str() {
            "none"       | "null" | "nil" => Ok(Self::None),
            "horizontal" | "h"    | "x"   => Ok(Self::Horizontal),
            "vertical"   | "v"    | "y"   => Ok(Self::Vertical),
            "both"       | "hv"   | "xy"  => Ok(Self::Both),
            _ => Err(FlipParseError(text.to_string())),
        }
    }
}
