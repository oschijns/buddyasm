use image::{Rgb, Rgba};
use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hash, Hasher},
    sync::{Arc, RwLock},
};

/// Define a partitioning of the color space given a finite set of colors.
#[derive(Debug, Clone)]
pub struct ColorSpace {
    /// Read-only part of the palette
    read: Arc<InnerRead>,

    /// Writable part of the palette
    write: Arc<RwLock<InnerWrite>>,
}

/// Storage for the processed palette.
#[derive(Debug)]
struct InnerRead {
    /// Set of unique colors extracted from the palette
    colorset: Vec<(ColorHash, usize, Rgb<u8>)>,

    /// Mapping between indexes and color
    map: HashMap<usize, Rgb<u8>>,
}

/// Storage for the processed palette.
#[derive(Debug)]
struct InnerWrite {
    /// Store already identified mapping between RGB colors and corresponding index.
    /// This map is initially filled with colors from the palette but may be completed
    /// overtime if we encounter colors that are close but not perfect match.
    lookup: HashMap<ColorHash, (usize, Rgb<u8>), BuildHasherDefault<ColorHasher>>,
}

/// Accumulator to build a color space partitioning
#[derive(Debug)]
pub struct ColorSpaceBuilder {
    /// color accumulator
    set: HashSet<Rgb<u8>>,

    /// threshold for discarding colors based on their alpha channel
    pub alpha_threshold: u8,
}

/// Default threshold for discarding colors based on their alpha channel.
/// Any value below this threshold is treated as transparent.
pub const ALPHA_THRESHOLD: u8 = 128;

impl ColorSpace {
    /// Create a color space from the given list of colors
    pub fn load(color_list: &[(Rgb<u8>, usize)]) -> Self {
        let (read, write) = load_indexed_colors(color_list);
        Self {
            read: Arc::new(read),
            write: Arc::new(RwLock::new(write)),
        }
    }
}

impl ColorSpaceBuilder {
    /// Create a builder for a color space partitioning
    #[inline]
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
            alpha_threshold: ALPHA_THRESHOLD,
        }
    }

    /// Create a builder for a color space partitioning
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            set: HashSet::with_capacity(capacity),
            alpha_threshold: ALPHA_THRESHOLD,
        }
    }

    /// Add a color in the accumulator
    #[inline]
    pub fn add(&mut self, color: Rgb<u8>) {
        self.set.insert(color);
    }

    /// Add a color in the accumulator
    #[inline]
    pub fn add_a(&mut self, color: Rgba<u8>) {
        let Rgba([r, g, b, a]) = color;
        if a >= self.alpha_threshold {
            self.set.insert(Rgb([r, g, b]));
        }
    }

    /// Output the color partitioning
    pub fn finish(self) -> ColorSpace {
        // Assign a unique index for each color
        let mut list = Vec::with_capacity(self.set.len());
        for (index, color) in self.set.iter().enumerate() {
            list.push((*color, index));
        }

        ColorSpace::load(&list)
    }
}

/// Given a list of colors with their corresponding index,
/// return structures for indexing the color space.
fn load_indexed_colors(color_list: &[(Rgb<u8>, usize)]) -> (InnerRead, InnerWrite) {
    // Assign a unique index to each color encountered by converting the color set into a list
    let count = color_list.len();
    let mut colorset = Vec::with_capacity(count);
    let mut map = HashMap::with_capacity(count);
    for (color, index) in color_list.iter() {
        colorset.push((ColorHash::from(*color), *index, *color));
        map.insert(*index, *color);
    }

    // Initialize the lookup table and fill it with color indexes
    let mut lookup =
        HashMap::with_capacity_and_hasher(colorset.len(), BuildHasherDefault::<ColorHasher>::new());
    for (hash, index, color) in colorset.iter() {
        lookup.insert(*hash, (*index, *color));
    }

    (InnerRead { colorset, map }, InnerWrite { lookup })
}

/// Error encountered when trying to lock a shared resource
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockError {
    #[error("Failed to lock the shared map for reading")]
    ReadLock,

    #[error("Failed to lock the shared map for writing")]
    WriteLock,
}

impl ColorSpace {
    /// Given an color, identify the indexes of the color of each pixel.
    /// If the color is not a perfect match return the index of the closest color
    /// in the set and cache the new mapping for faster lookup on the next iteration.
    pub fn identify_color_index(&self, color: Rgb<u8>) -> Result<(usize, Rgb<u8>), LockError> {
        // Check if the color has already been identified before
        let hash = ColorHash::from(color);

        // Lock the lookup map for reading
        let Ok(write) = self.write.read() else {
            return Err(LockError::ReadLock);
        };

        // Check if an index has already be identified for this color
        if let Some((index, color2)) = write.lookup.get(&hash) {
            Ok((*index, *color2))
        } else {
            // Drop the lock on the lookup map while we search
            // for the index of the closest color.
            drop(write);

            // Find the index of the closest color in the color set.
            let mut selected = (u32::MAX, usize::MAX, Rgb([0, 0, 0]));
            for &(_, index, color2) in self.read.colorset.iter() {
                let sqr_dist = squared_distance(color, color2);

                // If the squared distance is smaller than
                // the current minimum, update the selection.
                if sqr_dist < selected.0 {
                    selected = (sqr_dist, index, color2);
                }
            }

            // Lock the lookup map in writing to insert the new entry
            let Ok(mut write) = self.write.write() else {
                return Err(LockError::WriteLock);
            };
            write.lookup.insert(hash, (selected.1, selected.2));

            // Return the index found
            Ok((selected.1, selected.2))
        }
    }

    /// Get the index of the provided color.
    pub fn get_color_index(&self, color: Rgb<u8>) -> Option<usize> {
        // Check if the color has already been identified before
        let hash = ColorHash::from(color);

        // Lock the lookup map for reading
        let Ok(write) = self.write.read() else {
            return None;
        };

        // Check if an index has already be identified for this color
        if let Some((index, _)) = write.lookup.get(&hash) {
            Some(*index)
        } else {
            None
        }
    }

    /// Get the index of the provided color.
    pub fn by_index(&self, index: usize) -> Option<Rgb<u8>> {
        self.read.map.get(&index).map(|c| *c)
    }
}

/// Compute the squared distance between two points in 3D space
#[inline]
const fn squared_distance(p0: Rgb<u8>, p1: Rgb<u8>) -> u32 {
    let [r0, g0, b0] = p0.0;
    let [r1, g1, b1] = p1.0;

    #[inline]
    const fn pow2(x: i32) -> u32 {
        (x * x) as u32
    }

    // 255^2 * 3 is below the maximum value of 2^32 so overflow is not possible
    let r = pow2(r0 as i32 - r1 as i32);
    let g = pow2(g0 as i32 - g1 as i32);
    let b = pow2(b0 as i32 - b1 as i32);

    r + g + b
}

/// Wrapper to quickly hash the RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorHash(u32);

/// Hasher for colors
#[derive(Debug, Default, Clone, Copy)]
struct ColorHasher(u32);

impl From<Rgb<u8>> for ColorHash {
    fn from(color: Rgb<u8>) -> Self {
        let Rgb([r, g, b]) = color;
        Self(u32::from_ne_bytes([r, g, b, 0x00]))
    }
}

impl Hash for ColorHash {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}

impl Hasher for ColorHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0 = i;
    }

    /// Should not be used
    fn write(&mut self, _: &[u8]) {
        panic!("Invalid use")
    }
}
