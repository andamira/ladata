// ladata::list::array
//
//! Arrays are linear lists, sequentially allocated, statically sized.
//

use core::marker::PhantomData;

#[cfg(feature = "std")]
use crate::mem::Boxed;
use crate::mem::Storage;

mod methods;
mod std_impls;

/// An array, backed by a primitive [`array`][array].
pub struct Array<T, S: Storage, const LEN: usize> {
    array: S::Container<[T; LEN]>,
    _phantom: PhantomData<T>,
}

/// An [`Array`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedArray<T, const LEN: usize> = Array<T, Boxed, LEN>;

/// An [`Array`] stored in the stack.
pub type RawArray<T, const LEN: usize> = Array<T, (), LEN>;
