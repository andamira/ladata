// ladata::list::queue::std_impls
//
//!
//

use core::fmt;

use super::{Queue, Storage};
use crate::list::{Array, Deque, Stack};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize> Clone for Queue<T, S, CAP>
where
    S::Stored<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
            front: self.front,
            back: self.back,
            len: self.len,
        }
    }
}

// T:Copy
impl<T: Copy, S: Storage, const CAP: usize> Copy for Queue<T, S, CAP> where S::Stored<[T; CAP]>: Copy
{}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize> fmt::Debug for Queue<T, S, CAP>
where
    S::Stored<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![Queue]);
        debug
            .field("CAP", &CAP)
            .field("len", &self.len)
            .field("front", &self.front)
            .field("back", &self.back);

        if CAP <= 6 {
            debug.field("nodes", &self.array);
        } else {
            // IMPROVE: show first 3 and last 3
            debug.field("array { ... }", &());
        }
        debug.finish()
    }
}

// T:PartialEq
impl<T: PartialEq, S: Storage, const CAP: usize> PartialEq for Queue<T, S, CAP>
where
    S::Stored<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
            && self.len == other.len
            && self.front == other.front
            && self.back == other.back
    }
}
// T:Eq
impl<T: Eq, S: Storage, const CAP: usize> Eq for Queue<T, S, CAP> where S::Stored<[T; CAP]>: Eq {}

// TODO: WIP
// T:PartialOrd
// use core::cmp::Ordering;
// impl<T: PartialOrd, S: Storage, const CAP: usize> PartialEq for Queue<T, S, CAP>
// where
//     S::Stored<[T; CAP]>: PartialOrd,
// {
//     fn partial_cmp(&self, other: &Self) -> Ordering {
//         self.array == other.array
//             && self.len == other.len
//             && self.front == other.front
//             && self.back == other.back
//     }
// }
// // T:Ord
// impl<T: Ord, S: Storage, const CAP: usize> Ord for Queue<T, S, CAP> where S::Stored<[T; CAP]>: Ord {}

// S:() + T:Default
impl<T: Default, const CAP: usize> Default for Queue<T, (), CAP> {
    /// Returns an empty queue, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self {
            array: Array::default(),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T: Default, const CAP: usize> Default for Queue<T, Boxed, CAP> {
    /// Returns an empty queue, allocated in the heap,
    /// using the default value to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BoxedQueue;
    ///
    /// let mut s = BoxedQueue::<i32, 100>::default();
    /// ```
    fn default() -> Self {
        Self {
            array: Array::default(),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

/* From<> */

impl<T: Default, I, const CAP: usize> From<I> for Queue<T, (), CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a queue filled with an iterator, in the stack.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectQueue;
    ///
    /// let s: DirectQueue<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> Queue<T, (), CAP> {
        let mut s = Queue::<T, (), CAP>::default();
        let _ = s.extend(iterator);
        s
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T: Default, I, const CAP: usize> From<I> for Queue<T, Boxed, CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a queue filled with an iterator, in the heap.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BoxedQueue;
    ///
    /// let s: BoxedQueue<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> Queue<T, Boxed, CAP> {
        let mut s = Queue::<T, Boxed, CAP>::default();
        let _ = s.extend(iterator);
        s
    }
}

// TODO: CHECK
impl<T, S: Storage, const CAP: usize> From<Deque<T, S, CAP>> for Queue<T, S, CAP> {
    fn from(deque: Deque<T, S, CAP>) -> Self {
        Queue {
            array: deque.array,
            front: deque.front,
            back: deque.back,
            len: deque.len,
        }
    }
}
// TODO: CHECK
impl<T, S: Storage, const CAP: usize> From<Stack<T, S, CAP>> for Queue<T, S, CAP> {
    fn from(stack: Stack<T, S, CAP>) -> Self {
        Queue {
            array: stack.array,
            len: stack.len,
            front: 0,
            back: stack.len,
        }
    }
}
