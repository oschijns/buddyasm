/// Simple operations
mod base;

/// Process a stack
mod process;

use crate::tileset::{IndexMap, PaletteSetRgba, TileSet, builder::BuilderConfig};
use asefile::AsepriteFile;
use core::error::Error;
use image::RgbaImage;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Define a stack of data to process
#[derive(Debug)]
pub struct InputStack {
    /// Configuration of the tileset to build
    config: BuilderConfig,

    /// Default palette to use if one is not explicitely set
    palette: PaletteSetRgba,

    /// Stack of images to process
    stack: Vec<(PathBuf, InputImage, PaletteSetRgba)>,
}

/// Define one element to process
/// Either a static image or an animated sprite
#[derive(Debug)]
pub enum InputImage {
    /// static image
    Static(RgbaImage),

    /// Animated image from Aseprite
    Animated(Box<AsepriteFile>),
}

/// TileSet generated and associated index maps
#[derive(Debug)]
pub struct OutputStack {
    /// Generated tileset
    tileset: TileSet,

    /// Associated index maps
    images: HashMap<PathBuf, OutputImage>,
}

/// Result of processing a single image from the stack
#[derive(Debug, Clone)]
pub enum OutputImage {
    /// Output a single static image
    Static(IndexMap),

    /// Output an animated image
    Animated(HashMap<String, OutputAnimation>),
}

/// Store the data to reconstruct an animated sprite
#[derive(Debug, Clone)]
pub enum OutputAnimation {
    /// Produce a single animation
    Normal(IndexMap),

    /// Produce left and right variants of the animation
    LeftRight {
        /// Left variant of the animation
        left: IndexMap,

        /// RIght variant of the animation
        right: IndexMap,
    },

    /// Produce up and down variants of the animation
    UpDown {
        /// Up variant of the animation
        up: IndexMap,

        /// Down variant of the animation
        down: IndexMap,
    },

    /// Produce variants for all four directions
    FourWay {
        /// Left variant of the animation
        left: IndexMap,

        /// RIght variant of the animation
        right: IndexMap,

        /// Up variant of the animation
        up: IndexMap,

        /// Down variant of the animation
        down: IndexMap,
    },
}

pub fn test() -> Result<(), Box<dyn Error>> {
    let anim = asefile::AsepriteFile::read_file(Path::new("test"))?;

    let tag = anim.get_tag(0).unwrap();
    let frame = anim.frame(0).image();

    //DynamicImage::ImageLuma16(())
    //GrayImage::from(frame);

    let dir = tag.animation_direction();

    Ok(())
}
