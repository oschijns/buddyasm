use super::*;
use crate::{data::palette::Palette, input_stack::Aseprite, output_stack::OutError};
use aseprite_loader::loader::{AsepriteFile, LayerSelection, LoadImageError, Tag};
use image::{Rgba, RgbaImage};
use itertools::Itertools;
use regex::Regex;
use std::{str::FromStr, sync::LazyLock};
use strum::ParseError;

impl Builder {
    /// Process an Aseprite file
    pub(super) fn process_animations(
        &mut self,
        aseprite: &Aseprite,
        pal: &Palette,
    ) -> Result<(), Vec<OutError>> {
        // Push errors into this list
        let mut errors = Vec::<OutError>::new();

        let file = aseprite.file();
        let mut proc = ProcessSequence::new(file);
        for tag in file.tags() {
            match self.process_anim_sequence(&mut proc, pal, tag) {
                Ok(ok) => {}
                Err(err) => {
                    errors.push(err);
                    continue;
                }
            }
        }

        /* Iterate over the tags in the file
        for tag in file.tags() {
            // For now we will use the tag name to specify the `LR`, `UD` flags.
            let (name, flip) = match extract_flip_flag(&tag.name) {
                Ok(ok) => ok,
                Err(err) => {
                    errors.push(OutError::ParseFlipFlag(err));
                    continue;
                }
            };

            // Iterate over the frames constituing this animation
            for i in tag.range.clone().into_iter() {
                // Render the frame to the buffer
                // Pixels are written in RGBA8 format
                if let Err(err) =
                    file.render_frame(i as usize, &mut buffer, &LayerSelection::Visible)
                {
                    errors.push(value);
                }
                let frame = &file.frames[i as usize];

                // TODO: the following is not necessary ???
                let (fx, fy) = frame.origin;

                // Iterate over the cels in the frame
                for cel in frame.cels.iter() {
                    let (cx, cy) = cel.origin;
                    let (cw, ch) = cel.size;

                    // From the cel, we can find the image to use
                    let image = &file.images[cel.image_index];
                    let layer = &file.layers[cel.layer_index];

                    // We use the layer to data to figure out if it is visible or not
                    if layer.visible {
                        let x = fx + cx;
                        let y = fy + cy;

                        //let sub_img = image.data
                    }
                }
            }
        } // */

        todo!()
    }

    // TODO:
    // move the list of errors to the builder with proper identification of the location of the errors

    /// Process one animation sequence
    fn process_anim_sequence<'f>(
        &mut self,
        proc: &mut ProcessSequence<'f>,
        pal: &Palette,
        tag: &Tag,
    ) -> Result<(), OutError> {
        // For now we will use the tag name to specify the `LR`, `UD` flags.
        let (name, flip) = extract_flip_flag(&tag.name)?;

        // Allocate a buffer to store the frame as images
        let count = tag.range.clone().count();
        let mut frames = Vec::<()>::with_capacity(count);

        // Iterate over the frames constituing this animation
        for i in tag.range.clone().into_iter() {
            // Convert the image data into an exploitable format
            proc.render_frame(i as usize)?;
            let frame_data = &proc.file.frames[i as usize];

            // Process the image as an individual pixel art
            match self.process(&proc.image, pal) {
                Ok(tilemap) => {}
                Err(errors) => {}
            }

            // Push the frame in the array
            //frames.push((self.build_image(), frame_data.duration));
        }

        Ok(())
    }
}

/// Data structure for processing an animation sequence defined by a tag.
struct ProcessSequence<'f> {
    /// Parsed aseprite file
    file: &'f AsepriteFile<'f>,

    /// Buffer for writing frame image (size == width * height * 4)
    /// Image data will be written as RGBA8
    write_buffer: Vec<u8>,

    /// Image buffer to be passed to the builder for further processing
    image: RgbaImage,
}

impl<'f> ProcessSequence<'f> {
    /// Create a context for processing animations
    fn new(file: &'f AsepriteFile<'f>) -> Self {
        let (width, height) = file.size();

        // Allocate a buffer for rendering the images
        let size = width as usize * height as usize * 4usize;
        let mut write_buffer = Vec::<u8>::with_capacity(size);
        write_buffer.resize(size, 0);

        // Allocate a RGBA image buffer
        let image = RgbaImage::new(width as u32, height as u32);

        Self {
            file,
            write_buffer,
            image,
        }
    }

    /// Render the selected frame to the internal buffer
    fn render_frame(&mut self, frame_index: usize) -> Result<(), LoadImageError> {
        // Render the frame to the buffer
        // Pixels are written in RGBA8 format
        self.file.render_frame(
            frame_index,
            &mut self.write_buffer,
            &LayerSelection::Visible,
        )?;

        // Write the pixel into the image buffer
        for ((&r, &g, &b, &a), pix) in self
            .write_buffer
            .iter()
            .tuples()
            .zip(self.image.pixels_mut())
        {
            *pix = Rgba([r, g, b, a]);
        }

        Ok(())
    }
}

/// Flag to identify in the tag attribute data to determine how to flip the animation.
/// This is relevant as on most retro hardware flipping an sprite made of multiple
/// hardware sprites involves placing the sprites at specific offsets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum FlipFlag {
    /// Do not flip the animation.
    /// Thus generate only one sequence of frames.
    #[default]
    #[strum(serialize = "none")]
    None,

    /// Flip the animation horizontally.
    /// Thus generate two sequences of frames.
    #[strum(
        serialize = "left-right",
        serialize = "left_right",
        serialize = "lr",
        serialize = "horizontal",
        serialize = "h"
    )]
    LeftRight,

    /// Flip the animation vertically.
    /// Thus generate two sequences of frames.
    #[strum(
        serialize = "up-down",
        serialize = "up_down",
        serialize = "ud",
        serialize = "vertical",
        serialize = "v"
    )]
    UpDown,

    /// Flip the animation horizontally and vertically.
    /// Thus generate four sequences of frames.
    #[strum(serialize = "all", serialize = "*")]
    All,
}

/// Regex for extracting the flip flag from a tag name
static REGEX_FLIP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\w)+(\.([A-Za-z_\-\*]+)?)").unwrap());

/// Parse the given tag name to extract flip flag
fn extract_flip_flag(tag_name: &str) -> Result<(String, FlipFlag), ParseError> {
    // Use the regex to identify a flip flag
    if let Some(cap) = REGEX_FLIP.captures(tag_name) {
        let name = cap.get(1).expect("Name should have been captured").as_str();
        let flip = if let Some(flag) = cap.get(3) {
            FlipFlag::from_str(flag.as_str())?
        } else {
            FlipFlag::None
        };
        Ok((name.to_string(), flip))
    } else {
        // Normally all valid names should be matched anyway
        Ok((tag_name.to_string(), FlipFlag::None))
    }
}
