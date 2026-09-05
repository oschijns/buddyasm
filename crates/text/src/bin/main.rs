use buddyasm_common::anyhow;
use buddyasm_text::text::Config;
use clap::{self, Parser};

/// Command-line interface
#[derive(Debug, clap::Parser)]
struct Cli {
    /// Max length of each line of text
    #[clap(short, long)]
    line_length: usize,

    /// Max number of lines per page
    #[clap(short, long)]
    page_size: usize,

    /// Max number of lines per page
    #[clap(short, long, default_value_t = 0)]
    default_byte: u8,

    /// Text to process
    text: String,
}

fn main() -> Result<(), anyhow::Error> {
    // Read command-line arguments
    let args = Cli::parse();

    // Handle Form Feed characters
    let text = args.text.replace(r"\x0C", "\x0C");

    let config = Config::new(args.line_length, args.page_size);
    let text = config.split(&text);
    let json = serde_json::to_string_pretty(&text)?;

    print!("{}", json);
    Ok(())
}
