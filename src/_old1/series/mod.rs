// ladata::series
// pub
//
//! A series of data units.
//!
//! An one dimensional, indexable collection of data cells.
//!
//! - Optionally labeled.
//! - Optionally enforcing homogeneous data.
//
// TODO:
// - labeled vs unlabeled series (associated vec)
// - optionally homogeneous enforced (by type or by Apitrait)?

mod api;
pub use api::SeriesApi;

mod column;
// pub use column::{BytesColumn, CellsColumn, Column};
pub use column::Column;

mod row;
// pub use row::{BytesRow, CellsRow, Row};
pub use row::Row;

mod tests;

use crate::cell::{CellStorage, CellType};
// use crate::cell::{CellAble, CellData, CellStorage, CellType};
// use crate::error::{DataError, DataResult};

/// A one dimensional collection of *cells*.
#[derive(Clone, Debug, Default)]
pub struct Series<S: CellStorage> {
    vec: Vec<S>,
}

/// A one dimensional collection of *cells*,
/// dynamically enforced to have a single [`CellType`].
#[derive(Clone, Debug, Default)]
pub struct MonoSeries<S: CellStorage> {
    cell_type: CellType,
    vec: Vec<S>,
}
