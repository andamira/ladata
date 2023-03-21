// ladata::misc
//
//! Miscellaneous types.
//

// use super::error::LadataResult as Result;

mod count;
mod index;

pub use count::*;
pub use index::*;

/// An abstract Collection.
pub trait DataCollection {
    type Element;
    /// Returns `true` if the collection is empty, `false` if it's not,
    /// and `None` if it's not well defined for this data strucure;
    fn collection_is_empty(&self) -> Option<bool>;

    /// Returns `true` if the collection is full, `false` if it's not,
    /// and `None` if it's not well defined for this data structure.
    fn collection_is_full(&self) -> Option<bool>;

    ///
    fn collection_capacity(&self) -> usize;

    ///
    fn collection_len(&self) -> usize;

    ///
    fn collection_byte_len(&self) -> usize;
}

// /// An abstract dynamically-sized Collection.
// pub trait DynDataCollection: DataCollection {
//     fn collection_with_capacity(capacity: usize) -> Self;
//     fn collection_capacity(&self) -> usize;
//     fn collection_set_capacity(&mut self, capacity: usize) -> Result<()>;
//     //
//     fn collection_remaining_capacity(&self) -> usize {
//         self.collection_capacity() - self.collection_len()
//     }
// }
