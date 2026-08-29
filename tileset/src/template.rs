use crate::{
    input_stack::InputEntry,
    manifest::{
        Templating,
        default::{PREFIX_BIN, PREFIX_HEX, PREFIX_OCT},
    },
    output_stack::OutputImage,
};
use std::{collections::HashSet, io::Write, path::Path};
use tera::{Context, Kwargs, State, Tera, TeraResult};

/// Setup Tera with necessary functions and child templates
pub fn setup(
    tera: &mut Tera,
    root_path: &Path,
    config: Option<&Templating>,
    long_word: bool,
) -> TeraResult<()> {
    // Check templating block to set custom delimitiers and prefixes
    let (prefix_bin, prefix_oct, prefix_hex) = if let Some(config) = config {
        // set custom delimiters
        tera.set_delimiters(config.get_delimiters())?;

        // set custom prefixes for `bin`, `oct`, `hex` filters
        (
            config.prefix_bin.clone(),
            config.prefix_oct.clone(),
            config.prefix_hex.clone(),
        )
    } else {
        (
            PREFIX_BIN.to_string(),
            PREFIX_OCT.to_string(),
            PREFIX_HEX.to_string(),
        )
    };

    // Register functions provided by the profile
    if long_word {
        // Write words as 16-bits literals
        tera.register_filter("bin", move |x: u16, _: Kwargs, _: &State| {
            format!("{}{:0>16b}", prefix_bin, x)
        });
        tera.register_filter("oct", move |x: u16, _: Kwargs, _: &State| {
            format!("{}{:0>6o}", prefix_oct, x)
        });
        tera.register_filter("hex", move |x: u16, _: Kwargs, _: &State| {
            format!("{}{:0>4x}", prefix_hex, x)
        });
    } else {
        // Write words as 8-bits literals
        tera.register_filter("bin", move |x: u16, _: Kwargs, _: &State| {
            format!("{}{:0>8b}", prefix_bin, x)
        });
        tera.register_filter("oct", move |x: u16, _: Kwargs, _: &State| {
            format!("{}{:0>3o}", prefix_oct, x)
        });
        tera.register_filter("hex", move |x: u16, _: Kwargs, _: &State| {
            format!("{}{:0>2x}", prefix_hex, x)
        });
    }

    // If config block was defined, use it.
    if let Some(config) = config {
        tera.add_template_files(
            config
                .includes
                .iter()
                .map(|p| (root_path.join(p), p.to_str())),
        )?;
    }

    Ok(())
}

/// Load the templates used by the entries
pub fn load_templates(tera: &mut Tera, root_path: &Path, entries: &[InputEntry]) -> TeraResult<()> {
    // Now check each entry for a template file to use.
    // If two entries have the same template file, add it only once.
    let mut temp_set = HashSet::with_capacity(entries.len());
    for entry in entries {
        if let Some(template) = &entry.template {
            let _ = temp_set.insert(template);
        }
    }

    for template in temp_set {
        let path = root_path.join(template);
        tera.add_template_file(path, template.to_str())?;
    }

    Ok(())
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
