// ladata::series::api
//
//
//!
//!
//

use crate::{
    cell::CellStorage,
    series::{Series, MonoSeries}
};

/// Common API for all kinds of `Series`.
pub trait SeriesApi: Default {
    /// Returns a new empty series.
    fn new_empty() -> Self {
        Self::default()
    }
}

impl<S: CellStorage> SeriesApi for MonoSeries<S> {
}

impl<S: CellStorage> SeriesApi for Series<S> {
}
