// ladata::unit::nodata
//
//! Treating the unit type [`()`] to represent the absence of data.
//!
//! Mainly used for the non-`With` aliases.

use core::mem::{align_of, size_of};

use super::{DataType, DataTypeCopy, DataUnit, DataUnitCopy};

#[cfg(not(feature = "safe"))]
use super::RawData;

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
impl DataUnit for () {
    #[inline]
    fn is_copy(&self) -> bool {
        true
    }
}

impl DataTypeCopy for () {}

impl DataUnitCopy for () {}

#[cfg(not(feature = "safe"))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
unsafe impl RawData for () {}
