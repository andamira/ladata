// src/frame/column
//
//!
//

use crate::frame::{
    cell::{AcceptableData, CellData, CellType},
    error::{DataFrameError, Result},
    format::Format,
};

/// A homogeneous collection of *cells*. Orthogonal to a [`Row`].
///
/// All *cell*s has to have the same [`CellType`].
///
// indexable
///
/// [`Row`]: crate::frame::Row
#[derive(Debug, Clone)]
pub struct Column<F: Format> {
    cell_type: CellType,
    vec: Vec<F>,
}

/// A [`Column`] using [`Bytes`] to store *cells*.
///
/// [`Bytes`]: crate::frame::FormatType::Bytes
pub type BytesColumn = Column<u8>;

/// A [`Column`] using [`CellData`] to store *cells*.
///
/// [`CellData`]: crate::frame::FormatType::CellData
pub type CellsColumn = Column<CellData>;

impl<F: Format> Column<F> {
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
    pub fn from_iter<I, AD>(i: I) -> Result<Self>
    where
        I: IntoIterator<Item = AD>,
        AD: AcceptableData,
    {
        let vec: Vec<CellData> = i.into_iter().map(|d| d.to_cell_data()).collect();

        if vec.is_empty() {
            return Err(DataFrameError::Generic("empty iterator".into()));
        }
        let cell_type = vec[0].cell_type();

        Ok(Self { cell_type, vec })
    }
}

impl<F: Format> Column<F> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() -> Result<()> {
        let empty_u8 = Column::<CellData>::new_empty(CellType::U8);
        assert_eq![empty_u8.cell_type(), CellType::U8];
        assert_eq![empty_u8.len(), 0];
        assert![empty_u8.is_empty()];

        Ok(())
    }

    #[test]
    fn from_iter() -> Result<()> {
        let bools = Column::<CellData>::from_iter(&[true, false])?;
        assert_eq![2, bools.len()];
        assert_eq![CellType::Bool, bools.cell_type()];
        assert![!bools.is_empty()];

        let floats = Column::<CellData>::from_iter([0.4_f32])?;
        assert_eq![1, floats.len()];
        assert_eq![CellType::F32, floats.cell_type()];

        let ints = Column::<CellData>::from_iter(vec![4 ^ 10, 0, -8192])?;
        assert_eq![3, ints.len()];
        assert_eq![CellType::I32, ints.cell_type()];

        Ok(())
    }
}
