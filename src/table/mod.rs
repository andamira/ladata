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

    // TODO:
    // pub fn add_col(&mut self, header: &str, col: Into<Column>) {

    // TODO: return the index
    pub fn add_col<C: Into<Column>>(&mut self, header: &str, col: C) {
        self.cols.push(col.into());
        self.header.push(header.into());
    }

    // TODO: result
    pub fn del_col(&mut self, col: usize) {}
}
