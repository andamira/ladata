// ladata::frame
//
//
//! Data frames.
//!
//! A `DataFrame` could be regarded as a mixture between a Matrix and Table.
//!
//

mod tests;

use crate::cell::{CellAble, CellData, CellStorage, CellType};
// use crate::series::{Column, Row};
use std::collections::HashMap;

// /// A `DataFrame` using *bytes* as storage.
// //
// // [`Bytes`]: crate::frame::FormatType::Bytes
// pub type BytesDataFrame = DataFrame<u8>;
//
// /// A `DataFrame` using `CellData` as storage.
// //
// // [`CellData`]: crate::frame::FormatType::CellData
// pub type CellsDataFrame = DataFrame<CellData>;

/// A tabular collection of potentially heterogeneous data
/// stored [`Column`]-wise.
///
/// [`Column`]: crate::series::Column
pub struct DataFrame<S: CellStorage> {
    num_cols: usize,
    name_cols: Vec<String>,
    type_cols: Vec<CellType>,
    columns: Vec<S>,
    num_rows: usize,
}

/// # Constructors
impl<S: CellStorage> DataFrame<S> {
    /// Returns an empty `DataFrame`.
    pub fn new_empty() -> DataFrame<S> {
        Self {
            num_cols: 0,
            name_cols: vec![],
            type_cols: vec![],
            columns: vec![],
            num_rows: 0,
        }
    }
}

impl DataFrame<CellData> {
    /// Returns a new `Dataframe<CellData>` from the provided `HashMap`.
    ///
    /// Each key in the hashmap represents the name of a column
    // pub fn from_hashmap<CA>(hashmap: HashMap<String, Vec<CA>>) -> Self
    pub fn from_hashmap<I, CA>(hashmap: HashMap<String, I>) -> Self
    where
        I: IntoIterator<Item = CA>,
        CA: CellAble,
    {
        if hashmap.is_empty() {
            return DataFrame::new_empty();
        }

        let mut num_cols = 0;
        let mut name_cols = vec![];
        let mut type_cols = vec![];
        let mut columns = vec![];
        let mut num_rows = 0;

        // WIP TODO

        for (k, _v) in hashmap {
            num_cols += 1;
            name_cols.push(k);

            // if v.is_empty() {
            //
            // } else {
            //
            // }
        }

        Self {
            num_cols,
            name_cols,
            type_cols,
            columns,
            num_rows,
        }
    }
}

/// # Information
impl<S: CellStorage> DataFrame<S> {
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
