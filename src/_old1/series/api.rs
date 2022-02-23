// ladata::series::api
//
//
//!
//!
//

use crate::{
    cell::{CellStorage, CellType},
    series::{Series, MonoSeries}
};

/// Common API for all kinds of `Series`.
pub trait SeriesApi: Default {
    /// Returns a new empty series.
    fn new_empty() -> Self {
        Self::default()
    }

    // RETHINK DESIGN
    // same API for gettint a single `CellType` or a list / set of `CellType`s.
    // simplest would be a slice.
    // - cell_type_set() -> &[CellType]
    // - cell_type_list() -> &[CellType]
    // - cell_type() -> DataResult<CellType> // hmmm
    //   - errs if there are multiple types?
    //
    //
    // option B would be to impl cell_type in another trait
    // option C would be to make them work differently
    //   what would cell_type() would return for Series ?
    //   1. Err(DataError::SingleTypeExpected)
    //   2. dynamically try to reduce to 1 type, if not, Err(MultipleTypes)
    //   3. create & return a new enum that could be used for
    //      ordered, unordered, set & repeated mixed types… :)
    //      enum CellTypeNumber { None, Single(CellType), Mixed(&[CellType]) }
    //      enum Plural { None, Single(CellType), Mixed(&[CellType]) }
    //
    //
    
    // PROS: 
    // - 
    //
    // CONS:
    // - Semi(redundant with CellType::None)
    fn cell_type() -> Option<CellType>

    // fn cell_type() -> &[CellType];

    /*

    fn cell_types_list()
    fn cell_types_set()
    */
}

impl<S: CellStorage> SeriesApi for MonoSeries<S> {
}

impl<S: CellStorage> SeriesApi for Series<S> {
}
