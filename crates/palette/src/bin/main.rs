use buddyasm_common::anyhow;
use clap::{self, Parser};

/// Command-line interface
#[derive(Debug, clap::Parser)]
struct Cli {}

fn main() -> Result<(), anyhow::Error> {
    // Read command-line arguments
    let args = Cli::parse();

    Ok(())
}
