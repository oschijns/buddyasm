//! Utility functions for rendering output images using Tera templates.

use crate::{config::manifest::Entry, process::output::OutputImage};
use std::{collections::HashSet, io::Write, path::PathBuf};
use tera::{Context, Tera, TeraResult};

/// Builds a [`Tera`] context for generating source files using Tera templates.
pub fn build_renderer(includes: &[PathBuf], entries: &[Entry]) -> TeraResult<Tera> {
    // Add template files from the includes directory
    let mut tera = Tera::new();
    tera.add_template_files(includes.iter().map(|p| (p, Option::<&str>::None)))?;

    // Now check each entry for a template file to use.
    // If two entries have the same template file, add it only once.
    let mut temp_set = HashSet::with_capacity(entries.len());
    for entry in entries {
        if let Some(template) = &entry.template {
            let _ = temp_set.insert(template);
        }
    }
    for template in temp_set {
        tera.add_template_file(template, None)?;
    }

    Ok(tera)
}

/// Render the output image using the given Tera template and context.
pub fn render(
    tera: &Tera,
    template_name: &str,
    output_image: &OutputImage,
    write: impl Write,
) -> TeraResult<()> {
    let context = Context::from_serialize(output_image)?;
    tera.render_to(template_name, &context, write)
}
