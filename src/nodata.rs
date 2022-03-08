//
//!
//!

use core::mem::{align_of, size_of};

pub use crate::traits::{DataCells, DataCellsCopy, DataTypes, DataTypesCopy, DataUnsafeCells};

/// Represents the absence of data.
///
/// Implements all the defined traits.
///
/// It's used, for example, for the non-`With` type aliases of
/// [`DataTypes`]|[`DataCells`] implementors.
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
impl DataTypesCopy for NoData {}
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
impl DataCellsCopy for NoData {}

unsafe impl DataUnsafeCells for NoData {}
