// ladata::list::array
//
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;
use crate::mem::Storage;

mod core_impls;
mod methods;
mod traits;

/// An array, backed by the core primitive [`array`].
pub struct Array<T, S: Storage, const LEN: usize> {
    array: S::Stored<[T; LEN]>,
}

/// An [`Array`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub type BoxedArray<T, const LEN: usize> = Array<T, Boxed, LEN>;

/// An [`Array`] stored in the stack.
pub type DirectArray<T, const LEN: usize> = Array<T, (), LEN>;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::BoxedArray;

    #[doc(inline)]
    pub use super::{traits::DataArray, Array, DirectArray};
}
