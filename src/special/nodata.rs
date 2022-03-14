// ladata::newtypes::nodata
//
//!

use core::mem::{align_of, size_of};

use crate::traits::{DataCells, DataCellsCopy, DataTypes, DataTypesCopy, DataUnsafeCells};

/// A zero-sized type representing the absence of data.
///
/// Since it implements all traits it can represent the absence of
/// `DataType`, `DataCell` & `DataUnsafeCell` indistinctly.
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
    // #[inline]
    // fn type_id(&self) -> TypeId {
    //     TypeId::of::<NoData>()
    // }
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}
impl DataCells for NoData {
    // WIP ±
    // type TYPE = NoData;
    // #[inline]
    // fn data_type(&self) -> Self::TYPE {
    //     Self::TYPE
    // }
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}

impl DataTypesCopy for NoData {}

impl DataCellsCopy for NoData {}

unsafe impl DataUnsafeCells for NoData {}
