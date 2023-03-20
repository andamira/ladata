// ladata::list::array
//
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

use core::{marker::PhantomData, mem::size_of};

#[cfg(feature = "std")]
use crate::mem::Boxed;

use crate::{
    all::CollectionAdt,
    error::{LadataError as Error, LadataResult as Result},
    mem::Storage,
};

pub mod bit;

mod methods;
mod std_impls;

/// An abstract Array.
///
/// - <https://en.wikipedia.org/wiki/ArrayAdt_(data_type)#Abstract_arrays>
pub trait ArrayAdt: CollectionAdt {
    ///
    fn array_get(&self, index: usize) -> Result<&<Self as CollectionAdt>::Element>;
    ///
    fn array_set(&mut self, index: usize, element: <Self as CollectionAdt>::Element) -> Result<()>;
}

/// An array, backed by the core primitive [`array`].
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

pub use all::*;
pub(crate) mod all {
    #[cfg(feature = "std")]
    pub use super::{bit::BoxedBitArray, BoxedArray};
    #[doc(inline)]
    pub use super::{
        bit::{BitArray, DirectBitArray},
        Array, ArrayAdt, DirectArray,
    };
}

impl<T, S: Storage, const LEN: usize> CollectionAdt for Array<T, S, LEN> {
    type Element = T;
    fn collection_is_empty(&self) -> bool {
        self.is_empty()
    }
    fn collection_len(&self) -> usize {
        self.len()
    }
    fn collection_byte_len(&self) -> usize {
        self.len() * size_of::<T>()
    }
}

impl<T, S: Storage, const LEN: usize> ArrayAdt for Array<T, S, LEN> {
    fn array_get(&self, index: usize) -> Result<&<Self as CollectionAdt>::Element> {
        if let Some(e) = self.get(index) {
            Ok(e)
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    fn array_set(&mut self, index: usize, element: <Self as CollectionAdt>::Element) -> Result<()> {
        if let Some(e) = self.get_mut(index) {
            *e = element;
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
}
