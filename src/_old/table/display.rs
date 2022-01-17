//!

use std::fmt;

use super::Table;

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ncols = self.cols.len();
        if ncols == 0 {
            return Ok(());
        }

        // calculate the max width of each column (without separators)

        let mut cols_width = vec![0; ncols];
        for (n, h) in self.header.iter().enumerate() {
            cols_width[n] = h.chars().count();
        }

        for (ncol, col) in self.cols.iter().enumerate() {

            // WIP
            // for row in col.iter() {
            // }
        }

        // dbg![cols_width]; // DEBUG

        let mut table_str = String::new();

        // prepare the header

        let mut header_str = String::new();
        for h in &self.header {
            vertical_separator(&mut header_str, 0, 0);
            header_str += h;
        }
        vertical_separator(&mut header_str, 0, 0);

        let header_len = header_str.chars().count();
        table_str += &format!["{}\n", header_str];

        horizontal_line(&mut table_str, header_len);

        write!(f, "{}", table_str)
    }
}

/// draw a vertical separator in `string`, with optional spaces at left & right.
fn vertical_separator(string: &mut String, lspace: usize, rspaces: usize) {
    *string += "|";
}

/// draw a horizontal line
fn horizontal_line(string: &mut String, len: usize) {
    *string += &"-".repeat(len);
}
