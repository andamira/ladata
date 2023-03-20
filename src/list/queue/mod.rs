// ladata::list::queue
//
//! Queues are linear lists for which addittions are made at one end,
//! and removals are made at the opposite end.
//

use crate::{error::LadataResult as Result, list::Array, mem::Storage, misc::CollectionAdt};

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

mod impls;
mod methods;

/// An abstract Queue.
pub trait QueueAdt: CollectionAdt {
    fn queue_dequeue(&mut self) -> Result<<Self as CollectionAdt>::Element>;
    fn queue_enqueue(&mut self, element: <Self as CollectionAdt>::Element) -> Result<()>;
}

/// A queue, backed by an [`Array`].
pub struct Queue<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
    pub(crate) front: usize,
    pub(crate) back: usize,
}

// /// A queue, backed by a [`Vec`].
// pub struct Queue<T> {
//     vec: Vec<T>,
//     front: usize,
//     back: usize,
// }

/// A [`Queue`] stored in the stack.
pub type DirectQueue<T, const CAP: usize> = Queue<T, (), CAP>;

/// A [`Queue`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub type BoxedQueue<T, const CAP: usize> = Queue<T, Boxed, CAP>;

/* iterators */

/// A queue iterator.
pub struct QueueIter<'s, T, S: Storage, const CAP: usize> {
    queue: &'s Queue<T, S, CAP>,
    idx: usize,
}

impl<'s, T, S: Storage, const CAP: usize> Iterator for QueueIter<'s, T, S, CAP> {
    type Item = &'s T;
    /// Iterates over shared references.
    ///
    /// # Example
    /// ```
    /// use ladata::all::DirectQueue;
    ///
    /// let mut q = DirectQueue::<i32, 4>::from([1, 2]);
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
