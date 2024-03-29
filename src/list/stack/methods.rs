// ladata::list::stack::methods
//
//! Stacks.
//

#[cfg(feature = "unsafe_pop")]
use core::ptr;

use super::{Stack, StackIter};
use crate::{
    error::{LadataError as Error, LadataResult as Result},
    list::Array,
    mem::Storage,
};

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

// `S:() + T:Clone`
impl<T: Clone, const CAP: usize> Stack<T, (), CAP> {
    /// Returns an empty stack, allocated in the stack,
    /// using `element` to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<_, 16>::new(0);
    /// ```
    pub fn new(element: T) -> Self {
        Self {
            array: Array::<T, (), CAP>::with(element),
            len: 0,
        }
    }
}

// `S:Boxed + T:Clone`
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
impl<T: Clone, const CAP: usize> Stack<T, Boxed, CAP> {
    /// Returns an empty stack, allocated in the heap,
    /// using `element` to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::BoxedStack;
    ///
    /// let mut s = BoxedStack::<_, 100>::new(0);
    /// ```
    pub fn new(element: T) -> Self {
        Self {
            array: Array::<T, Boxed, CAP>::with(element),
            len: 0,
        }
    }
}

// ``
impl<T, S: Storage, const CAP: usize> Stack<T, S, CAP> {
    /// Returns the number of stacked elements.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Checks `true` if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<i32, 8>::default();
    /// assert![s.is_empty()];
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the stack is full.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<_, 3>::from([1, 2, 3]);
    /// assert![s.is_full()];
    /// ```
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len() == CAP
    }

    /// Returns the stack's total capacity.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<i32, 3>::default();
    /// assert_eq![3, s.capacity()];
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// Returns the stack's remaining capacity.
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<i32, 3>::default();
    /// assert_eq![3, s.remaining_capacity()];
    /// s.push(1)?;
    /// assert_eq![2, s.remaining_capacity()];
    ///
    /// # Ok(()) }
    /// ```
    #[inline]
    pub const fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }

    //

    /// Returns the stack as a shared slice.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<_, 3>::from([1, 2, 3]);
    /// assert_eq![s.as_slice(), &[1, 2, 3]];
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.array[..self.len]
    }

    /// Returns the stack as an exclusive slice.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 3>::from([1, 2, 3]);
    /// assert_eq![s.as_mut_slice(), &mut [1, 2, 3]];
    /// ```
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.array[..self.len]
    }

    /// Extends the stack from an iterator.
    ///
    /// # Errors
    /// Errors if the stack becomes full before the iterator finishes.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 5>::default();
    /// s.extend([1, 2, 3]);
    /// assert_eq![s.as_slice(), &[1, 2, 3]];
    ///
    /// s.extend([4, 5, 6, 7, 8]);
    /// assert_eq![s.as_slice(), &[1, 2, 3, 4, 5]];
    /// ```
    pub fn extend<I>(&mut self, iterator: I) -> Result<()>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iterator.into_iter();
        while !self.is_full() {
            if let Some(e) = iter.next() {
                let _ = self.push(e);
            } else {
                return Ok(());
            }
        }
        Err(Error::NotEnoughSpace(None))
    }

    //

    /// Peeks the top stack element.
    ///
    /// `( 1 -- 1 )`
    ///
    /// Returns a shared reference to the top stack element.
    ///
    /// # Errors
    /// Errors if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<_, 2>::from([1, 2]);
    /// assert_eq![s.peek(), Ok(&2)];
    /// ```
    #[inline]
    pub fn peek(&self) -> Result<&T> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            let e = &self.array[self.len - 1];
            Ok(e)
        }
    }

    /// Mutably peeks the top stack element.
    ///
    /// `( 1 -- 1 )`
    ///
    /// Returns an exclusive reference to the top stack element.
    ///
    /// # Errors
    /// Errors if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 2>::from([1, 2]);
    /// assert_eq![s.peek_mut(), Ok(&mut 2)];
    /// ```
    #[inline]
    pub fn peek_mut(&mut self) -> Result<&mut T> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            let e = &mut self.array[self.len - 1];
            Ok(e)
        }
    }

    /// Peeks the `nth` element from the top of the stack.
    ///
    /// `( 1 -- 1 )`
    ///
    /// Returns a shared reference to the `nth` element,
    /// starting from 0 for the top, 1 for the next-of-stack, etc.
    ///
    /// # Errors
    /// Errors if the stack has not enough elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<_, 5>::from([1, 2, 3, 4, 5]);
    /// assert_eq![s.peek_nth(0), Ok(&5)];
    /// assert_eq![s.peek_nth(4), Ok(&1)];
    /// ```
    #[inline]
    pub fn peek_nth(&self, nth: usize) -> Result<&T> {
        if self.len() <= nth {
            Err(Error::NotEnoughElements(nth))
        } else {
            let e = &self.array[self.len - 1 - nth];
            Ok(e)
        }
    }

    /// Mutably peeks the `nth` element from the top of the stack.
    ///
    /// `( 1 -- 1 )`
    ///
    /// Returns an exclusive reference to the `nth` element,
    /// starting from 0 for the top, 1 for the next-of-stack, etc.
    ///
    /// # Errors
    /// Errors if the stack has not enough elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 5>::from([1, 2, 3, 4, 5]);
    /// assert_eq![s.peek_nth_mut(0), Ok(&mut 5)];
    /// assert_eq![s.peek_nth_mut(4), Ok(&mut 1)];
    /// ```
    #[inline]
    pub fn peek_nth_mut(&mut self, nth: usize) -> Result<&mut T> {
        if self.len() <= nth {
            Err(Error::NotEnoughElements(nth))
        } else {
            let e = &mut self.array[self.len - 1 - nth];
            Ok(e)
        }
    }

    /// Swaps the top two stack elements.
    ///
    /// `( 1 2 -- 2 1 )`
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least 2 elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 2>::from([1, 2]);
    /// s.swap();
    /// assert_eq![s.as_slice(), &[2, 1]];
    /// ```
    #[inline]
    pub fn swap(&mut self) -> Result<()> {
        if self.len() < 2 {
            Err(Error::NotEnoughElements(2))
        } else {
            self.array.swap(self.len - 2, self.len - 1);
            Ok(())
        }
    }

    /// Swaps the top two pair stack elements.
    ///
    /// `( 1 2 3 4 -- 3 4 1 2 )`
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least 4 elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 4>::from([1, 2, 3, 4]);
    /// s.swap2();
    /// assert_eq![s.as_slice(), &[3, 4, 1, 2]];
    /// ```
    #[inline]
    pub fn swap2(&mut self) -> Result<()> {
        if self.len() < 4 {
            Err(Error::NotEnoughElements(4))
        } else {
            self.array.swap(self.len - 4, self.len - 2);
            self.array.swap(self.len - 3, self.len - 1);
            Ok(())
        }
    }

    /// Clears the stack.
    ///
    /// `( 1 2 3 -- )`
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 8>::from([1, 2, 3, 4]);
    /// s.clear();
    /// assert![s.is_empty()];
    /// ```
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Drops the top stack element.
    ///
    /// `( 1 2 -- 1 )`
    ///
    /// # Errors
    /// Errors if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 2>::from([1, 2]);
    /// s.drop();
    /// assert_eq![s.as_slice(), &[1]];
    /// ```
    #[inline]
    pub fn drop(&mut self) -> Result<()> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            self.len -= 1;
            Ok(())
        }
    }

    /// Drops the top `n` stack elements.
    ///
    /// `( 1 2 3 4 -- 1 )` for `n == 3`
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least `nth` elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 4>::from([1, 2, 3, 4]);
    /// s.drop_n(3);
    /// assert_eq![s.as_slice(), &[1]];
    /// ```
    #[inline]
    pub fn drop_n(&mut self, nth: usize) -> Result<()> {
        if self.len() < nth {
            Err(Error::NotEnoughElements(nth))
        } else {
            self.len -= nth;
            Ok(())
        }
    }

    /// Drops the next of stack element.
    ///
    /// `( 1 2 -- 2 )`
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least 2 elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 2>::from([1, 2]);
    /// s.nip();
    /// assert_eq![s.as_slice(), &[2]];
    /// ```
    #[inline]
    pub fn nip(&mut self) -> Result<()> {
        if self.len() < 2 {
            Err(Error::NotEnoughElements(2))
        } else {
            self.array.swap(self.len - 2, self.len - 1);
            self.len -= 1;
            Ok(())
        }
    }

    /// Drops the pair of next stack elements.
    ///
    /// `( 1 2 3 4 -- 3 4 )`
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least 4 elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 8>::from([1, 2, 3, 4]);
    /// s.nip2();
    /// assert_eq![s.as_slice(), &[3, 4]];
    /// ```
    #[inline]
    pub fn nip2(&mut self) -> Result<()> {
        if self.len() < 4 {
            Err(Error::NotEnoughElements(4))
        } else {
            self.array.swap(self.len - 4, self.len - 2);
            self.array.swap(self.len - 3, self.len - 1);
            self.len -= 2;
            Ok(())
        }
    }

    /// Rotates the top three stack elements, clockwise.
    ///
    /// `( 1 2 3 -- 2 3 1 ) `
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least 3 elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<_, 3>::from(['a', 'b', 'c']);
    /// s.rot()?;
    /// assert_eq![s.as_slice(), &['b', 'c', 'a']];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn rot(&mut self) -> Result<()> {
        if self.len() < 3 {
            Err(Error::NotEnoughElements(3))
        } else {
            self.array[self.len - 3..self.len].rotate_left(1);
            Ok(())
        }
    }

    /// Rotates the top three stack elements, counter-clockwise.
    ///
    /// `( 1 2 3 -- 3 1 2 ) `
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least 3 elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<_, 3>::from(['a', 'b', 'c']);
    /// s.rot_cc()?;
    /// assert_eq![s.as_slice(), &['c', 'a', 'b']];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn rot_cc(&mut self) -> Result<()> {
        if self.len() < 3 {
            Err(Error::NotEnoughElements(3))
        } else {
            self.array[self.len - 3..self.len].rotate_right(1);
            Ok(())
        }
    }

    /// Rotates the top six stack elements, clockwise, two times.
    ///
    /// `( 1 2 3 4 5 6 -- 3 4 5 6 1 2 ) `
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least 6 elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<_, 6>::from(['a', 'b', 'c', 'd', 'e', 'f']);
    /// s.rot2()?;
    /// assert_eq![s.as_slice(), &['c', 'd', 'e', 'f', 'a', 'b']];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn rot2(&mut self) -> Result<()> {
        if self.len() < 6 {
            Err(Error::NotEnoughElements(6))
        } else {
            self.array[self.len - 6..self.len].rotate_left(2);
            Ok(())
        }
    }

    /// Rotates the top six stack elements, counter-clockwise, two times.
    ///
    /// `( 1 2 3 4 5 6 -- 5 6 1 2 3 4 ) `
    ///
    /// # Errors
    /// Errors if the stack doesn't contain at least 6 elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<_, 6>::from(['a', 'b', 'c', 'd', 'e', 'f']);
    /// s.rot2()?;
    /// assert_eq![s.as_slice(), &['c', 'd', 'e', 'f', 'a', 'b']];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn rot2_cc(&mut self) -> Result<()> {
        if self.len() < 6 {
            Err(Error::NotEnoughElements(6))
        } else {
            self.array[self.len - 6..self.len].rotate_right(2);
            Ok(())
        }
    }

    /// Pushes a new element to the top of the stack.
    ///
    /// `( 1 -- 1 2 )`
    ///
    /// # Errors
    /// Errors if the stack is full.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<u8, 2>::default();
    /// s.push(1)?;
    /// s.push(2)?;
    /// assert![s.push(3).is_err()];
    /// assert_eq![s.as_slice(), &[1, 2]];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn push(&mut self, e: T) -> Result<()> {
        if self.is_full() {
            Err(Error::NotEnoughSpace(Some(1)))
        } else {
            self.array[self.len] = e;
            self.len += 1;
            Ok(())
        }
    }

    /// Pops the top stack element.
    ///
    /// `( 1 2 -- 1 )`
    ///
    /// # Errors
    /// Errors if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<_, 2>::from([1, 2]);
    /// assert_eq![2, s.pop()?];
    /// assert_eq![1, s.pop()?];
    /// assert![s.is_empty()];
    /// # Ok(()) }
    /// ```
    #[inline]
    #[cfg(feature = "unsafe_pop")]
    pub fn pop(&mut self) -> Result<T> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            self.len -= 1;
            // SAFETY: we're not gonna access the value, but move it out
            // MOTIVATION: to not depend on T: Clone
            let e = unsafe { ptr::read((self.array.get_unchecked(self.len)) as *const T) };
            Ok(e)
        }
    }
}

