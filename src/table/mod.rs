//!
//!
//!
//!

// TODO:
// - sync the rows on operations
//   - support non-existing indices when retrieving/printing a row.

#![allow(unused_variables, dead_code)]

mod column;
pub use column::{Categorical, Column, FloatVec, IntVec, Numerical};

// use num::{PrimInt, Float};

///
///
///
#[derive(Debug, Default)]
pub struct Table {
    cols: Vec<Column>,
    header: Vec<String>,
    rows: Option<usize>,
}

// impl Default for Table {
// }
//
impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of columns.
    pub fn cols(&self) -> usize {
        self.cols.len()
    }

    /// Adds a column to the table.
    ///
    /// # Example
    /// ```
    /// # use ladata::Table;
    /// let mut t = Table::new();
    /// t.add_col("bytes", vec![23_u8, 53, 32]);
    /// t.add_col("floats", vec![23., 32.3, 0.]);
    /// ```
    // TODO: return the index
    pub fn add_col<C: Into<Column>>(&mut self, header: &str, col: C) {
        self.cols.push(col.into());
        self.header.push(header.into());
    }

    // TODO: result
    pub fn del_col(&mut self, col: usize) {}
}
