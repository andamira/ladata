// ladata::list::stack::data
//
//! `DataStack` abstract data type.
//

use super::{Stack, Storage};
use crate::all::{DataCollection, LadataResult as Result};

/// An abstract Stack.
pub trait DataStack: DataCollection {
    ///
    fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element>;
    ///
    fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()>;
}

impl<T, S: Storage, const CAP: usize> DataCollection for Stack<T, S, CAP> {
    type Element = T;

    fn collection_is_empty(&self) -> Option<bool> {
        Some(self.is_empty())
    }
    fn collection_is_full(&self) -> Option<bool> {
        Some(self.is_full())
    }
    fn collection_capacity(&self) -> usize {
        CAP
    }
    fn collection_len(&self) -> usize {
        self.len()
    }
    fn collection_byte_len(&self) -> usize {
        todo![]
    }
}
