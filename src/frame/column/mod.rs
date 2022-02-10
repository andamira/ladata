// src/frame/column
//
//! A *Column* is a homogeneous, collection of *cells*,
//!
//! It is generic in regards to how cells are internally stored.
//!
//! It is orthogonal to a *row*.
//!
//

use crate::frame::{
    cell::{CellData, CellFormat, CellType, InnerCellDataType},
    error::{DataFrameError, Result},
};

/// A column is generic over the the cell storage format.
#[derive(Debug, Clone)]
pub struct Column<F: CellFormat> {
    cell_type: CellType,
    vec: Vec<F>,
}

/// A `Column` that stores its cell data as bytes.
pub type ColumnB = Column<u8>;

/// A `Column` that stores its cell data as [`CellData.`]
pub type ColumnC = Column<CellData>;

impl<F: CellFormat> Column<F> {
    /// The type of the cells of this `Column`.
    #[inline]
    pub fn cell_type(&self) -> CellType {
        self.cell_type
    }

    /// Returns the size of the cell in bytes.
    pub fn cell_size(&self) -> usize {
        self.cell_type.size()
    }

    /// The number of cells in this `Column`.
    #[inline]
    pub fn num_cells(&self) -> usize {
        self.vec.len()
    }

    /// Returns `true` if this Column has no cells, or `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Returns the current size of this `Column`, in bytes.
    pub fn size(&self) -> usize {
        1 + self.cell_type.size() * self.vec.len()
    }
}

impl<F: CellFormat> Column<F> {
    /// Returns a new empty column.
    pub fn new_empty(cell_type: CellType) -> Self {
        Self {
            cell_type,
            vec: vec![],
        }
    }
}

impl Column<CellData> {
    /// Returns a new `ColumnC` from an iterable.
    pub fn from_iter<I, T>(i: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
        T: InnerCellDataType,
    {
        let vec: Vec<CellData> = i.into_iter().map(|d| d.to_cell_data()).collect();

        if vec.is_empty() {
            return Err(DataFrameError::Generic("empty iterator".into()));
        }
        let cell_type = vec[0].cell_type();

        Ok(Self { cell_type, vec })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::cell::CellDataNested;

    #[test]
    fn new_empty() -> Result<()> {
        let empty_u8 = Column::<CellData>::new_empty(CellType::U8);
        assert_eq![empty_u8.cell_type(), CellType::U8];
        assert_eq![empty_u8.num_cells(), 0];
        assert![empty_u8.is_empty()];

        let empty_f32_nested = Column::<CellDataNested>::new_empty(CellType::F32);
        assert_eq![empty_f32_nested.cell_type(), CellType::F32];
        Ok(())
    }
}
