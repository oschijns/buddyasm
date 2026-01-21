use super::{InputImage, InputStack};
use crate::tileset::{PaletteSetRgba, builder::BuilderConfig};
use asefile::AsepriteFile;
use image::{Rgba, RgbaImage};
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
    pub fn add_static(&mut self, path: &Path, image: RgbaImage, palette: Option<PaletteSetRgba>) {
        // Pick the palette to use
        let palette = palette.unwrap_or_else(|| self.palette.clone());

        // Add the image to the stack
        self.stack
            .push((path.to_path_buf(), InputImage::Static(image), palette));
    }

    /// Add an animated image to the stack
    pub fn add_animated(
        &mut self,
        path: &Path,
        image: AsepriteFile,
        palette: Option<PaletteSetRgba>,
    ) {
        // Pick the palette to use
        let palette = palette.unwrap_or_else(|| self.palette.clone());

        // Add the image to the stack
        self.stack.push((
            path.to_path_buf(),
            InputImage::Animated(Box::new(image)),
            palette,
        ));
    }
}
