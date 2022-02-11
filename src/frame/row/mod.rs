// src/frame/row
//
//!
//!
//

use crate::frame::{
    cell::{AcceptableData, CellData, CellType},
    error::{DataFrameError, Result},
    format::Format,
};

/// A heterogeneous collection of *cells*.
/// Orthogonal to a [`Column`].
///
// named, ordered & indexable
//
/// Each *cell* can have a different [`CellType`].
///
/// [`Column`]: crate::frame::Column
#[derive(Debug, Clone)]
pub struct Row<F: Format> {
    cell_types: Vec<CellType>,
    vec: Vec<F>,
}

/// A [`Row`] using [`Bytes`] to store *cells*.
///
/// [`Bytes`]: crate::frame::FormatType::Bytes
pub type BytesRow = Row<u8>;

/// A [`Row`] using [`CellData`] to store *cells*.
///
/// [`CellData`]: crate::frame::FormatType::CellData
pub type CellsRow = Row<CellData>;

impl<F: Format> Row<F> {
    // WIP
    /// Returns a new empty row.
    ///
    // pub fn new_empty<I>(cell_types: I) -> Self
    // pub fn new_empty<I>(cell_types: I) -> Self
    //     where I: IntoIterator<Item = CellType> {
    pub fn new_empty() -> Self {
        Self {
            // cell_types: cell_types.into_iter().collect(),
            cell_types: vec![],
            vec: vec![],
        }
    }
}
