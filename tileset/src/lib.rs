//! Process pictures to generate a unified tileset

pub mod config;
pub mod data;
pub mod process;
pub mod render;
pub mod serial;

pub mod prelude {
    // re-export common types

    pub use crate::config::{
        input::{BuilderConfig, InputImage, InputStack},
        manifest::{Entry, InputError, MapRange, TileSetManifest},
    };
    pub use crate::data::{
        builder::{TileError, TileMapBuilder},
        coords::{Coords, Dimensions, TileSize, Vec2},
        flip::{Flip, FlipParseError},
        mapping::Mapping,
        palette::{
            NoPaletteMatchError, PaletteSet, PaletteSetLuma, PaletteSetLumaA, PaletteSetRgb,
            PaletteSetRgba,
        },
        tile::{Pix, Tile, TileSet},
    };
    pub use crate::process::output::{
        IndexMap, OutTile, OutputAnimation, OutputImage, OutputStack,
    };
}
