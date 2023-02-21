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

/// An array, backed by a [`primitive array`][array].
pub struct Array<T, S: Storage, const LEN: usize> {
    array: S::Stored<[T; LEN]>,
    _phantom: PhantomData<T>,
}

/// An [`Array`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedArray<T, const LEN: usize> = Array<T, Boxed, LEN>;

/// An [`Array`] stored in the stack.
pub type RawArray<T, const LEN: usize> = Array<T, (), LEN>;
