//! Process pictures to generate a unified tileset

/// Define the structure of the manifest file
pub mod manifest;

/// Define target hardware profiles
pub mod profile;

/// Store all the raw data to process
pub mod input_stack;

/// Store all the raw data to write back to disk
pub mod output_stack;

/// Raw data being processed
pub mod data;

/// Convert the input stack into an output stack
pub mod process;

/// Define templating relevant procedures
pub mod template;

//pub mod config;
//pub mod render;
//pub mod serial;

pub mod prelude {
    // re-export common types
}
