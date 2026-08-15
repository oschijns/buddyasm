//! Process pictures to generate a unified tileset

use buddyasm_common::{
    anyhow::{self, anyhow},
    clap::{self, Parser},
    manifest::{Manifest, load_manifest},
};
use buddyasm_tileset::{
    prelude::*,
    serial::{SerialTile, profile::Serial},
};
use image::EncodableLayout;
use std::{
    collections::{BTreeMap, HashMap},
    env,
    ffi::OsString,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

fn main() -> Result<(), anyhow::Error> {
    // Read command-line arguments
    let args = Cli::parse();

    // Get the current directory
    let current_directory = env::current_dir()?;

    // Load the manifest
    let man_path = if let Some(path) = &args.manifest {
        PathBuf::from(path)
    } else {
        current_directory.join("manifest.toml")
    };
    let manifest = load_manifest::<TileSetManifest, _>(&man_path)?;

    // Create the serializer for the processed tilesets
    let serial = Serial::new(
        manifest.config.hardware,
        manifest.config.kind,
        manifest.config.bit_plane.as_deref(),
    )?;

    // Where to write the output?
    let out_path = if let Some(output) = args.output {
        Path::new(&output).to_path_buf().canonicalize()?
    } else if let Some(output) = &manifest.config.output {
        manifest.evaluate_path(output)
    } else {
        current_directory.join("output")
    };

    // Create the input stack to process
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
            InputImage::Static(image) => match builder.process(image, palette) {
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
            InputImage::Aseprite(aseprite) => todo!("Aseprite files not yet supported"),
            InputImage::TiledTileset(tileset) => todo!("Tiled files not yet supported"),
            InputImage::TiledMap(map) => todo!("Tiled files not yet supported"),
        }
    }

    // Make the output stack
    let output = OutputStack {
        tileset: builder.complete(),
        images: output_images,
    };

    // serialize the result
    let bits = serial.serialize(&output.tileset);
    let out_path = Path::new(&out_path).to_path_buf().canonicalize()?;
    let mut file = File::create(out_path.join("tileset.chr"))?;
    file.write_all(bits.as_bytes())?;

    Ok(())
}

/// Command-line interface
#[derive(Debug, clap::Parser)]
struct Cli {
    /// Path to the manifest file to load.
    /// If not specified, defaults to `tileset.manifest.toml` in the current directory.
    #[clap(short, long)]
    manifest: Option<OsString>,

    /// Output directory override.
    /// If not specified, defaults to `output` next to the manifest file.
    #[clap(short, long)]
    output: Option<OsString>,
}
