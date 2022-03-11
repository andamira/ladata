// ladata::nodata
//
//!
//!

use core::mem::{align_of, size_of};

pub use crate::traits::{DataCells, DataCellsCopy, DataTypes, DataTypesCopy, DataUnsafeCells};

/// Represents the absence of data of any kind.
///
/// It's zero-sized, and it implements all traits so it can represent the
/// absence of DataType, DataCell & DataUnsafeCell indistinctly.
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

unsafe impl DataUnsafeCells for NoData {}
