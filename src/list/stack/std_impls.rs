// ladata::list::stack::std_impls
//
//!
//

use core::fmt;

use super::{Array, ArrayStack, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize> Clone for ArrayStack<T, S, CAP>
where
    S::Container<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
            len: self.len,
        }
    }
}

// T:Copy
impl<T: Copy, S: Storage, const CAP: usize> Copy for ArrayStack<T, S, CAP> where
    S::Container<[T; CAP]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize> fmt::Debug for ArrayStack<T, S, CAP>
where
    S::Container<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![ArrayStack]);
        debug.field("CAP", &CAP).field("len", &self.len);

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
impl<T: PartialEq, S: Storage, const CAP: usize> PartialEq for ArrayStack<T, S, CAP>
where
    S::Container<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.len == other.len
    }
}
// T:Eq
impl<T: Eq, S: Storage, const CAP: usize> Eq for ArrayStack<T, S, CAP> where
    S::Container<[T; CAP]>: Eq
{
}

// S:() + T:Default
impl<T: Default, const CAP: usize> Default for ArrayStack<T, (), CAP> {
    /// Returns an empty stack, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        Self {
            array: Array::default(),
            len: 0,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "std")]
impl<T: Default, const CAP: usize> Default for ArrayStack<T, Boxed, CAP> {
    /// Returns an empty stack, allocated in the heap,
    /// using the default value to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BoxedStack;
    ///
    /// let mut s = BoxedStack::<i32, 100>::default();
    /// ```
    fn default() -> Self {
        Self {
            array: Array::default(),
            len: 0,
        }
    }
}

impl<T: Default, I, const CAP: usize> From<I> for ArrayStack<T, (), CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a stack filled with an iterator, in the stack.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Stack;
    ///
    /// let s: Stack<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> ArrayStack<T, (), CAP> {
        let mut s = ArrayStack::<T, (), CAP>::default();
        let _ = s.extend(iterator);
        s
    }
}

#[cfg(feature = "std")]
impl<T: Default, I, const CAP: usize> From<I> for ArrayStack<T, Boxed, CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a stack filled with an iterator, in the heap.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BoxedStack;
    ///
    /// let s: BoxedStack<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> ArrayStack<T, Boxed, CAP> {
        let mut s = ArrayStack::<T, Boxed, CAP>::default();
        let _ = s.extend(iterator);
        s
    }
}

// TODO
// impl<T, S: Storage, const CAP: usize> From<ArrayDeque<T, S, CAP>> for ArrayStack<T, S, CAP> {
//     fn from(deque: ArrayDeque<T, S, CAP>) -> Self {
//         ArrayQueue {
//             array: deque.array,
//             front: deque.front,
//             back: deque.back,
//             len: deque.len,
//         }
//     }
// }
// impl<T, S: Storage, const CAP: usize> From<ArrayStack<T, S, CAP>> for ArrayQueue<T, S, CAP> {
//     fn from(stack: ArrayStack<T, S, CAP>) -> Self {
//         ArrayQueue {
//             array: deque.array,
//             front: deque.front,
//             back: deque.back,
//             len: deque.len,
//         }
//     }
// }
