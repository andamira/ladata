//
//!
//!

use core::{
    any::TypeId,
    mem::{align_of, size_of},
};

pub use crate::traits::{DataCells, DataTypes};

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
    fn type_align(&self) -> usize {
        align_of::<NoData>()
    }
    #[inline]
    fn type_size(&self) -> usize {
        size_of::<NoData>()
    }
    #[inline]
    fn type_id(&self) -> TypeId {
        TypeId::of::<NoData>()
    }
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}
impl DataCells for NoData {
    // CHECK: RETHINK API
    // #[inline]
    // fn data_type<T: DataTypes>(&self, _:T) -> DataTypeWith<T> {
    //     DataTypeWith::<T>::None
    // }
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}
