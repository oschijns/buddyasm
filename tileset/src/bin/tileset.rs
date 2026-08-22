//! Process pictures to generate a unified tileset

use buddyasm_common::{
    anyhow::{self, anyhow},
    clap::{self, Parser},
    manifest::{Manifest, load_manifest},
};
use buddyasm_tileset::{
    prelude::*,
    process::process_stack,
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

    // Create the input stack to process and generate the output stack
    let input = InputStack::try_from(manifest)?;
    let output = process_stack(&input)?;

    // serialize the result
    let bits = serial.serialize(&output.tileset);
    let out_path = Path::new(&out_path).to_path_buf().canonicalize()?;
    let mut file = File::create(out_path.join("tileset.chr"))?;
    file.write_all(bits.as_bytes())?;

    // serialize the output stack
    for entry in &output.entries {
        if entry.output_json {
            let mut path_file = out_path.join(&entry.name);
            path_file.set_extension("json");
            let mut file = File::create(path_file)?;
            serde_json::to_writer_pretty(&mut file, &entry.image)?;
        }
    }

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
