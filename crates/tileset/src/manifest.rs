use buddyasm_common::system::System;
use minijinja::syntax::SyntaxConfig;
use serde::Deserialize;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use crate::manifest::default::{PREFIX_BIN, PREFIX_HEX, PREFIX_OCT};

/// Configuration of the input stack to process
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    /// Absolute path to this manifest file
    #[serde(default)]
    path: PathBuf,

    /// Main configuration
    #[serde(alias = "configuration")]
    pub config: Config,

    /// Templating configuration
    #[serde(alias = "template")]
    pub templating: Option<Templating>,

    /// Entries to process
    #[serde(rename = "entry")]
    pub entries: Vec<ManifestEntry>,
}

impl buddyasm_common::manifest::Manifest for Manifest {
    /// Path to the directory containing this manifest file
    fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    /// Path to the directory containing this manifest file
    #[inline]
    fn get_path(&self) -> &Path {
        &self.path
    }
}

/// Configure main components such as default palette and target hardware
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Target system
    pub system: System,

    /// Type of tiles to generate
    pub kind: TileKind,

    /// Sprite size parameter (if necessary)
    #[serde(default)]
    pub sprite_size: Option<String>,

    /// Bit plane configuration (if necessary)
    #[serde(default)]
    pub bit_plane: Option<String>,

    /// Default palette to use
    #[serde(default)]
    pub default_palette: Option<PathBuf>,

    /// Path to the output directory relative to the manifest file.
    /// If not specified, defaults to `output` next to the manifest file.
    #[serde(default)]
    pub output: Option<PathBuf>,

    /// Path to the CHR file.
    /// If not specified, defaults to `tileset.chr` in the output directory.
    #[serde(default)]
    pub file_chr: Option<PathBuf>,
}

/// Templating configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Templating {
    /// List of templates includes if any.
    #[serde(default, alias = "include")]
    pub includes: Option<PathBuf>,

    /// Prefix for binary literals
    /// Defaults to `0b`
    #[serde(default = "default::prefix_bin")]
    pub prefix_bin: String,

    /// Prefix for octal literals
    /// Defaults to `0o`
    #[serde(default = "default::prefix_oct")]
    pub prefix_oct: String,

    /// Prefix for hexadecimal literals
    /// Defaults to `0x`
    #[serde(default = "default::prefix_hex")]
    pub prefix_hex: String,

    /// Override delimiter for template blocks
    /// Defaults to `{%%}`
    #[serde(default)]
    pub delimiter_block: Option<[String; 2]>,

    /// Override delimiter for template variables
    /// Defaults to `{{}}`
    #[serde(default)]
    pub delimiter_variable: Option<[String; 2]>,

    /// Override delimiter for template comments
    /// Defaults to `{##}`
    #[serde(default)]
    pub delimiter_comment: Option<[String; 2]>,

    /// Enable line statement using the provided prefix
    #[serde(default)]
    pub prefix_statement: Option<String>,

    /// Enable line comment using the provided prefix
    #[serde(default)]
    pub prefix_comment: Option<String>,
}

/// Build the default config for templates
impl Default for Templating {
    fn default() -> Self {
        Self {
            includes: None,
            prefix_bin: PREFIX_BIN.to_string(),
            prefix_oct: PREFIX_OCT.to_string(),
            prefix_hex: PREFIX_HEX.to_string(),
            delimiter_block: None,
            delimiter_variable: None,
            delimiter_comment: None,
            prefix_statement: None,
            prefix_comment: None,
        }
    }
}

impl Templating {
    /// Get overridden template delimiters
    pub fn get_syntax_config(&self) -> Result<SyntaxConfig, minijinja::Error> {
        let mut builder = SyntaxConfig::builder();

        // Jinja blocks
        if let Some([a, b]) = &self.delimiter_block {
            builder.block_delimiters(Cow::Owned(a.clone()), Cow::Owned(b.clone()));
        }
        if let Some([a, b]) = &self.delimiter_variable {
            builder.variable_delimiters(Cow::Owned(a.clone()), Cow::Owned(b.clone()));
        }
        if let Some([a, b]) = &self.delimiter_comment {
            builder.comment_delimiters(Cow::Owned(a.clone()), Cow::Owned(b.clone()));
        }

        // Jinja lines
        if let Some(a) = &self.prefix_statement {
            builder.line_statement_prefix(Cow::Owned(a.clone()));
        }
        if let Some(a) = &self.prefix_comment {
            builder.line_comment_prefix(Cow::Owned(a.clone()));
        }

        builder.build()
    }
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

/// An input image to load
#[derive(Debug, Clone, Deserialize)]
pub struct ManifestEntry {
    /// Image to process
    pub image: PathBuf,

    /// Name of the entry (used as the output file name)
    #[serde(default)]
    pub name: Option<String>,

    /// Optional palette override
    #[serde(default)]
    pub palette: Option<PathBuf>,

    /// Optional fixed mapping
    #[serde(default)]
    pub fixed_mapping: Vec<CharacterRange>,

    /// Specify if the entry should generate a JSON file.
    /// This is used to visualize the mapping of tiles to indices.
    /// Or to pass the mapping to an external tool.
    #[serde(default)]
    pub output_json: bool,

    /// Optional template file to use for this entry
    #[serde(default)]
    pub template: Option<PathBuf>,
}

impl ManifestEntry {
    /// Specify if this entry uses fixed mapping
    /// This is mostly used to encode character sets
    #[inline]
    pub fn has_fixed_mapping(&self) -> bool {
        !self.fixed_mapping.is_empty()
    }
}

/// Map a sequence of tiles from the input image to a specific index
#[derive(Debug, Clone, Deserialize)]
pub struct CharacterRange {
    /// First tile of the sequence to map to a target
    #[serde(default)]
    pub start: usize,

    /// Last tile of the sequence to map (excluded)
    pub end: usize,

    /// Target index of the tiles
    pub target: usize,
}

impl CharacterRange {
    /// Get the number of tiles covered by the map range
    #[inline]
    pub fn size(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

/// Set of default values
pub mod default {

    /// Default prefix for binary literals
    pub const PREFIX_BIN: &str = "0b";

    /// Default prefix for octal literals
    pub const PREFIX_OCT: &str = "0o";

    /// Default prefix for hexadecimal literals
    pub const PREFIX_HEX: &str = "0x";

    /// Prefix to use for binary notation
    #[inline]
    pub(crate) fn prefix_bin() -> String {
        PREFIX_BIN.to_string()
    }

    /// Prefix to use for octal notation
    #[inline]
    pub(crate) fn prefix_oct() -> String {
        PREFIX_OCT.to_string()
    }

    /// Prefix to use for hexadecimal notation
    #[inline]
    pub(crate) fn prefix_hex() -> String {
        PREFIX_HEX.to_string()
    }
}
