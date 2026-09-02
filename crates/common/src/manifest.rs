//! Process pictures to generate a unified tileset

use serde::de::DeserializeOwned;
use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

/// A manifest file needs to know its absolute directory path
/// so that the other files can be found with relative paths.
pub trait Manifest {
    /// Set the path to the directory containing this manifest
    fn set_path(&mut self, path: PathBuf);

    /// Get the absolute path to the directory containing this manifest
    fn get_path(&self) -> &Path;

    /// Evaluate the path to an element in this manifest
    #[inline]
    fn evaluate_path<P: AsRef<Path>>(&self, other: P) -> PathBuf {
        self.get_path().join(other)
    }
}

/// Load a manifest file from a system path
pub fn load_manifest<M, P>(path: P) -> Result<M, ManifestError>
where
    P: AsRef<Path>,
    M: Manifest + DeserializeOwned,
{
    // Evaluate the path
    let mut path = path.as_ref().to_path_buf().canonicalize()?;

    // Read the content of the file
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let mut manifest = toml::from_str::<M>(&content)?;

    // Once we have the manifest, we want to store the
    // path to the directory containing this manifest.
    path.pop();
    manifest.set_path(path);
    Ok(manifest)
}

/// Error encountered when loading a manifest file
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    /// Failed to oepn the file
    #[error("Failed to open the file: {0}")]
    Io(#[from] io::Error),

    /// Failed to parse the TOML file
    #[error("Failed to parse the TOML data: {0}")]
    Toml(#[from] toml::de::Error),
}
