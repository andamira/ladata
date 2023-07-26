// ladata::list::array::traits
//
//!
//

use crate::{
    error::{LadataError as Error, LadataResult as Result},
    list::Array,
    mem::Storage,
    misc::DataCollection,
};

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
