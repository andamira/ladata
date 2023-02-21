// ladata::list::queue::methods
//
//! ArrayQueues.
//

#[cfg(not(feature = "safe"))]
use core::{
    mem::{self, MaybeUninit},
    ptr,
};

use super::{Array, ArrayQueue, QueueIter};

use crate::{
    error::{LadataError as Error, LadataResult as Result},
    mem::Storage,
};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// `S:() + T:Clone`
impl<T: Clone, const CAP: usize> ArrayQueue<T, (), CAP> {
    /// Returns an empty queue, allocated in the stack,
    /// using `element` to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::ArrayQueue;
    ///
    /// let q = ArrayQueue::<_, (), 16>::new('\0');
    /// ```
    pub fn new(element: T) -> Self {
        Self {
            array: Array::<T, (), CAP>::with(element),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

// `S:Boxed + T:Clone`
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T: Clone, const CAP: usize> ArrayQueue<T, Boxed, CAP> {
    /// Returns an empty queue, allocated in the stack,
    /// using `element` to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::BoxedQueue;
    ///
    /// let q = BoxedQueue::<_, 16>::new('\0');
    /// ```
    pub fn new(element: T) -> Self {
        Self {
            array: Array::<T, Boxed, CAP>::with(element),
            front: 0,
            back: 0,
            len: 0,
        }
    }
}

// ``
impl<T, S: Storage, const CAP: usize> ArrayQueue<T, S, CAP> {
    // Returns the `nth` element's index counting from the front.
    #[inline(always)]
    pub(super) const fn idx_front(&self, nth: usize) -> usize {
        (self.front + nth) % CAP
    }

    /// Moves an array into a [`full`][Self::is_full] queue.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::ArrayQueue;
    ///
    /// let q = ArrayQueue::<_, (), 3>::from_array([1, 2, 3]);
    /// ```
    // TODO: IMPROVE(like stack)
    pub fn from_array(arr: [T; CAP]) -> ArrayQueue<T, S, CAP> {
        Self {
            array: Array::new(arr),
            front: 0,
            back: 0,
            len: CAP,
        }
    }

    /// Returns the number of queued elements.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Checks `true` if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::Queue;
    ///
    /// let q = Queue::<i32, 8>::default();
    /// assert![q.is_empty()];
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the stack is full.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::Queue;
    ///
    /// let q = Queue::<_, 3>::from([1, 2, 3]);
    /// assert![q.is_full()];
    /// ```
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len() == CAP
    }

    /// Returns the queue's total capacity.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::Queue;
    ///
    /// let q = Queue::<i32, 3>::default();
    /// assert_eq![3, q.capacity()];
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// Returns the queue's remaining capacity.
    /// ```
    /// use ladata::list::Queue;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut q = Queue::<i32, 3>::default();
    /// assert_eq![3, q.remaining_capacity()];
    /// q.push(1)?;
    /// assert_eq![2, q.remaining_capacity()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub const fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }

    /* iter */

    pub fn iter(&self) -> QueueIter<'_, T, S, CAP> {
        QueueIter {
            queue: self,
            idx: 0,
        }
    }

    /* extend */

    /// Extends the queue from an iterator.
    ///
    /// `( a … b -- a … b c d e f )` for [c d e f]
    ///
    /// # Errors
    /// Errors if the queue becomes full before the iterator finishes.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::Queue;
    ///
    /// let mut q = Queue::<_, 6>::from([1, 2, 3]);
    /// q.extend([4, 5, 6, 7, 8]);
    /// assert_eq![q.to_array(), Some([1, 2, 3, 4, 5, 6])];
    /// ```
    pub fn extend<I>(&mut self, iterator: I) -> Result<()>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iterator.into_iter();
        while !self.is_full() {
            if let Some(e) = iter.next() {
                self.push_unchecked(e);
            } else {
                return Ok(());
            }
        }
        Err(Error::NotEnoughSpace(None))
    }

    /* push */

