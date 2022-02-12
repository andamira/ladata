// ladata::frame::row
//
//
//!
//!
//

mod tests;

// use crate::error::{DataError, DataResult};
use crate::cell::{CellStorage, CellType};
use crate::series::{Series, SeriesApi};

/// A heterogeneous collection of *cells*.
/// Orthogonal to a [`Column`].
///
// named, ordered & indexable
//
/// [`Column`]: crate::series::Column
#[derive(Debug, Clone)]
pub struct Row<S: CellStorage> {
    cell_types: Vec<CellType>,
    series: Series<S>,
}

// /// A `Row` using *bytes* as storage.
// //
// // [`Bytes`]: crate::frame::FormatType::Bytes
// pub type BytesRow = Row<u8>;
//
// /// A `Row` using `CellData` as storage.
// //
// // [`CellData`]: crate::frame::FormatType::CellData
// pub type CellsRow = Row<CellData>;

impl<S: CellStorage> Row<S> {
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
            series: Series::new_empty(),
        }
    }
}
