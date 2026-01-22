use super::{InputImage, InputStack};
use crate::tileset::{PaletteSetRgba, builder::BuilderConfig};
use asefile::AsepriteFile;
use image::{DynamicImage, RgbaImage};
use std::path::Path;

impl InputStack {
    /// Create a new stack from the provided data
    #[inline]
    pub fn new(config: BuilderConfig, palette: PaletteSetRgba, capacity: usize) -> Self {
        Self {
            config,
            palette,
            stack: Vec::with_capacity(capacity),
        }
    }

    /// Add a static image to the stack
    pub fn add(&mut self, path: &Path, image: InputImage, palette: Option<PaletteSetRgba>) {
        // Pick the palette to use
        let palette = palette.unwrap_or_else(|| self.palette.clone());

        // Add the image to the stack
        self.stack.push((path.to_path_buf(), image, palette));
    }
}

/// Convert a simple image
impl From<RgbaImage> for InputImage {
    #[inline]
    fn from(value: RgbaImage) -> Self {
        Self::Static(value)
    }
}

/// Convert a simple image
impl From<DynamicImage> for InputImage {
    #[inline]
    fn from(value: DynamicImage) -> Self {
        Self::Static(value.to_rgba8())
    }
}

/// Convert a Aseprite file
impl From<AsepriteFile> for InputImage {
    #[inline]
    fn from(value: AsepriteFile) -> Self {
        Self::Animated(Box::new(value))
    }
}
