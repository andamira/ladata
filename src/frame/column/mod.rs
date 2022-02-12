// ladata::frame::column
//
//
//!
//

mod tests;

use crate::cell::{CellAble, CellData, CellStorage, CellType};
use crate::error::{DataError, DataResult};

/// A homogeneous collection of *cells*. Orthogonal to a [`Row`].
///
/// All *cell*s has to have the same [`CellType`].
///
// indexable
///
/// [`Row`]: crate::frame::Row
#[derive(Debug, Clone)]
pub struct Column<S: CellStorage> {
    cell_type: CellType,
    vec: Vec<S>,
}

/// A `Column` using *bytes* as storage.
//
// [`Bytes`]: crate::frame::FormatType::Bytes
pub type BytesColumn = Column<u8>;

/// A `Column` using `CellData` as storage.
//
// [`CellData`]: crate::frame::FormatType::CellData
pub type CellsColumn = Column<CellData>;

impl<S: CellStorage> Column<S> {
    /// Returns a new empty column.
    pub fn new_empty(cell_type: CellType) -> Self {
        Self {
            cell_type,
            vec: vec![],
        }
    }
}

impl Column<CellData> {
    /// Returns a new `Column<CellData>` from an iterable.
    //
    // FIX: rename
    pub fn from_iter<I, CA>(i: I) -> DataResult<Self>
    where
        I: IntoIterator<Item = CA>,
        CA: CellAble,
    {
        let vec: Vec<CellData> = i.into_iter().map(|d| d.to_cell_data()).collect();

        if vec.is_empty() {
            return Err(DataError::Generic("empty iterator".into()));
        }
        let cell_type = vec[0].cell_type();

        Ok(Self { cell_type, vec })
    }
}

impl<S: CellStorage> Column<S> {
    /// The type of the cells of this `Column`.
    #[inline]
    pub fn cell_type(&self) -> CellType {
        self.cell_type
    }

    /// Returns the size of the cell in bytes.
    #[inline]
    pub fn cell_size(&self) -> usize {
        self.cell_type.size()
    }

    /// The number of cells in this `Column`.
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns `true` if this Column has no cells, or `false` otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Returns the current size of the column, in bytes.
    #[inline]
    pub fn size(&self) -> usize {
        1 + self.cell_type.size() * self.vec.len()
    }
}
