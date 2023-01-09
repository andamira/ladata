// ladata::list::deque
//
//! Double-ended queues are linear lists for which any accesses are made from
//! either end.
//

use crate::mem::{Array, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// A double-ended queue, backed by an [`Array`].
///
/// It has the [`Queue`] and [`Stack`] methods implemented for both the front
/// and the back sides.
///
/// [`Queue`]: crate::all::Queue
/// [`Stack`]: crate::all::Stack
pub struct ArrayDeque<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
    pub(crate) front: usize,
    pub(crate) back: usize,
}

/// An [`ArrayDeque`] stored in the stack.
pub type Deque<T, const CAP: usize> = ArrayDeque<T, (), CAP>;

/// An [`ArrayDeque`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedDeque<T, const CAP: usize> = ArrayDeque<T, Boxed, CAP>;

/* iterators */

pub struct DequeIter<'s, T, S: Storage, const CAP: usize> {
    deque: &'s ArrayDeque<T, S, CAP>,
    idx: usize,
}

impl<'s, T, S: Storage, const CAP: usize> Iterator for DequeIter<'s, T, S, CAP> {
    type Item = &'s T;
    /// Iterates over shared references.
    ///
    /// # Example
    /// ```
    /// use ladata::all::Deque;
    ///
    /// let mut dq = Deque::<i32, 4>::from([1, 2]);
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
    /// use ladata::all::Deque;
    ///
    /// let mut dq = Deque::<i32, 4>::from([1, 2]);
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
