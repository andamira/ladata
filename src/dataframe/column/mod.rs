// src/dataframe/column
//!

use crate::dataframe::{
    cell::{CellType, InnerCellDataType},
    error::{DataFrameError, Result},
};

/// A homogeneous indexed collection of `Cell`s. Orthogonal to a `Row`.
///
pub struct Column {
    cell_type: CellType,
    vec: Vec<u8>,
}

impl Column {
    /// Returns a new column, filled with at least 1 cell.
    ///
    /// Errors if the `data` is empty.
    pub fn new<T: InnerCellDataType>(data: &[T]) -> Result<Self> {
        if data.is_empty() {
            return Err(DataFrameError::Generic("".into()));
        }

        let _cell_type = data[0].cell_type();

        Err(DataFrameError::Generic("".into()))
    }

    /// Returns a new empty column of the desired `cell_type`.
    pub fn new_empty(cell_type: CellType) -> Self {
        Self {
            cell_type,
            vec: vec![],
        }
    }

    /// Returns the number of cells contained in this column.
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns `true` if the column contains no cells, or `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Returns the type of the cells contained in this column.
    pub fn cell_type(&self) -> CellType {
        self.cell_type
    }

    /// Returns the size of the cell in bytes.
    pub fn cell_size(&self) -> usize {
        self.cell_type.size()
    }

    // TODO:
    // - add(&mut self)
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn new() {
    //     let bools = Column::new(CellType::Bool, &[true, false]).unwrap();
    //     assert_eq![2, bools.len()];
    // }
}
