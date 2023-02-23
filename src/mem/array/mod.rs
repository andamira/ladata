// ladata::mem::array
//
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

use core::marker::PhantomData;

#[cfg(feature = "std")]
use crate::mem::Boxed;
use crate::mem::Storage;

mod methods;
mod std_impls;

/// A generic array, backed by the core primitive [`array`].
pub struct CoreArray<T, S: Storage, const LEN: usize> {
    array: S::Stored<[T; LEN]>,
    _phantom: PhantomData<T>,
}

/// A [`CoreArray`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedArray<T, const LEN: usize> = CoreArray<T, Boxed, LEN>;

/// A [`CoreArray`] stored in the stack.
pub type DirectArray<T, const LEN: usize> = CoreArray<T, (), LEN>;
