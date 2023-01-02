// ladata::line::stack::std_impls
//
//!
//

use core::fmt;

#[cfg(not(feature = "no_unsafe"))]
use core::mem::{self, MaybeUninit};

use super::{Stack, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// T:Clone
impl<T: Clone, S: Storage, const CAP: usize> Clone for Stack<T, S, CAP>
where
    S::Container<[T; CAP]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            stack: self.stack.clone(),
            len: self.len,
        }
    }
}

// T:Copy
impl<T: Copy, S: Storage, const CAP: usize> Copy for Stack<T, S, CAP> where
    S::Container<[T; CAP]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const CAP: usize> fmt::Debug for Stack<T, S, CAP>
where
    S::Container<[T; CAP]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![Stack]);
        debug.field("CAP", &CAP).field("len", &self.len);

        if CAP <= 6 {
            debug.field("nodes", &self.stack);
        } else {
            // IMPROVE: show first 3 and last 3
            debug.field("stack { ... }", &());
        }
        debug.finish()
    }
}

// S:() + T:Default
impl<T: Default, const CAP: usize> Default for Stack<T, (), CAP> {
    /// Returns an empty stack, allocated in the stack,
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
            stack: data.into(),
            len: 0,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "std")]
impl<T: Default, const CAP: usize> Default for Stack<T, Boxed, CAP> {
    /// Returns an empty stack, allocated in the heap,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        let data = {
            let mut v = Vec::<T>::with_capacity(CAP);

            for _ in 0..v.len() {
                v.push(T::default());
            }

            let Ok(array) = v.into_boxed_slice().try_into() else {
                panic!("Can't turn the boxed slice into a boxed array");
            };
            array
        };

        Self {
            stack: data,
            len: 0,
        }
    }
}

impl<T, S: Storage, const CAP: usize> From<[T; CAP]> for Stack<T, S, CAP> {
    /// Converts an array to a [`full`][Self::is_full] stack.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Stack;
    ///
    /// let s: Stack<_, (), 3> = [1, 2, 3].into();
    /// ```
    fn from(arr: [T; CAP]) -> Stack<T, S, CAP> {
        Self {
            // CHECK
            stack: arr.into(),
            len: CAP,
        }
    }
}
