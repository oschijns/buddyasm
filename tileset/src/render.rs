//! Utility functions for rendering output images using Tera templates.

use crate::config::output::OutputImage;
use tera::{Context, Tera};

/// Converts an [`OutputImage`] to a [`Context`] for use with Tera templates.
pub fn output_to_context(output: &OutputImage) -> Context {
    let mut context = Context::new();

    match output {
        OutputImage::Static(map) => {
            context.insert("tiles", map);
        }
        OutputImage::Animated(animations) => {
            context.insert("animations", animations);
        }
    }
    context
}
