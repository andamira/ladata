// ladata::list::stack
//
//! Stacks ar linear lists for which all accesses are made from one end.
//!
//! <https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>
//

use crate::{error::LadataResult as Result, list::Array, mem::Storage, misc::CollectionAdt};

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// An abstract Stack.
pub trait StackAdt: CollectionAdt {
    ///
    fn stack_pop(&mut self) -> Result<<Self as CollectionAdt>::Element>;
    ///
    fn stack_push(&mut self, element: <Self as CollectionAdt>::Element) -> Result<()>;
}

/// A stack, backed by a [`Array`].
pub struct Stack<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
}

/// A [`Stack`] stored in the stack.
pub type DirectStack<T, const CAP: usize> = Stack<T, (), CAP>;

/// A [`Stack`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedStack<T, const CAP: usize> = Stack<T, Boxed, CAP>;

/* iterators */

/// A stack iterator.
pub struct StackIter<'s, T, S: Storage, const CAP: usize> {
    stack: &'s Stack<T, S, CAP>,
    idx: usize,
}

impl<'s, T, S: Storage, const CAP: usize> Iterator for StackIter<'s, T, S, CAP> {
    type Item = &'s T;
    /// Iterates over shared references.
    ///
    /// # Example
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<i32, 4>::from([1, 2]);
    ///
    /// let mut si = s.iter();
    /// assert_eq![Some(&1), si.next()];
    /// assert_eq![Some(&2), si.next()];
    /// assert_eq![None, si.next()];
    ///
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.idx == self.stack.len() {
            None
        } else {
            Some(&self.stack.array[self.idx])
        };
        self.idx += 1;
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.stack.len()))
    }
}

impl<'s, T, S: Storage, const CAP: usize> ExactSizeIterator for StackIter<'s, T, S, CAP> {}
