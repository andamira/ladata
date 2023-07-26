// ladata::list::array
//
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

use crate::{
    all::DataCollection,
    error::{LadataError as Error, LadataResult as Result},
    mem::Storage,
};

mod impls;
mod methods;

/// An abstract Array.
///
/// - <https://en.wikipedia.org/wiki/Array_(data_type)#Abstract_arrays>
pub trait DataArray: DataCollection {
    ///
    fn array_get(&self, index: usize) -> Result<&<Self as DataCollection>::Element>;
    ///
    fn array_set(&mut self, index: usize, element: <Self as DataCollection>::Element)
        -> Result<()>;
}

/// An array, backed by the core primitive [`array`].
pub struct Array<T, S: Storage, const LEN: usize> {
    array: S::Stored<[T; LEN]>,
    _phantom: PhantomData<T>,
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
    pub use super::{Array, DataArray, DirectArray};
}

impl<T, S: Storage, const LEN: usize> DataCollection for Array<T, S, LEN> {
    type Element = T;
    fn collection_is_empty(&self) -> Option<bool> {
        None
    }
    fn collection_is_full(&self) -> Option<bool> {
        None
    }
    fn collection_capacity(&self) -> usize {
        LEN
    }
    fn collection_len(&self) -> usize {
        self.len()
    }
}

impl<T, S: Storage, const LEN: usize> DataArray for Array<T, S, LEN> {
    fn array_get(&self, index: usize) -> Result<&<Self as DataCollection>::Element> {
        if let Some(e) = self.get(index) {
            Ok(e)
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    fn array_set(
        &mut self,
        index: usize,
        element: <Self as DataCollection>::Element,
    ) -> Result<()> {
        if let Some(e) = self.get_mut(index) {
            *e = element;
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
}
