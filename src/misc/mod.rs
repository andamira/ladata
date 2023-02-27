// ladata::misc
//
//! Miscellaneous types.
//

use super::error::LadataResult as Result;

mod index;
pub use index::*;

/// An abstract Collection.
pub trait CollectionAdt {
    type Element;
    fn collection_is_empty(&self) -> bool;
    fn collection_len(&self) -> usize;
    fn collection_byte_len(&self) -> usize;
}

/// An abstract dynamically-sized Collection.
pub trait DynCollectionAdt: CollectionAdt {
    fn collection_with_capacity(capacity: usize) -> Self;
    fn collection_capacity(&self) -> usize;
    fn collection_set_capacity(&mut self, capacity: usize) -> Result<()>;
    //
    fn collection_remaining_capacity(&self) -> usize {
        self.collection_capacity() - self.collection_len()
    }
}
