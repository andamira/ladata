// ladata::list::deque
//
//! Double-ended queues are linear lists for which any accesses are made from
//! either end.
//

use crate::{
    error::LadataResult as Result,
    list::{Array, QueueAdt},
    mem::Storage,
    misc::CollectionAdt,
};

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// An abstract Deque.
pub trait DequeAdt: CollectionAdt + QueueAdt {
    ///
    fn deque_dequeue_back(&mut self) -> Result<<Self as CollectionAdt>::Element>;
    ///
    fn deque_enqueue_front(&mut self, element: <Self as CollectionAdt>::Element) -> Result<()>;
    //
    ///
    fn deque_dequeue_front(&mut self) -> Result<<Self as CollectionAdt>::Element> {
        self.queue_dequeue()
    }
    ///
    fn deque_enqueue_back(&mut self, element: <Self as CollectionAdt>::Element) -> Result<()> {
        self.queue_enqueue(element)
    }
}

/// A double-ended queue, backed by an [`Array`].
///
/// It has the [`DirectQueue`] and [`DirectStack`] methods implemented for both
/// the front and the back sides.
///
/// [`DirectQueue`]: crate::all::DirectQueue
/// [`DirectStack`]: crate::all::DirectStack
pub struct Deque<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
    pub(crate) front: usize,
    pub(crate) back: usize,
}

/// A [`Deque`] stored in the stack.
pub type DirectDeque<T, const CAP: usize> = Deque<T, (), CAP>;

/// A [`Deque`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedDeque<T, const CAP: usize> = Deque<T, Boxed, CAP>;

/* iterators */

/// A deque iterator.
pub struct DequeIter<'s, T, S: Storage, const CAP: usize> {
    deque: &'s Deque<T, S, CAP>,
    idx: usize,
}

impl<'s, T, S: Storage, const CAP: usize> Iterator for DequeIter<'s, T, S, CAP> {
    type Item = &'s T;
    /// Iterates over shared references.
    ///
    /// # Example
    /// ```
    /// use ladata::all::DirectDeque;
    ///
    /// let mut dq = DirectDeque::<i32, 4>::from([1, 2]);
    /// dq.pop_front();
    /// dq.push_back(3);
    /// dq.pop_front();
    /// dq.push_back(4);
    ///
    /// let mut dqi = dq.iter();
    /// assert_eq![Some(&3), dqi.next()];
    /// assert_eq![Some(&4), dqi.next()];
    /// assert_eq![None, dqi.next()];
    ///
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.idx == self.deque.len() {
            None
        } else {
            Some(&self.deque.array[self.deque.idx_front(self.idx)])
        };
        self.idx += 1;
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.deque.len(), Some(self.deque.len()))
    }
}

impl<'s, T, S: Storage, const CAP: usize> ExactSizeIterator for DequeIter<'s, T, S, CAP> {}

impl<'s, T, S: Storage, const CAP: usize> DoubleEndedIterator for DequeIter<'s, T, S, CAP> {
    /// Iterates over shared references.
    ///
    /// # Example
    /// ```
    /// use ladata::all::DirectDeque;
    ///
    /// let mut dq = DirectDeque::<i32, 4>::from([1, 2]);
    /// dq.pop_front();
    /// dq.push_back(3);
    /// dq.pop_front();
    /// dq.push_back(4);
    ///
    /// let mut dqi = dq.iter();
    /// assert_eq![Some(&3), dqi.next()];
    /// assert_eq![Some(&4), dqi.next()];
    /// assert_eq![None, dqi.next()];
    ///
    /// let mut dqi = dq.iter();
    /// assert_eq![Some(&4), dqi.next_back()];
    /// assert_eq![Some(&3), dqi.next_back()];
    /// assert_eq![None, dqi.next_back()];
    /// ```
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = if self.idx == self.deque.len() {
            None
        } else {
            Some(&self.deque.array[self.deque.idx_back(self.idx)])
        };
        self.idx += 1;
        item
    }
}
