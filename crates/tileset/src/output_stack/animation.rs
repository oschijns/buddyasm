use crate::output_stack::OutMap;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Set of animations extracted from a Aseprite file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSet {
    /// Animation sequences identified by name
    pub sequences: BTreeMap<String, AnimSequence>,

    /// Frames to index
    pub frames: Vec<OutMap>,
}

/// Animation sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "indexes")]
pub enum AnimSequence {
    /// Simple sequence
    #[serde(rename = "default")]
    Single(Vec<AnimFrame>),

    /// Sequence flipped horizontally
    #[serde(rename = "left-right")]
    FlipH(Vec<AnimFrame<Flip1>>),

    /// Sequence flipped vertically
    #[serde(rename = "up-down")]
    FlipV(Vec<AnimFrame<Flip1>>),

    /// Sequence flipped horizontally and vertically
    #[serde(rename = "all")]
    FlipBoth(Vec<AnimFrame<Flip2>>),
}

/// Frame of an animation sequence
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct AnimFrame<I = Flip0> {
    /// Index of the frame to use
    #[serde(flatten)]
    pub frame_indexes: I,

    /// Duration of the frame
    pub duration: u16,
}

/// Single index (no frame flipping)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Flip0 {
    pub index: u16,
}

/// Two indexes (horizontal or vertical flipping)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Flip1 {
    /// Index for initial frame
    pub index: u16,

    /// Index for flipped frame
    pub index_flipped: u16,
}

/// Four indexes (horizontal and vertical flipping)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Flip2 {
    /// Index of the frame to use
    pub index: u16,

    /// Index of the horizontally flipped frame
    pub index_flip_h: u16,

    /// Index of the vertically flipped frame
    pub index_flip_v: u16,

    /// Index of the horizontally and vertically flipped frame
    pub index_flip_both: u16,
}

impl AnimFrame<Flip0> {
    #[inline]
    pub fn new0(index: usize, duration: u16) -> Self {
        Self {
            frame_indexes: Flip0 {
                index: index as u16,
            },
            duration,
        }
    }
}

impl AnimFrame<Flip1> {
    #[inline]
    pub fn new1(index: usize, index_flipped: usize, duration: u16) -> Self {
        Self {
            frame_indexes: Flip1 {
                index: index as u16,
                index_flipped: index_flipped as u16,
            },
            duration,
        }
    }
}

impl AnimFrame<Flip2> {
    #[inline]
    pub fn new2(
        index: usize,
        index_flip_h: usize,
        index_flip_v: usize,
        index_flip_both: usize,
        duration: u16,
    ) -> Self {
        Self {
            frame_indexes: Flip2 {
                index: index as u16,
                index_flip_h: index_flip_h as u16,
                index_flip_v: index_flip_v as u16,
                index_flip_both: index_flip_both as u16,
            },
            duration,
        }
    }
}