// `T:Clone`
impl<T: Clone, S: Storage, const CAP: usize> Stack<T, S, CAP> {
    /// Pops the top stack element.
    ///
    /// `( 1 2 -- 1 )`
    ///
    /// # Errors
    /// Errors if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<_, 2>::from([1, 2]);
    /// assert_eq![2, s.pop()?];
    /// assert_eq![1, s.pop()?];
    /// assert![s.is_empty()];
    /// # Ok(()) }
    /// ```
    #[inline]
    #[cfg(not(feature = "unsafe_pop"))]
    // safe-only version that depends on T: Clone
    pub fn pop(&mut self) -> Result<T> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            self.len -= 1;
            let e = self.array[self.len].clone();
            Ok(e)
        }
    }

    /// Duplicates the top stack element.
    ///
    /// `( 1 -- 1 1 )`
    ///
    /// # Errors
    /// Errors if the stack is either empty or full.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<u8, 2>::from([1]);
    /// s.dup()?;
    /// assert_eq![&[1, 1], s.as_slice()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn dup(&mut self) -> Result<()> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else if self.is_full() {
            Err(Error::NotEnoughSpace(Some(1)))
        } else {
            self.array[self.len] = self.array[self.len - 1].clone();
            self.len += 1;
            Ok(())
        }
    }

    /// Duplicates the top stack pair of elements.
    ///
    /// `( 1 2 -- 1 2 1 2 )`
    ///
    /// # Errors
    /// Errors if the stack doesn't have at least 2 elements,
    /// or if it doesn't have enough space for 2 extra elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<u8, 5>::from([1, 2]);
    /// s.dup2()?;
    /// assert_eq![&[1, 2, 1, 2], s.as_slice()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn dup2(&mut self) -> Result<()> {
        if self.len() < 2 {
            Err(Error::NotEnoughElements(2))
        } else if self.len() > CAP - 2 {
            Err(Error::NotEnoughSpace(Some(2)))
        } else {
            let a = self.array[self.len - 2].clone();
            let b = self.array[self.len - 1].clone();
            self.array[self.len] = a;
            self.array[self.len + 1] = b;
            self.len += 2;
            Ok(())
        }
    }

    /// Duplicates the next of stack element to the top.
    ///
    /// `( 1 2 -- 1 2 1 )`
    ///
    /// # Errors
    /// Errors stack doesn't have at least 2 elements,
    /// or if it doesn't have enough space for 1 extra element.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<u8, 3>::from([1, 2]);
    /// s.over()?;
    /// assert_eq![&[1, 2, 1], s.as_slice()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn over(&mut self) -> Result<()> {
        if self.len() < 2 {
            Err(Error::NotEnoughElements(2))
        } else if self.is_full() {
            Err(Error::NotEnoughSpace(Some(1)))
        } else {
            self.array[self.len] = self.array[self.len - 2].clone();
            self.len += 1;
            Ok(())
        }
    }

    /// Duplicates the next of stack pair of elements to the top.
    ///
    /// `( 1 2 3 4 -- 1 2 3 4 1 2 )`
    ///
    /// # Errors
    /// Errors stack doesn't have at least 4 elements,
    /// or if it doesn't have enough space for 2 extra elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()> {
    ///
    /// let mut s = DirectStack::<u8, 6>::from([1, 2, 3, 4]);
    /// s.over2()?;
    /// assert_eq![&[1, 2, 3, 4, 1, 2], s.as_slice()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn over2(&mut self) -> Result<()> {
        if self.len() < 4 {
            Err(Error::NotEnoughElements(4))
        } else if self.remaining_capacity() < 2 {
            Err(Error::NotEnoughSpace(Some(2)))
        } else {
            let a = self.array[self.len - 4].clone();
            let b = self.array[self.len - 3].clone();
            self.array[self.len] = a;
            self.array[self.len + 1] = b;
            self.len += 2;
            Ok(())
        }
    }

    /// Duplicates the top element before the next of stack element.
    ///
    /// `( 1 2 -- 2 1 2 )`
    ///
    /// # Errors
    /// Errors stack doesn't have at least 2 elements,
    /// or if it doesn't have enough space for 1 extra element.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()>  {
    ///
    /// let mut s = DirectStack::<u8, 3>::from([1, 2]);
    /// s.tuck()?;
    /// assert_eq![&[2, 1, 2], s.as_slice()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn tuck(&mut self) -> Result<()> {
        if self.len() < 2 {
            Err(Error::NotEnoughElements(2))
        } else if self.is_full() {
            Err(Error::NotEnoughSpace(Some(1)))
        } else {
            let a = self.array[self.len - 1].clone();
            self.array.swap(self.len - 2, self.len - 1);
            self.array[self.len] = a;
            self.len += 1;
            Ok(())
        }
    }

    /// Duplicates the top pair of elements before the next of stack pair of elements.
    ///
    /// `( 1 2 3 4 -- 3 4 1 2 3 4 )`
    ///
    /// # Errors
    /// Errors stack doesn't have at least 4 elements,
    /// or if it doesn't have enough space for 2 extra elements.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    /// # fn main() -> ladata::error::LadataResult<()>  {
    ///
    /// let mut s = DirectStack::<u8, 6>::from([1, 2, 3, 4]);
    /// s.tuck2()?;
    /// assert_eq![&[3, 4, 1, 2, 3, 4], s.as_slice()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn tuck2(&mut self) -> Result<()> {
        if self.len() < 4 {
            Err(Error::NotEnoughElements(4))
        } else if self.remaining_capacity() < 2 {
            Err(Error::NotEnoughSpace(Some(2)))
        } else {
            // swap2
            self.array.swap(self.len - 4, self.len - 2);
            self.array.swap(self.len - 3, self.len - 1);

            // over2
            let a = self.array[self.len - 4].clone();
            let b = self.array[self.len - 3].clone();
            self.array[self.len] = a;
            self.array[self.len + 1] = b;

            self.len += 2;
            Ok(())
        }
    }
}

