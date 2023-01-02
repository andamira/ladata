// ladata::unit::nodata
//
//! Treating the unit type [`()`] to represent the absence of data.
//!
//! Mainly used for the non-`With` aliases.

use core::mem::{align_of, size_of};

use super::{DataBares, DataCells, DataCellsCopy, DataTypes, DataTypesCopy};

impl DataTypes for () {
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
impl DataCells for () {
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}

impl DataTypesCopy for () {}

impl DataCellsCopy for () {}

unsafe impl DataBares for () {}