    /// Pushes a new element to the back of the queue.
    ///
    /// `( a … b -- a … b c )`
    ///
    /// # Errors
    /// Errors if the queue is full.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let mut q = Queue::<u8, 3>::default();
    /// q.push(1)?;
    /// q.push(2)?;
    /// q.push(3)?;
    /// assert_eq![q.to_array(), Some([1, 2, 3])];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn push(&mut self, element: T) -> Result<()> {
        if self.is_full() {
            Err(Error::NotEnoughSpace(Some(1)))
        } else {
            self.array[self.back] = element;
            self.back = (self.back + 1) % CAP;
            self.len += 1;
            Ok(())
        }
    }

    /// Alias of [`push`][Self::push].
    #[inline(always)]
    pub fn enqueue(&mut self, element: T) -> Result<()> {
        self.push(element)
    }

    /// Unchecked version of [`push`][Self::push].
    ///
    /// # Panics
    /// Panics if the queue is full.
    #[inline]
    pub fn push_unchecked(&mut self, element: T) {
        self.array[self.back] = element;
        self.back = (self.back + 1) % CAP;
        self.len += 1;
    }

    /* peek */

    /// Returns a shared reference to the front element.
    ///
    /// # Errors
    /// Errors if the queue is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let q = Queue::<_, 8>::from([1, 2, 3]);
    /// assert_eq![&1, q.peek()?];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn peek(&self) -> Result<&T> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            let fi = self.idx_front(0);
            Ok(&self.array[fi])
        }
    }

    /// Returns an exclusive reference to the front element.
    ///
    /// # Errors
    /// Errors if the queue is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut q = Queue::<_, 8>::from([1, 2, 3]);
    /// assert_eq![&mut 1, q.peek_mut()?];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn peek_mut(&mut self) -> Result<&mut T> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            let fi = self.idx_front(0);
            Ok(&mut self.array[fi])
        }
    }

    /// Returns a shared reference to the `nth` front element.
    ///
    /// # Errors
    /// Errors if the queue doesn't have at least `nth` elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let q = Queue::<_, 8>::from([1, 2, 3, 4]);
    /// assert_eq![&3, q.peek_nth(2)?];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn peek_nth(&self, nth: usize) -> Result<&T> {
        if self.len() <= nth {
            Err(Error::NotEnoughElements(nth))
        } else {
            let bi = self.idx_front(nth);
            Ok(&self.array[bi])
        }
    }

    /// Returns an exclusive reference to the `nth` front element.
    ///
    /// # Errors
    /// Errors if the queue doesn't have at least `nth` elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut q = Queue::<_, 8>::from([1, 2, 3, 4]);
    /// assert_eq![&mut 3, q.peek_nth_mut(2)?];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn peek_nth_mut(&mut self, nth: usize) -> Result<&mut T> {
        if self.len() <= nth {
            Err(Error::NotEnoughElements(nth))
        } else {
            let bi = self.idx_front(nth);
            Ok(&mut self.array[bi])
        }
    }

    /* clear */

    /// Clears the queue.
    ///
    /// `( a … b -- )`
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    ///
    /// let mut q = Queue::<_, 8>::from([1, 2, 3, 4]);
    /// q.clear();
    /// assert![q.is_empty()];
    /// ```
    pub fn clear(&mut self) {
        self.front = 0;
        self.back = 0;
        self.len = 0;
    }

    /* pop */

    /// Pops the front element.
    ///
    /// `( a … b -- … b )`
    ///
    /// # Errors
    /// Errors if the queue is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut q = Queue::<_, 8>::from([1, 2, 3]);
    /// assert_eq![1, q.pop()?];
    /// assert_eq![2, q.pop()?];
    /// assert_eq![3, q.pop()?];
    /// assert![q.is_empty()];
    /// # Ok(()) }
    /// ```
    #[inline]
    #[cfg(not(feature = "safe"))]
    pub fn pop(&mut self) -> Result<T> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            // SAFETY: we're not gonna access the value, but move it out
            // MOTIVATION: to not depend on T: Clone
            let e = unsafe { ptr::read((self.array.get_unchecked(self.front)) as *const T) };

            self.front = (self.front + 1) % CAP;
            self.len -= 1;
            Ok(e)
        }
    }
    /// Alias of [`pop`][Self::pop].
    #[inline(always)]
    #[cfg(not(feature = "safe"))]
    pub fn dequeue(&mut self) -> Result<T> {
        self.pop()
    }
}

