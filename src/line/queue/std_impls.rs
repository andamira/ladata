// ladata::line::queue::std_impls
//
//!
//

use core::fmt;

#[cfg(not(feature = "no_unsafe"))]
use core::mem::{self, MaybeUninit};

use super::{Queue, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize> Clone for Queue<T, S, CAP>
where
    S::Container<[T; CAP]>: Clone,
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
impl<T: Copy, S: Storage, const CAP: usize> Copy for Queue<T, S, CAP> where
    S::Container<[T; CAP]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize> fmt::Debug for Queue<T, S, CAP>
where
    S::Container<[T; CAP]>: fmt::Debug,
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
    S::Container<[T; CAP]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
            && self.len == other.len
            && self.front == other.front
            && self.back == other.back
    }
}
// T:Eq
impl<T: Eq, S: Storage, const CAP: usize> Eq for Queue<T, S, CAP> where S::Container<[T; CAP]>: Eq {}

// S:() + T:Default
impl<T: Default, const CAP: usize> Default for Queue<T, (), CAP> {
    /// Returns an empty queue, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        #[cfg(not(feature = "no_unsafe"))]
        let data = {
            let mut arr: [MaybeUninit<T>; CAP] = unsafe { MaybeUninit::uninit().assume_init() };
            for i in &mut arr[..] {
                let _ = i.write(T::default());
            }
            unsafe { mem::transmute_copy::<_, [T; CAP]>(&arr) }
        };

        #[cfg(feature = "no_unsafe")]
        let data = core::array::from_fn(|_| T::default());

        Self {
            array: data.into(),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "std")]
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
        #[cfg(feature = "no_unsafe")]
        let data = {
            let mut v = Vec::<T>::with_capacity(CAP);

            for _ in 0..CAP {
                v.push(T::default());
            }

            let Ok(array) = v.into_boxed_slice().try_into() else {
                panic!("Can't turn the boxed slice into a boxed array");
            };
            array
        };

        #[cfg(not(feature = "no_unsafe"))]
        let data = {
            let mut v = Vec::<T>::with_capacity(CAP);

            for _ in 0..CAP {
                v.push(T::default());
            }

            let slice = v.into_boxed_slice();
            let raw_slice = Box::into_raw(slice);
            // SAFETY: pointer comes from using `into_raw`, and capacity is right.
            unsafe { Box::from_raw(raw_slice as *mut [T; CAP]) }
        };

        Self {
            array: data,
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

impl<T: Default, I, const CAP: usize> From<I> for Queue<T, (), CAP>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a queue filled with an iterator, in the stack.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::RawQueue;
    ///
    /// let s: RawQueue<_, 3> = [1, 2, 3].into();
    /// ```
    fn from(iterator: I) -> Queue<T, (), CAP> {
        let mut s = Queue::<T, (), CAP>::default();
        let _ = s.extend_back(iterator);
        s
    }
}

#[cfg(feature = "std")]
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
        let _ = s.extend_back(iterator);
        s
    }
}
