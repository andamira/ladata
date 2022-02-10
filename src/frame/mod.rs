// src/frame
//
//! A DataFrame could be regarded as a mixture between a Matrix and Table.
//!
//! Sources
//! - [Is a Dataframe Just a Table? by Yifan Wu (2020)](https://doi.org/10.4230/OASIcs.PLATEAU.2019.6)
//

pub mod cell;
pub use cell::{CellData, CellFormat, CellType, InnerCellDataType};

pub mod column;
pub use column::Column;

mod error;

pub mod handle;

mod row;
pub use row::Row;

/// A 2 dimensional collection of potentially heterogeneous data,
/// stored in a series of homogeneous columns.
///
/// A collection of columns of data with homogeneous underlying [`CellFormat`].
///
/// - indexable by columns and by rows.
/// - optionally ordered columns and rows.
///
/// A data frame is a mixture between a database and a matrix.
///
pub struct DataFrame<F: CellFormat> {
    num_cols: usize,
    name_cols: Vec<String>,
    type_cols: Vec<CellType>,
    columns: Vec<F>,
    num_rows: usize,
}

/// # Constructors
impl<F: CellFormat> DataFrame<F> {
    /// Returns an empty `DataFrame`.
    fn new_empty() -> DataFrame<F> {
        Self {
            num_cols: 0,
            name_cols: vec![],
            type_cols: vec![],
            columns: vec![],
            num_rows: 0,
        }
    }

}

#[cfg(test)]
mod tests {
}