// ``
impl<T, S: Storage, const CAP: usize> Stack<T, S, CAP> {
    /// Converts an array into a [`full`][Self::is_full] stack.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<_, 3>::from_array([1, 2, 3]);
    /// ```
    // IMPROVE?
    // - MAYBE arr: impl Into<[T; CAP]?> Even None…
    pub fn from_array(arr: [T; CAP]) -> Stack<T, S, CAP> {
        Self {
            array: Array::new(arr),
            len: CAP,
        }
    }

    pub fn iter(&self) -> StackIter<'_, T, S, CAP> {
        StackIter {
            stack: self,
            idx: 0,
        }
    }
}

// `T: PartialEq`
impl<T: PartialEq, S: Storage, const CAP: usize> Stack<T, S, CAP> {
    /// Returns true if the stack contains `element`.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let s = DirectStack::<_, 6>::from([5, 78, 42, 33, 9]);
    ///
    /// assert![s.contains(&9)];
    /// assert![!s.contains(&8)];
    /// ```
    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|n| n == element)
    }
}

// `T: Default`
impl<T: Default, S: Storage, const CAP: usize> Stack<T, S, CAP> {
    /// Drops the top of stack element,
    /// replacing the underlying data with the default value.
    ///
    /// `( 1 2 -- 1 )`
    ///
    /// # Errors
    /// Errors if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::DirectStack;
    ///
    /// let mut s = DirectStack::<_, 2>::from([1, 2]);
    /// s.drop_replace_default();
    /// assert_eq![s.as_slice(), &[1]];
    /// ```
    #[inline]
    pub fn drop_replace_default(&mut self) -> Result<()> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            self.array[self.len - 1] = T::default();
            self.len -= 1;
            Ok(())
        }
    }
}
