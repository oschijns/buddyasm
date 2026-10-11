use image::{Rgb, Rgba};
use regex::Regex;
use std::{num::ParseIntError, sync::LazyLock};

/// Error encountered when reading color data from the configuration
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ColorError {
    /// Pattern is not an hexadecimal string
    #[error("Could not recognize color from \"{0}\" pattern")]
    UnexpectedPattern(String),

    /// Failed to parse the value as an integer
    #[error("Failed to parse the hexadecimal value")]
    ParseInt(#[from] ParseIntError),
}

/// Regex for identifying colors encoded as hexadecimal strings
static RGB_HEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^#([0-9A-Fa-f]{2}){3}$").unwrap());

/// Try to identify a color from an hexadecimal string
pub fn rgb_from_hex(hex: &str) -> Result<Rgb<u8>, ColorError> {
    if RGB_HEX.is_match(hex) {
        // trim the first '#'
        let hex = hex[1..].to_string();

        // Parse the whole value
        let [_, r, g, b] = u32::from_str_radix(&hex, 16)?.to_be_bytes();

        // Convert the bytes into floats
        Ok(Rgb([r, g, b]))
    } else {
        Err(ColorError::UnexpectedPattern(hex.to_string()))
    }
}

/// Regex for identifying colors encoded as hexadecimal strings
static RGBA_HEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#([0-9A-Fa-f]{2}){3,4}$").unwrap());

/// Try to identify a color from an hexadecimal string
pub fn rgba_from_hex(hex: &str) -> Result<Rgba<u8>, ColorError> {
    if RGBA_HEX.is_match(hex) {
        // trim the first '#'
        let mut hex = hex[1..].to_string();

        // Check if the alpha channel is defined in the string.
        // If not add two extra digits so that the size is always 8.
        if hex.len() == 6 {
            hex += "FF";
        }

        // Parse the whole value
        let [r, g, b, a] = u32::from_str_radix(&hex, 16)?.to_be_bytes();

        // Convert the bytes into floats
        Ok(Rgba([r, g, b, a]))
    } else {
        Err(ColorError::UnexpectedPattern(hex.to_string()))
    }
}
