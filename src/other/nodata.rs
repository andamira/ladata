// ladata::newtypes::nodata
//
//!

use core::mem::{align_of, size_of};

use crate::traits::{DataBares, DataCells, DataCellsCopy, DataTypes, DataTypesCopy};

/// A zero-sized type representing the absence of data.
///
/// Since it implements all traits it can represent the absence of
/// `DataType`, `DataCell` & `DataBare` indistinctly.
///
/// Mainly used for the non-`With` aliases of the
/// [`DataTypes`]*[[`Copy`][DataTypesCopy]]* and
/// [`DataCells`]*[[`Copy`][DataCellsCopy]]* default implementations.
#[derive(Debug, Copy, Clone)]
pub struct NoData;

impl DataTypes for NoData {
    #[inline]
    fn data_align(&self) -> usize {
        align_of::<NoData>()
    }
    #[inline]
    fn data_size(&self) -> usize {
        size_of::<NoData>()
    }
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}
impl DataCells for NoData {
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}

impl DataTypesCopy for NoData {}

impl DataCellsCopy for NoData {}

unsafe impl DataBares for NoData {}
