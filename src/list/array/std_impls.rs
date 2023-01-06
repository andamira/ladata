// ladata::list::array::std_impls
//
//!
//

#[cfg(not(feature = "no_unsafe"))]
use core::mem::{self, MaybeUninit};

use core::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::Array;
use crate::mem::{Raw, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// Deref
impl<T, S: Storage, const LEN: usize> Deref for Array<T, S, LEN> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.array.deref()
    }
}
// DerefMut
impl<T, S: Storage, const LEN: usize> DerefMut for Array<T, S, LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.array.deref_mut()
    }
}

// T:Clone
impl<T: Clone, S: Storage, const LEN: usize> Clone for Array<T, S, LEN>
where
    S::Container<[T; LEN]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
            _phantom: PhantomData,
        }
    }
}

// T:Copy
impl<T: Copy, S: Storage, const LEN: usize> Copy for Array<T, S, LEN> where
    S::Container<[T; LEN]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const LEN: usize> fmt::Debug for Array<T, S, LEN>
where
    S::Container<[T; LEN]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![Array]);
        debug.field("LEN", &LEN);
        debug.field("", &self.array);
        debug.finish()
    }
}

// T:PartialEq
impl<T: PartialEq, S: Storage, const LEN: usize> PartialEq for Array<T, S, LEN>
where
    S::Container<[T; LEN]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.len() == other.len()
    }
}
// T:Eq
impl<T: Eq, S: Storage, const LEN: usize> Eq for Array<T, S, LEN> where S::Container<[T; LEN]>: Eq {}

// S:() + T:Default
impl<T: Default, const LEN: usize> Default for Array<T, (), LEN> {
    /// Returns an empty array, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        #[cfg(not(feature = "no_unsafe"))]
        let data = {
            let mut arr: [MaybeUninit<T>; LEN] = unsafe { MaybeUninit::uninit().assume_init() };
            for i in &mut arr[..] {
                let _ = i.write(T::default());
            }
            unsafe { mem::transmute_copy::<_, [T; LEN]>(&arr) }
        };

        #[cfg(feature = "no_unsafe")]
        let data = core::array::from_fn(|_| T::default());

        Array {
            array: Raw::new(data),
            _phantom: PhantomData,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "std")]
impl<T: Default, const LEN: usize> Default for Array<T, Boxed, LEN> {
    /// Returns an empty array, allocated in the heap,
    /// using the default value to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::list::BoxedArray;
    ///
    /// let mut s = BoxedArray::<i32, 100>::default();
    /// ```
    fn default() -> Self {
        #[cfg(feature = "no_unsafe")]
        let data = {
            let mut v = Vec::<T>::with_capacity(LEN);

            for _ in 0..LEN {
                v.push(T::default());
            }

            let Ok(array) = v.into_boxed_slice().try_into() else {
                panic!("Can't turn the boxed slice into a boxed array");
            };
            array
        };

        #[cfg(not(feature = "no_unsafe"))]
        let data = {
            let mut v = Vec::<T>::with_capacity(LEN);

            for _ in 0..LEN {
                v.push(T::default());
            }

            let slice = v.into_boxed_slice();
            let raw_slice = Box::into_raw(slice);
            // SAFETY: pointer comes from using `into_raw`, and capacity is right.
            unsafe { Box::from_raw(raw_slice as *mut [T; LEN]) }
        };

        Array {
            array: data,
            _phantom: PhantomData,
        }
    }
}

impl<T, const LEN: usize> From<Array<T, (), LEN>> for [T; LEN] {
    fn from(array: Array<T, (), LEN>) -> [T; LEN] {
        array.array.0
    }
}
