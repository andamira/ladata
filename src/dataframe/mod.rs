// src/dataframe
//
//! A DataFrame could be regarded as a mixture between a Matrix and Table.
//!
//! Sources
//! - [Is a Dataframe Just a Table? by Yifan Wu (2020)](https://doi.org/10.4230/OASIcs.PLATEAU.2019.6)

pub mod cell;
mod column;
mod error;
pub mod handle;
mod row;

pub use cell::{CellData, CellType};
pub use column::Column;
pub use row::Row;

pub struct DataFrame {
    num_cols: usize,
    name_cols: Vec<String>,
    type_cols: Vec<CellType>,
    columns: Vec<Column>,
    num_rows: usize,
}

impl DataFrame {
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

    /// Returns a reference to the `nth` `Column`.
    pub fn col(&self, index: usize) -> &Column {
        &self.columns[index]
    }

    /// Returns a mutable reference to the `nth` `Column`.
    pub fn col_mut(&mut self, index: usize) -> &mut Column {
        &mut self.columns[index]
    }

    /// Returns the number of rows.
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }
}
