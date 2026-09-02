use ndarray::{Array2, Array3, Ix2, Ix3};
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

impl Config {
    /// Reformat the text into pages of bytes to be encoded into source files
    pub fn reformat(&self, text: &str) -> Paragraph {
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
        for (i, window) in lines.windows(self.lines_per_page).enumerate() {
            lines_per_page.push(window.len() as u16);

            // iterate over the lines
            for (j, line) in window.iter().enumerate() {
                let line = line.as_ref();
                line_lengths[Ix2(i, j)] = line.len() as u16;

                // iterate over the characters
                for (k, c) in line.char_indices() {
                    if let Some(&byte) = self.mapping.get(&c) {
                        data[Ix3(i, j, k)] = byte;
                    }
                }
            }
        }

        // Output the paragraph
        Paragraph {
            data,
            lines_per_page,
            line_lengths,
        }
    }
}

/// Processed data stored in matrices
#[derive(Debug)]
pub struct Paragraph {
    /// raw matrix containing the characters
    data: Array3<u8>,

    /// Number of lines per page
    lines_per_page: Vec<u16>,

    /// Length of the lines
    line_lengths: Array2<u16>,
}

impl Paragraph {
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
}
