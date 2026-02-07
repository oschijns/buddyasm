//! Map a given tile to a target index

use crate::{
    config::manifest::MapRange,
    data::coords::{Coords, Dimensions},
};
use std::{collections::HashMap, rc::Rc};

/// Enforce a fixed mapping between a tile position and a target index
#[derive(Default, Debug, Clone)]
pub struct Mapping(pub(crate) Rc<HashMap<Coords, usize>>);

impl Mapping {
    /// Create a new mapping from a hashmap
    #[inline]
    pub fn new(data: HashMap<Coords, usize>) -> Self {
        Self(Rc::new(data))
    }

    /// Create mapping from a list of ranges
    pub fn from_ranges(dimensions: Dimensions, ranges: &[MapRange]) -> Self {
        // evaluate the number of entries to generate
        let count = ranges.iter().fold(0usize, |acc, range| acc + range.size());

        // Convert the ranges into explicit mapping
        // between a tile position and a target index.
        let mut out = HashMap::with_capacity(count);
        for range in ranges.iter() {
            for i in range.start..range.end {
                let c = dimensions.to_coords(i);
                out.insert(c, range.target);
            }
        }

        Self::new(out)
    }
}
