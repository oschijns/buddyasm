//! Common components for the other sub-crates

/// Load manifest files
pub mod manifest;

/// List of systems supported
pub mod system;

// re-export dependencies
pub use anyhow;
pub use toml;

pub mod prelude {
    // re-export
    pub use crate::manifest::{Manifest, ManifestError, load_manifest};
}