// `T:Clone`
impl<T: Clone, S: Storage, const CAP: usize> ArrayQueue<T, S, CAP> {
    /// Pops the front element.
    ///
    /// `( a … b -- … b )`
    ///
    /// # Errors
    /// Errors if the queue is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut q = Queue::<_, 8>::from([1, 2, 3]);
    /// assert_eq![1, q.pop()?];
    /// assert_eq![2, q.pop()?];
    /// assert_eq![3, q.pop()?];
    /// assert![q.is_empty()];
    /// # Ok(()) }
    /// ```
    #[inline]
    #[cfg(feature = "safe")]
    // safe-only version that depends on T: Clone
    pub fn pop(&mut self) -> Result<T> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            let e = self.array[self.front].clone();
            self.front = (self.front + 1) % CAP;
            self.len -= 1;
            Ok(e)
        }
    }
    /// Alias of [`pop`][Self::pop].
    #[inline(always)]
    #[cfg(feature = "safe")]
    pub fn dequeue(&mut self) -> Result<T> {
        self.pop()
    }

    /* to_vec, to_array */

    /// Returns the queued elements as a vector.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::Queue;
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let mut q = Queue::<_, 5>::from([1, 2, 3, 4, 5]);
    /// assert_eq![q.to_vec(), vec![1, 2, 3, 4, 5]];
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        if !self.is_empty() {
            let mut index = self.front;

            // makes sure a full queue is not rejected
            vec.push(self.array[index].clone());
            index = (index + 1) % CAP;

            while index != self.back {
                vec.push(self.array[index].clone());
                index = (index + 1) % CAP;
            }
        }
        vec
    }

    /// Returns some `LEN` queued elements as an array, or `None` if the queue
    /// is empty, or there are not at least `LEN` elements.
    ///
    /// This is a `no_std` alternative method to [`to_vec`][Self::to_vec].
    ///
    /// # Panics
    /// Panics if the new LEN sized array can't be allocated.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::Queue;
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let mut q = Queue::<_, 5>::from([1, 2, 3, 4, 5]);
    /// assert_eq![q.to_array::<5>(), Some([1, 2, 3, 4, 5])];
    /// # Ok(()) }
    /// ```
    //
    // TODO IMPROVE
    pub fn to_array<const LEN: usize>(&self) -> Option<[T; LEN]> {
        // MAYBE return not option
        // TODO: improve from_iter
        // Some(Array::<T, S, LEN>::new())

        if self.is_empty() || LEN > self.len() || LEN == 0 {
            None
        } else {
            #[cfg(not(feature = "safe"))]
            let arr = {
                let mut unarr: [MaybeUninit<T>; LEN] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for (n, i) in unarr.iter_mut().enumerate().take(LEN) {
                    let index = (self.front + n) % CAP;
                    let _ = i.write(self.array[index].clone());
                }

                // TEMP:FIX: can't use transmute for now:
                // - https://github.com/rust-lang/rust/issues/62875
                // - https://github.com/rust-lang/rust/issues/61956
                // mem::transmute::<_, [T; LEN]>(&arr)
                unsafe { mem::transmute_copy::<_, [T; LEN]>(&unarr) }
            };

            #[cfg(feature = "safe")]
            let arr = core::array::from_fn(|n| {
                let index = (self.front + n) % CAP;
                self.array[index].clone()
            });

            Some(arr)
        }
    }
}

// `T: PartialEq`
impl<T: PartialEq, S: Storage, const CAP: usize> ArrayQueue<T, S, CAP> {
    /// Returns true if the queue contains `element`.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Queue;
    ///
    /// let q = Queue::<_, 6>::from([5, 78, 42, 33, 9]);
    ///
    /// assert![q.contains(&9)];
    /// assert![!q.contains(&8)];
    /// ```
    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|n| n == element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test the private idx_* functions
    #[test]
    fn idx() {
        let q = ArrayQueue::<_, (), 5>::from([1, 2, 3]);

        // counting from the front:
        assert_eq![0, q.idx_front(0)];
        assert_eq![1, q.idx_front(1)];
        assert_eq![2, q.idx_front(2)];
        // ignores current len()
        assert_eq![3, q.idx_front(3)];
        assert_eq![4, q.idx_front(4)];
        // loops over CAP
        assert_eq![0, q.idx_front(5)];
    }
}
