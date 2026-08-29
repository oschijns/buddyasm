//! Process pictures to generate a unified tileset

use buddyasm_common::{
    anyhow,
    clap::{self, Parser},
    manifest::{Manifest, load_manifest},
};
use buddyasm_tileset::{
    config::profile::Profile,
    prelude::*,
    process::{output, process_stack},
    render::{build_renderer, render},
    serial::{
        SerialTile,
        profile::Serial,
        tile_data::{TileData, encode_tiles},
    },
};
use image::EncodableLayout;
use std::{
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

    // Create a profile for the target hardware
    let profile = Profile::new(
        manifest.config.hardware,
        manifest.config.kind,
        manifest.config.sprite_size.as_deref(),
    )?;

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

    // We need a renderer for generating the sources
    let tera = build_renderer(
        &manifest.get_path(),
        &manifest.templating,
        &manifest.entries,
    )?;

    // Create the input stack to process and generate the output stack
    let input = InputStack::new(&profile, &manifest)?;
    let mut output = process_stack(&input)?;

    // serialize the result
    let bits = serial.serialize(&output.tileset);
    let out_path = Path::new(&out_path).to_path_buf().canonicalize()?;
    let mut file = File::create(out_path.join("tileset.chr"))?;
    file.write_all(bits.as_bytes())?;

    // Generate encoded tile data
    for entry in output.entries.iter_mut() {
        match &mut entry.image {
            OutputImage::Static(index_map) => {
                encode_tiles(&profile, index_map);
            }
            OutputImage::Animated(hash_map) => {
                for anim in hash_map.values_mut() {
                    match anim {
                        OutputAnimation::Normal(map) => encode_tiles(&profile, map),
                        OutputAnimation::LeftRight { left, right } => {
                            encode_tiles(&profile, left);
                            encode_tiles(&profile, right);
                        }
                        OutputAnimation::UpDown { up, down } => {
                            encode_tiles(&profile, up);
                            encode_tiles(&profile, down);
                        }
                        OutputAnimation::FourWays {
                            up_left,
                            up_right,
                            down_left,
                            down_right,
                        } => {
                            encode_tiles(&profile, up_left);
                            encode_tiles(&profile, up_right);
                            encode_tiles(&profile, down_left);
                            encode_tiles(&profile, down_right);
                        }
                    }
                }
            }
        }
    }

    // serialize the output stack
    for entry in &output.entries {
        // Generate the output JSON if requested
        if entry.output_json {
            let mut path_file = out_path.join(&entry.name);
            path_file.set_extension("json");
            let mut file = File::create(path_file)?;
            serde_json::to_writer_pretty(&mut file, &entry.image)?;
        }

        // Generate the source file if requested
        if let Some(template) = &entry.template {
            let mut path_file = out_path.join(&entry.name);
            if let Some(ext) = template.extension() {
                path_file.set_extension(ext);
            }
            let mut file = File::create(path_file)?;
            render(&tera, &template.to_string_lossy(), &entry.image, &mut file)?;
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
