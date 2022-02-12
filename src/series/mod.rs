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

mod tests;

use crate::cell::CellStorage;

/// A one dimensional collection of *cells*.
#[derive(Debug, Clone)]
pub struct Series<S: CellStorage> {
    vec: Vec<S>,
}
