//! Process the input stack and generate an output stack

pub mod output;

use crate::{
    config::input::{InputImage, InputStack},
    data::builder::{TileError, TileMapBuilder},
    process::output::{OutputEntry, OutputImage, OutputStack},
};
use core::{error, fmt};
use std::{collections::BTreeMap, path::PathBuf};

/// Represents errors that occur during the processing of the input stack
#[derive(Debug)]
pub struct OutputStackError {
    /// List of errors grouped by file path
    pub errors: BTreeMap<PathBuf, Vec<TileError>>,
}

/// Formats the error message for the output stack error
impl fmt::Display for OutputStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (path, errors) in &self.errors {
            writeln!(f, "{}:", path.display())?;
            for error in errors {
                writeln!(f, "  - {}", error)?;
            }
        }
        Ok(())
    }
}

/// Implements the error trait for the output stack error
impl error::Error for OutputStackError {}

/// Process the input stack and generate an output stack
pub fn process_stack(input: &InputStack) -> Result<OutputStack, OutputStackError> {
    // Create a builder for the tilemap
    let mut builder = TileMapBuilder::new(input.config.clone());

    // Store the index maps if some are generated
    let mut out_entries = Vec::with_capacity(input.stack.len());

    // Store errors encountered while processing the stack
    let mut errors = BTreeMap::new();

    // Process each element in the input stack
    for entry in input.stack.iter() {
        match &entry.image {
            // Input is a common static image
            InputImage::Static(image) => match builder.process(image, &entry.palette) {
                Ok(index) => {
                    out_entries.push(OutputEntry::new(
                        entry.name.clone(),
                        OutputImage::Static(index),
                        entry.output_json,
                        entry.template.clone(),
                    ));
                }
                Err(err) => {
                    errors.insert(entry.path.clone(), err);
                }
            },
            // Input is a character set (or similar)
            InputImage::FixedPosition { image, mapping } => {
                match builder.process_fixed(image, &entry.palette, mapping) {
                    Ok(()) => {}
                    Err(err) => {
                        errors.insert(entry.path.clone(), err);
                    }
                }
            }
            InputImage::Aseprite(aseprite) => todo!("Aseprite files not yet supported"),
            InputImage::TiledTileset(tileset) => todo!("Tiled files not yet supported"),
            InputImage::TiledMap(map) => todo!("Tiled files not yet supported"),
        }
    }

    // Check if there are any errors and return the appropriate result
    if errors.is_empty() {
        Ok(OutputStack {
            tileset: builder.complete(),
            entries: out_entries,
        })
    } else {
        Err(OutputStackError { errors })
    }
}
