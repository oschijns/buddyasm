//! Common components for the other sub-crates

/// Load manifest files
pub mod manifest;

/// List of systems supported
pub mod system;

/// Color space partitioning system
#[cfg(feature = "color-space")]
pub mod color_space;

#[cfg(any(feature = "image", feature = "color-space"))]
pub mod color;

// re-export dependencies
pub use anyhow;
pub use toml;

#[cfg(any(feature = "image", feature = "color-space"))]
pub use image;

pub mod prelude {
    // re-export
    pub use crate::manifest::{Manifest, ManifestError, load_manifest};
}

/// Non mutable owned string
pub type Str = Box<str>;

/// Non mutable list
pub type List<T> = Box<[T]>;
