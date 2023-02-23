// ladata::list::deque::std_impls
//
//!
//

use core::fmt;

use super::{ArrayDeque, CoreArray, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize> Clone for ArrayDeque<T, S, CAP>
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
impl<T: Copy, S: Storage, const CAP: usize> Copy for ArrayDeque<T, S, CAP> where
    S::Stored<[T; CAP]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize> fmt::Debug for ArrayDeque<T, S, CAP>
where
    S::Stored<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![DirectDeque]);
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
impl<T: PartialEq, S: Storage, const CAP: usize> PartialEq for ArrayDeque<T, S, CAP>
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
impl<T: Eq, S: Storage, const CAP: usize> Eq for ArrayDeque<T, S, CAP> where S::Stored<[T; CAP]>: Eq {}

// TODO: WIP
// T:PartialOrd
// use core::cmp::Ordering;
// impl<T: PartialOrd, S: Storage, const CAP: usize> PartialEq for ArrayDeque<T, S, CAP>
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
// impl<T: Ord, S: Storage, const CAP: usize> Ord for ArrayDeque<T, S, CAP> where S::Stored<[T; CAP]>: Ord {}

// S:() + T:Default
impl<T: Default, const CAP: usize> Default for ArrayDeque<T, (), CAP> {
    /// Returns an empty queue, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self {
            array: CoreArray::default(),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "std")]
impl<T: Default, const CAP: usize> Default for ArrayDeque<T, Boxed, CAP> {
    /// Returns an empty queue, allocated in the heap,
    /// using the default value to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BoxedDeque;
    ///
    /// let mut s = BoxedDeque::<i32, 100>::default();
    /// ```
    fn default() -> Self {
        Self {
            array: CoreArray::default(),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

impl<T: Default, I, const CAP: usize> From<I> for ArrayDeque<T, (), CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a queue filled with an iterator, in the stack.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::DirectDeque;
    ///
    /// let s: DirectDeque<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> ArrayDeque<T, (), CAP> {
        let mut s = ArrayDeque::<T, (), CAP>::default();
        let _ = s.extend_back(iterator);
        s
    }
}

#[cfg(feature = "std")]
impl<T: Default, I, const CAP: usize> From<I> for ArrayDeque<T, Boxed, CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a queue filled with an iterator, in the heap.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BoxedDeque;
    ///
    /// let s: BoxedDeque<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> ArrayDeque<T, Boxed, CAP> {
        let mut s = ArrayDeque::<T, Boxed, CAP>::default();
        let _ = s.extend_back(iterator);
        s
    }
}
