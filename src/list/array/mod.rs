// ladata::list::array
//
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

use core::marker::PhantomData;

#[cfg(feature = "std")]
use crate::mem::Boxed;
use crate::{all::CollectionAdt, error::LadataResult as Result, mem::Storage};

#[cfg(feature = "bv")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "bv")))]
pub mod bit;
// #[cfg(feature = "bv")]
// #[doc(inline)]
// pub use bit::*;

mod methods;
mod std_impls;

/// An abstract Array.
///
/// - <https://en.wikipedia.org/wiki/ArrayAdt_(data_type)#Abstract_arrays>
pub trait ArrayAdt: CollectionAdt {
    ///
    fn array_get(&mut self, index: usize) -> Result<<Self as CollectionAdt>::Element>;
    ///
    fn array_set(&mut self, index: usize, element: <Self as CollectionAdt>::Element) -> Result<()>;
}

/// A generic array, backed by the core primitive [`array`].
pub struct Array<T, S: Storage, const LEN: usize> {
    array: S::Stored<[T; LEN]>,
    _phantom: PhantomData<T>,
}

/// An [`Array`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedArray<T, const LEN: usize> = Array<T, Boxed, LEN>;

/// An [`Array`] stored in the stack.
pub type DirectArray<T, const LEN: usize> = Array<T, (), LEN>;
