// ladata::other::nodata
//
//! Treating the unit type [`()`] to represent the absence of data.
//!
//! Mainly used for the non-`With` aliases.

use core::mem::{align_of, size_of};

use crate::traits::{DataBare, DataCell, DataCellCopy, DataType, DataTypeCopy};

impl DataType for () {
    #[inline]
    fn data_align(&self) -> usize {
        align_of::<()>()
    }
    #[inline]
    fn data_size(&self) -> usize {
        size_of::<()>()
    }
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}
impl DataCell for () {
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}

impl DataTypeCopy for () {}

impl DataCellCopy for () {}

unsafe impl DataBare for () {}
