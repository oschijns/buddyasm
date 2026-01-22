use crate::tileset::builder::BuilderConfig;
use serde::Deserialize;
use std::path::PathBuf;

/// Configuration of the input stack to process
#[derive(Debug, Clone, Deserialize)]
pub struct InputStackConfig {
    /// Tileset configuration
    config: BuilderConfig,

    /// Default palette to use
    default_palette: PathBuf,

    /// Entries to process
    #[serde(flatten)]
    entries: Vec<Entry>,
}

/// An input image to load
#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// Image to process
    image: PathBuf,

    /// Optional palette override
    #[serde(default)]
    palette: Option<PathBuf>,

    /// Optional fixed mapping
    #[serde(default)]
    mapping: Option<Vec<Rule>>,
}

/// Map a tile from the input image to a specific index
#[derive(Debug, Clone)]
pub struct Rule {
    /// X-coordinate of the tile
    tx: u32,

    /// Y-coordinate of the tile
    ty: u32,

    /// Target index of the tile
    index: usize,
}

/// Custom deserialization rule so that rules can be written as
/// `"(x, y) -> i"`
impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
