use crate::{
    input_stack::InputEntry,
    manifest::{
        Templating,
        default::{PREFIX_BIN, PREFIX_HEX, PREFIX_OCT},
    },
    output_stack::OutputImage,
};
use minijinja::{Environment, path_loader};
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufReader, Read, Write},
    path::Path,
};

/// Setup Tera with necessary functions and child templates
pub fn setup(
    env: &mut Environment,
    root_path: &Path,
    config: Option<&Templating>,
    long_word: bool,
) -> Result<(), minijinja::Error> {
    // Check templating block to set custom delimitiers and prefixes
    let (prefix_bin, prefix_oct, prefix_hex) = if let Some(config) = config {
        // set custom syntax
        env.set_syntax(config.get_syntax_config()?);

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
        env.add_filter("bin", move |x: u16| format!("{}{:0>16b}", prefix_bin, x));
        env.add_filter("oct", move |x: u16| format!("{}{:0>6o}", prefix_oct, x));
        env.add_filter("hex", move |x: u16| format!("{}{:0>4x}", prefix_hex, x));
    } else {
        // Write words as 8-bits literals
        env.add_filter("bin", move |x: u16| format!("{}{:0>8b}", prefix_bin, x));
        env.add_filter("oct", move |x: u16| format!("{}{:0>3o}", prefix_oct, x));
        env.add_filter("hex", move |x: u16| format!("{}{:0>2x}", prefix_hex, x));
    }

    // If config block was defined, use it.
    if let Some(config) = config
        && let Some(includes) = &config.includes
    {
        env.set_loader(path_loader(root_path.join(includes)));
    }

    Ok(())
}

/// Error when loading templates for the entries
#[derive(thiserror::Error, Debug)]
pub enum TempError {
    #[error("IO: {0}")]
    Io(#[from] io::Error),

    #[error("Jinja: {0}")]
    Jinja(#[from] minijinja::Error),
}

/// Load the templates used by the entries
pub fn load_templates(
    env: &mut Environment,
    root_path: &Path,
    entries: &[InputEntry],
) -> Result<(), TempError> {
    // Now check each entry for a template file to use.
    // If two entries have the same template file, add it only once.
    let mut temp_set = HashSet::with_capacity(entries.len());
    for entry in entries {
        if let Some(template) = &entry.template {
            let _ = temp_set.insert(template);
        }
    }

    // Given a template, we want to load it into the environment.
    // And identify it by its filename.
    for template in temp_set {
        // Get a usable filename identifier
        if let Some(name) = template.file_name().and_then(|t| t.to_str()) {
            // Evaluate the path to the file to load
            let path = root_path.join(template);

            // Load the template file
            let mut source = String::new();
            BufReader::new(File::open(path)?).read_to_string(&mut source)?;
            env.add_template_owned(name.to_string(), source)?;
        }
    }

    Ok(())
}

/// Render the output image using the given Tera template and context.
pub fn render(
    env: &Environment,
    template_name: &str,
    output_image: &OutputImage,
    write: impl Write,
) -> Result<(), minijinja::Error> {
    let context = minijinja::Value::from_serialize(output_image);
    let temp = env.get_template(template_name)?;
    temp.render_captured_to(context, write)?;
    Ok(())
}
