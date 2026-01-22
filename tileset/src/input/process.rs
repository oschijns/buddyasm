use super::{InputStack, OutputStack};
use crate::tileset::builder::TileError;

/// Return the errors regarding a stack of input images
#[derive(Debug)]
pub struct StackError {
    /// Index of the image to process
    image_index: usize,

    /// List of tiles that have errors
    errors: Vec<TileError>,
}

impl InputStack {
    // For each image in the stack, process it
    pub fn process(&self) -> Result<OutputStack, Vec<StackError>> {
        todo!()
    }
}
