// ladata::list::queue
//
//! Queues are linear lists for which addittions are made at one end,
//! and removals are made at the opposite end.
//

use crate::mem::{CoreArray, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// A queue, backed by a [`CoreArray`].
pub struct ArrayQueue<T, S: Storage, const CAP: usize> {
    pub(crate) array: CoreArray<T, S, CAP>,
    pub(crate) len: usize,
    pub(crate) front: usize,
    pub(crate) back: usize,
}

// /// A queue, backed by a [`Vec`].
// pub struct ArrayQueue<T> {
//     vec: Vec<T>,
//     front: usize,
//     back: usize,
// }

/// An [`ArrayQueue`] stored in the stack.
pub type Queue<T, const CAP: usize> = ArrayQueue<T, (), CAP>;

/// An [`ArrayQueue`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedQueue<T, const CAP: usize> = ArrayQueue<T, Boxed, CAP>;

/* iterators */

pub struct QueueIter<'s, T, S: Storage, const CAP: usize> {
    queue: &'s ArrayQueue<T, S, CAP>,
    idx: usize,
}

impl<'s, T, S: Storage, const CAP: usize> Iterator for QueueIter<'s, T, S, CAP> {
    type Item = &'s T;
    /// Iterates over shared references.
    ///
    /// # Example
    /// ```
    /// use ladata::all::Queue;
    ///
    /// let mut q = Queue::<i32, 4>::from([1, 2]);
    /// q.pop();
    /// q.push(3);
    /// q.pop();
    /// q.push(4);
    ///
    /// let mut qi = q.iter();
    /// assert_eq![Some(&3), qi.next()];
    /// assert_eq![Some(&4), qi.next()];
    /// assert_eq![None, qi.next()];
    ///
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.idx == self.queue.len() {
            None
        } else {
            Some(&self.queue.array[self.queue.idx_front(self.idx)])
        };
        self.idx += 1;
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.queue.len(), Some(self.queue.len()))
    }
}

impl<'s, T, S: Storage, const CAP: usize> ExactSizeIterator for QueueIter<'s, T, S, CAP> {}
