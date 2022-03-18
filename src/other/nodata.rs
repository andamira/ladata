// ladata::newtypes::nodata
//
//!

use core::mem::{align_of, size_of};

use crate::traits::{DataLones, DataTypes, DataTypesCopy, DataUnits, DataUnitsCopy};

/// A zero-sized type representing the absence of data.
///
/// Since it implements all traits it can represent the absence of
/// `DataType`, `DataUnit` & `DataLone` indistinctly.
///
/// Mainly used for the non-`With` aliases of the
/// [`DataTypes`]*[[`Copy`][DataTypesCopy]]* and
/// [`DataUnits`]*[[`Copy`][DataUnitsCopy]]* default implementations.
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
impl DataUnits for NoData {
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}

impl DataTypesCopy for NoData {}

impl DataUnitsCopy for NoData {}

unsafe impl DataLones for NoData {}
