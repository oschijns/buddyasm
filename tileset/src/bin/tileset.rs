//! Process pictures to generate a unified tileset

use buddyasm_common::{
    anyhow::{self, anyhow},
    clap::{self, Parser},
    manifest::load_manifest,
};
use buddyasm_tileset::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    ffi::OsString,
};

fn main() -> Result<(), anyhow::Error> {
    // Read command-line arguments
    let args = Cli::parse();
    let manifest = load_manifest::<TileSetManifest, _>(&args.manifest)?;
    let input = match InputStack::try_from(manifest) {
        Ok(input) => input,
        Err(errors) => {
            for error in errors.iter() {
                eprintln!("{}", error);
            }
            return Err(anyhow!("Errors in the input"));
        }
    };

    // Create a builder for the tilemap
    let mut builder = TileMapBuilder::new(input.config.clone());

    // Store the index maps if some are generated
    let mut output_images = HashMap::with_capacity(input.stack.len());

    // Store errors encountered while processing the stack
    let mut errors = BTreeMap::new();

    // Process each element in the input stack
    for (path, input_image, palette) in input.stack.iter() {
        match input_image {
            // Input is a common static image
            InputImage::Static(image_buffer) => match builder.process(image_buffer, palette) {
                Ok(index) => {
                    output_images.insert(path.clone(), OutputImage::Static(index));
                }
                Err(err) => {
                    errors.insert(path.clone(), err);
                }
            },
            // Input is a character set (or similar)
            InputImage::FixedPosition { image, mapping } => {
                match builder.process_fixed(image, palette, mapping) {
                    Ok(()) => {}
                    Err(err) => {
                        errors.insert(path.clone(), err);
                    }
                }
            }
            InputImage::Aseprite(aseprite_file) => todo!("Aseprite files not yet supported"),
            InputImage::TiledTileset(tileset) => todo!("Tiled files not yet supported"),
            InputImage::TiledMap(map) => todo!("Tiled files not yet supported"),
        }
    }

    // Make the output stack
    let output = OutputStack {
        tileset: builder.complete(),
        images: output_images,
    };

    Ok(())
}

/// Command-line interface
#[derive(Debug, clap::Parser)]
struct Cli {
    /// Path to the manifest file to load
    #[clap(short, long)]
    manifest: OsString,

    /// Output directory
    #[clap(short, long)]
    output: OsString,
}
