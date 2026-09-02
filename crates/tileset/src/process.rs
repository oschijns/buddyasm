/// Create a generic input config from a hardware profile
pub mod config;

/// Build an input stack from the manifest data
pub mod prepare;

/// Process the input stack and generate an output stack
pub mod builder;

/// Serialize the tileset in a format supported by the target hardware
/// This will generate a raw binary CHR file
pub mod serialize;

/// Encode the tilemap data in a format that can be directly used by the hardware
/// This will be used by the templating system to generate a source file
pub mod encode;
