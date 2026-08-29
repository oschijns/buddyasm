//! Utility functions for rendering output images using Tera templates.

use crate::{
    config::{
        manifest::{Entry, PREFIX_BIN, PREFIX_HEX, PREFIX_OCT, Templating},
        profile::Profile,
    },
    process::output::OutputImage,
};
use std::{borrow::Cow, collections::HashSet, io::Write, path::Path};
use tera::{Context, Delimiters, Kwargs, State, Tera, TeraResult};

/// Builds a [`Tera`] context for generating source files using Tera templates.
pub fn build_renderer(
    base_path: &Path,
    templating: &Option<Templating>,
    entries: &[Entry],
) -> TeraResult<Tera> {
    // Add template files from the includes directory
    let mut tera = Tera::new();

    // Check templating block to set custom delimitiers and prefixes
    let (prefix_bin, prefix_oct, prefix_hex) = if let Some(templating) = templating {
        // set custom delimiters
        tera.set_delimiters(templating.get_delimiters())?;

        // set custom prefixes for `bin`, `oct`, `hex` filters
        (
            templating.prefix_bin.clone(),
            templating.prefix_oct.clone(),
            templating.prefix_hex.clone(),
        )
    } else {
        (
            PREFIX_BIN.to_string(),
            PREFIX_OCT.to_string(),
            PREFIX_HEX.to_string(),
        )
    };

    // Register functions provided by the profile
    tera.register_filter("bin", move |x: u16, _: Kwargs, _: &State| {
        format!("{}{:0>16b}", prefix_bin, x)
    });
    tera.register_filter("oct", move |x: u16, _: Kwargs, _: &State| {
        format!("{}{:0>6o}", prefix_oct, x)
    });
    tera.register_filter("hex", move |x: u16, _: Kwargs, _: &State| {
        format!("{}{:0>4x}", prefix_hex, x)
    });

    // If templating block was defined, use it.
    if let Some(templating) = templating {
        tera.add_template_files(
            templating
                .includes
                .iter()
                .map(|p| (base_path.join(p), p.to_str())),
        )?;
    }

    // Now check each entry for a template file to use.
    // If two entries have the same template file, add it only once.
    let mut temp_set = HashSet::with_capacity(entries.len());
    for entry in entries {
        if let Some(template) = &entry.template {
            let _ = temp_set.insert(template);
        }
    }

    for template in temp_set {
        let path = base_path.join(template);
        tera.add_template_file(path, template.to_str())?;
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

impl Templating {
    pub fn get_delimiters(&self) -> Delimiters {
        let [block_start, block_end] = split_delimiter(&self.delimiter_block);
        let [variable_start, variable_end] = split_delimiter(&self.delimiter_variable);
        let [comment_start, comment_end] = split_delimiter(&self.delimiter_comment);
        Delimiters {
            block_start,
            block_end,
            variable_start,
            variable_end,
            comment_start,
            comment_end,
        }
    }
}

fn split_delimiter(delimiter: &str) -> [Cow<'static, str>; 2] {
    let parts = delimiter.split_at(2);
    [
        Cow::Owned(parts.0.to_string()),
        Cow::Owned(parts.1.to_string()),
    ]
}
