use super::*;
use crate::{
    data::{palette::Palette, tilemap::TileMap},
    input_stack::Aseprite,
    output_stack::{
        OutError,
        animation::{AnimFrame, AnimSequence},
    },
};
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
        palette: &Palette,
    ) -> Result<(BTreeMap<String, AnimSequence>, Vec<TileMap>), OutError> {
        // Access the Aseprite file
        let file = aseprite.file();

        // Allocate a workarea to process sequences of animation
        let mut work_area = WorkArea::new(file);
        // Create a map to store the animation with their frame indexes
        let mut out = BTreeMap::new();

        // Iterate over the tagged animations
        for tag in file.tags() {
            let (name, seq) = self.process_anim_sequence(&mut work_area, palette, tag)?;
            out.insert(name, seq);
        }

        // Return the animation sequences and the frames
        Ok((out, work_area.frames))
    }

    /// Process one animation sequence
    fn process_anim_sequence<'f>(
        &mut self,
        work_area: &mut WorkArea<'f>,
        palette: &Palette,
        tag: &Tag,
    ) -> Result<(String, AnimSequence), OutError> {
        // For now we will use the tag name to specify the `LR`, `UD` flags.
        let (name, flip) = extract_flip_flag(&tag.name)?;

        // Allocate buffers to store the animation sequences
        let count = tag.range.clone().count();

        let seq = match flip {
            FlipFlag::None => {
                let mut seq = Vec::with_capacity(count);

                // Iterate over the frames in this animation
                for i in tag.range.clone().into_iter() {
                    // Convert the image data into an exploitable format
                    work_area.render_frame(i as usize)?;
                    let frame_data = &work_area.file.frames[i as usize];

                    // Process the image as an individual pixel art
                    let tilemap = self.process(&work_area.image, palette)?;
                    let index = work_area.identify_frame(tilemap);

                    // Push the frame in the array
                    seq.push(AnimFrame::new0(index, frame_data.duration));
                }
                AnimSequence::Single(seq)
            }
            FlipFlag::LeftRight => {
                let mut seq = Vec::with_capacity(count);

                // Iterate over the frames in this animation
                for i in tag.range.clone().into_iter() {
                    // Convert the image data into an exploitable format
                    work_area.render_frame(i as usize)?;
                    let frame_data = &work_area.file.frames[i as usize];

                    // Process the image as an individual pixel art
                    let tilemap = self.process(&work_area.image, palette)?;
                    let index_n = work_area.identify_frame(tilemap.clone());
                    let index_h = work_area.identify_frame(tilemap.flip_h());

                    // Push the frame in the array
                    seq.push(AnimFrame::new1(index_n, index_h, frame_data.duration));
                }
                AnimSequence::FlipH(seq)
            }
            FlipFlag::UpDown => {
                let mut seq = Vec::with_capacity(count);

                // Iterate over the frames in this animation
                for i in tag.range.clone().into_iter() {
                    // Convert the image data into an exploitable format
                    work_area.render_frame(i as usize)?;
                    let frame_data = &work_area.file.frames[i as usize];

                    // Process the image as an individual pixel art
                    let tilemap = self.process(&work_area.image, palette)?;
                    let index_n = work_area.identify_frame(tilemap.clone());
                    let index_v = work_area.identify_frame(tilemap.flip_v());

                    // Push the frame in the array
                    seq.push(AnimFrame::new1(index_n, index_v, frame_data.duration));
                }
                AnimSequence::FlipV(seq)
            }
            FlipFlag::All => {
                let mut seq = Vec::with_capacity(count);

                // Iterate over the frames in this animation
                for i in tag.range.clone().into_iter() {
                    // Convert the image data into an exploitable format
                    work_area.render_frame(i as usize)?;
                    let frame_data = &work_area.file.frames[i as usize];

                    // Process the image as an individual pixel art
                    let tilemap = self.process(&work_area.image, palette)?;
                    let index_n = work_area.identify_frame(tilemap.clone());
                    let index_h = work_area.identify_frame(tilemap.flip_h());
                    let index_v = work_area.identify_frame(tilemap.flip_v());
                    let index_b = work_area.identify_frame(tilemap.flip_both());

                    // Push the frame in the array
                    seq.push(AnimFrame::new2(
                        index_n,
                        index_h,
                        index_v,
                        index_b,
                        frame_data.duration,
                    ));
                }
                AnimSequence::FlipBoth(seq)
            }
        };

        Ok((name, seq))
    }
}

// MARK: Work area for animation sequence

/// Data structure for processing an animation sequence defined by a tag.
struct WorkArea<'f> {
    /// Parsed aseprite file
    file: &'f AsepriteFile<'f>,

    /// Buffer for writing frame image (size == width * height * 4)
    /// Image data will be written as RGBA8
    write_buffer: Vec<u8>,

    /// Image buffer to be passed to the builder for further processing
    image: RgbaImage,

    /// Store individual frames to be identified by index
    frames: Vec<TileMap>,
}

impl<'f> WorkArea<'f> {
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
            frames: Vec::new(),
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

    /// Look into the already generated frame for a matching one.
    /// If none is matching, insert the new frame at the tail of the list.
    /// Return the index corresponding to the frame.
    fn identify_frame(&mut self, frame: TileMap) -> usize {
        // Look for a pre-existing instance of the frame
        if let Some(index) = self.frames.iter().position(|f| *f == frame) {
            index
        } else {
            // Otherwise insert the new frame in the stack
            let index = self.frames.len();
            self.frames.push(frame);
            index
        }
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
    LazyLock::new(|| Regex::new(r"(\w+)(\.([A-Za-z_\-\*]+)?)").unwrap());

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
