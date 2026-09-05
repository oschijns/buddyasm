use ndarray::{Array1, Array2, Array3, Ix1, Ix2, Ix3};
use serde::Serialize;
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

    /// Mapping between common ASCII characters and the tile index to output.
    /// Use the character code as an index to find the corresponding byte.
    pub mapping: Vec<u8>,
}

/// Text splitted into pages of lines
#[derive(Debug, Serialize)]
pub struct Text(Vec<Vec<String>>);

/// Convert splitted text into a 3D matrix of bytes
#[derive(Debug)]
pub struct TextMatrix {
    /// Number of lines per page
    page_lines: Array1<u16>,

    /// Length of the lines in each page
    line_lengths: Array2<u16>,

    /// raw matrix containing the characters
    data: Array3<u8>,
}

impl Config {
    /// Create a config from a width and height
    pub fn new(line_length: usize, lines_per_page: usize) -> Self {
        Self {
            line_length,
            lines_per_page,
            default_byte: 0,
            mapping: (0u8..0x7F).into_iter().collect::<Vec<_>>(),
        }
    }

    /// Split the provided text into pages of lines
    pub fn split(&self, text: &str) -> Text {
        const FORM_FEED: char = 0x0C as char;

        // Store pages to convert into a matrix
        let mut collect = Vec::new();

        // textwarp options
        let opt = Options::new(self.line_length);

        // Split the text into blocks using the Form Feed control character
        for block in text.split(FORM_FEED) {
            // Cut the block into multiple lines
            let lines = textwrap::wrap(block, opt.clone());

            // collect pages for this block of text
            for page in lines.chunks(self.lines_per_page) {
                let lines = page.iter().map(|row| row.to_string()).collect::<Vec<_>>();
                collect.push(lines);
            }
        }
        Text(collect)
    }

    /// From the splitted text, convert it into a 3D matrix
    pub fn convert(&self, text: &Text) -> TextMatrix {
        let page_count = text.0.len();

        // Prepare the matrix
        let mut page_lines = Array1::zeros(page_count);
        let mut line_lengths = Array2::zeros((page_count, self.lines_per_page));
        let mut data = Array3::from_elem(
            (page_count, self.lines_per_page, self.line_length),
            self.default_byte,
        );

        // Iterate over the splitted text to populate the matrix
        for (i, page) in text.0.iter().enumerate() {
            page_lines[Ix1(i)] = page.len() as u16;
            for (j, line) in page.iter().enumerate() {
                line_lengths[Ix2(i, j)] = line.len() as u16;
                for (k, chr) in line.chars().enumerate() {
                    data[Ix3(i, j, k)] = self.mapping[chr as usize];
                }
            }
        }

        TextMatrix {
            page_lines,
            line_lengths,
            data,
        }
    }
}
