// src/frame
//
//! A DataFrame could be regarded as a mixture between a Matrix and Table.
//!
//
#![allow(unused_mut, dead_code)] // TEMP

use std::collections::HashMap;

pub mod cell;
pub use cell::{AcceptableData, CellData, CellType};

mod column;
#[doc(inline)]
pub use column::{BytesColumn, CellsColumn, Column};

mod format;
pub use format::Format; // FormatType

mod error;

pub mod handle;

mod row;
pub use row::{BytesRow, CellsRow, Row};

/// A [`DataFrame`] using [`Bytes`] to store *cells*.
///
/// [`Bytes`]: crate::frame::FormatType::Bytes
pub type BytesDataFrame = DataFrame<u8>;

/// A [`DataFrame`] using [`CellData`] to store *cells*.
///
/// [`CellData`]: crate::frame::FormatType::CellData
pub type CellsDataFrame = DataFrame<CellData>;

/// A tabular collection of potentially heterogeneous data,
/// stored in a series of [`Column`]s.
//
// - indexable by columns and by rows.
// - optionally ordered columns and rows.
//
// A data frame is a mixture between a database and a matrix.
//
pub struct DataFrame<F: Format> {
    num_cols: usize,
    name_cols: Vec<String>,
    type_cols: Vec<CellType>,
    columns: Vec<F>,
    num_rows: usize,
}

/// # Constructors
impl<F: Format> DataFrame<F> {
    /// Returns an empty `DataFrame`.
    pub fn new_empty() -> DataFrame<F> {
        Self {
            num_cols: 0,
            name_cols: vec![],
            type_cols: vec![],
            columns: vec![],
            num_rows: 0,
        }
    }
}

/// # Information
impl<F: Format> DataFrame<F> {
    /// Returns the number of columns.
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    /// Returns the name of all the columns.
    pub fn name_cols(&self) -> &[String] {
        &self.name_cols
    }

    /// Returns the name of the `nth` column.
    pub fn name_col(&self, index: usize) -> &str {
        &self.name_cols[index]
    }

    /// Returns the type of all the columns.
    pub fn type_cols(&self) -> &[CellType] {
        &self.type_cols
    }

    /// Returns the type of the `nth` column.
    pub fn type_col(&self, index: usize) -> CellType {
        self.type_cols[index]
    }

    /// Returns the number of rows.
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    /// Returns the current size of the dataframe, in bytes.
    pub fn size(&self) -> usize {
        let mut s = 0_usize;
        for c in &self.type_cols {
            s += c.size() * self.num_rows
        }
        s
    }
}

#[cfg(test)]
mod tests {
}