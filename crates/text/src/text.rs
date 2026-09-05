use ndarray::{Array2, Array3, Ix2, Ix3};
use serde::Serialize;
use std::collections::HashMap;
use textwrap::Options;

/// Define the size of the page of text to display
#[derive(Debug)]
pub struct Config {
    /// Number of characters per line
    pub line_length: usize,

    /// Number of lines that can be displayed at once
    pub lines_per_page: usize,

    /// Default byte value to fill the matrix with
    pub default_byte: u8,

    /// Mapping between common ASCII characters and the tile index to output
    pub mapping: HashMap<char, u8>,
}

/// Processed data stored in matrices
#[derive(Debug, Serialize)]
pub struct TextMatrix {
    /// raw matrix containing the characters
    data: Array3<u8>,

    /// Number of lines per page
    lines_per_page: Vec<u16>,

    /// Length of the lines
    line_lengths: Array2<u16>,
}

/// Processed data stored in matrices
#[derive(Debug, Serialize)]
pub struct Text {
    data: Vec<Vec<String>>,
}

impl TextMatrix {
    /// return the number of lines for the given page
    pub fn page_line_count(&self, page: usize) -> usize {
        if page < self.lines_per_page.len() {
            self.lines_per_page[page] as usize
        } else {
            0
        }
    }

    /// return the length of the line of the given page
    pub fn line_length(&self, page: usize, line: usize) -> usize {
        if page < self.lines_per_page.len() && line < self.line_lengths.dim().1 {
            self.line_lengths[Ix2(page, line)] as usize
        } else {
            0
        }
    }

    /// Reformat the matrix into text to be serialized using serde
    pub fn to_serde(&self) -> Text {
        let (pc, _, cc) = self.data.dim();

        // Buffer to collect bytes and convert them into a string
        let mut buffer = Vec::with_capacity(cc);

        // Prepare the buffer to store the data
        let mut data = Vec::with_capacity(pc);
        for p in 0..pc {
            let lc = self.lines_per_page[p] as usize;

            // Allocate a buffer to store the lines of the page
            let mut page = Vec::with_capacity(lc);
            for l in 0..lc {
                let cc = self.line_lengths[Ix2(p, l)] as usize;

                // Iterate over the characters
                buffer.clear();
                for c in 0..cc {
                    buffer.push(self.data[Ix3(p, l, c)]);
                }
                let line = String::from_utf8_lossy(&buffer).to_string();
                page.push(line);
            }
            data.push(page);
        }
        Text { data }
    }
}

impl Config {
    /// Create a config from a width and height
    pub fn new(line_length: usize, lines_per_page: usize) -> Self {
        // Cover all ASCII characters
        let mut mapping = HashMap::with_capacity(0x7F - 0x20);
        for i in 0x20..0x7Fu8 {
            mapping.insert(i as char, i);
        }

        Self {
            line_length,
            lines_per_page,
            default_byte: 0,
            mapping,
        }
    }

    /// Reformat the text into pages of bytes to be encoded into source files
    pub fn reformat(&self, text: &str) -> TextMatrix {
        // Cut the text into multiple lines
        let lines = textwrap::wrap(text, Options::new(self.line_length));

        // Cut the paragraph into pages
        let page_count = lines.len().div_ceil(self.lines_per_page);

        // Prepare the matrix
        let mut data = Array3::from_elem(
            (page_count, self.lines_per_page, self.line_length),
            self.default_byte,
        );
        let mut lines_per_page = Vec::with_capacity(page_count);
        let mut line_lengths = Array2::zeros((page_count, self.lines_per_page));

        // iterate over the pages
        for (i, page) in lines.chunks(self.lines_per_page).enumerate() {
            lines_per_page.push(page.len() as u16);

            // iterate over the lines
            for (j, line) in page.iter().enumerate() {
                //println!("LINE: {}", line);
                let line = line.as_ref();
                line_lengths[Ix2(i, j)] = line.len() as u16;

                // iterate over the characters
                for (k, chr) in line.char_indices() {
                    if let Some(&byte) = self.mapping.get(&chr) {
                        data[Ix3(i, j, k)] = byte;
                    }
                }
            }
        }

        // Output the paragraph
        TextMatrix {
            data,
            lines_per_page,
            line_lengths,
        }
    }
}
