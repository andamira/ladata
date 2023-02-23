// ladata::mem::array::std_impls
//
//!
//

#[cfg(not(feature = "safe"))]
use core::mem::{self, MaybeUninit};

use core::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::mem::{CoreArray, Direct, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// Deref
impl<T, S: Storage, const LEN: usize> Deref for CoreArray<T, S, LEN> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.array.deref()
    }
}
// DerefMut
impl<T, S: Storage, const LEN: usize> DerefMut for CoreArray<T, S, LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.array.deref_mut()
    }
}

// T:Clone
impl<T: Clone, S: Storage, const LEN: usize> Clone for CoreArray<T, S, LEN>
where
    S::Stored<[T; LEN]>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            array: self.array.clone(),
            _phantom: PhantomData,
        }
    }
}

// T:Copy
impl<T: Copy, S: Storage, const LEN: usize> Copy for CoreArray<T, S, LEN> where
    S::Stored<[T; LEN]>: Copy
{
}

// T:Debug
impl<T: fmt::Debug, S: Storage, const LEN: usize> fmt::Debug for CoreArray<T, S, LEN>
where
    S::Stored<[T; LEN]>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct(stringify![CoreArray]);
        debug.field("LEN", &LEN);
        debug.field("", &self.array);
        debug.finish()
    }
}

// T:PartialEq
impl<T: PartialEq, S: Storage, const LEN: usize> PartialEq for CoreArray<T, S, LEN>
where
    S::Stored<[T; LEN]>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.len() == other.len()
    }
}
// T:Eq
impl<T: Eq, S: Storage, const LEN: usize> Eq for CoreArray<T, S, LEN> where S::Stored<[T; LEN]>: Eq {}

// S:() + T:Default
impl<T: Default, const LEN: usize> Default for CoreArray<T, (), LEN> {
    /// Returns an empty array, allocated in the stack,
    /// using the default value to fill the remaining free data.
    fn default() -> Self {
        #[cfg(not(feature = "safe"))]
        let data = {
            let mut arr: [MaybeUninit<T>; LEN] = unsafe { MaybeUninit::uninit().assume_init() };
            for i in &mut arr[..] {
                let _ = i.write(T::default());
            }
            unsafe { mem::transmute_copy::<_, [T; LEN]>(&arr) }
        };

        #[cfg(feature = "safe")]
        let data = core::array::from_fn(|_| T::default());

        CoreArray {
            array: Direct::new(data),
            _phantom: PhantomData,
        }
    }
}

// S:Boxed + T:Default
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T: Default, const LEN: usize> Default for CoreArray<T, Boxed, LEN> {
    /// Returns an empty array, allocated in the heap,
    /// using the default value to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::mem::BoxedArray;
    ///
    /// let mut s = BoxedArray::<i32, 100>::default();
    /// ```
    fn default() -> Self {
        #[cfg(feature = "safe")]
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

        #[cfg(not(feature = "safe"))]
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

        CoreArray {
            array: data,
            _phantom: PhantomData,
        }
    }
}

impl<T, const LEN: usize> From<CoreArray<T, (), LEN>> for [T; LEN] {
    fn from(array: CoreArray<T, (), LEN>) -> [T; LEN] {
        array.array.0
    }
}
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T, const LEN: usize> From<CoreArray<T, Boxed, LEN>> for Box<[T; LEN]> {
    fn from(array: CoreArray<T, Boxed, LEN>) -> Box<[T; LEN]> {
        array.array
    }
}

impl<T: Default, I, const LEN: usize> From<I> for CoreArray<T, (), LEN>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a array filled with an iterator, in the stack.
    ///
    /// If the `iterator` length is less than the array length `LEN`,
    /// the missing elements will be the default value of `T`.
    ///
    /// # Examples
    /// ```
    /// use ladata::mem::DirectArray;
    ///
    /// let s: DirectArray<_, 4> = [1, 2, 3].into();
    ///
    /// assert_eq![s.as_slice(), &[1, 2, 3, 0]];
    /// ```
    fn from(iterator: I) -> CoreArray<T, (), LEN> {
        let mut iterator = iterator.into_iter();

        #[cfg(not(feature = "safe"))]
        let data = {
            let mut arr: [MaybeUninit<T>; LEN] = unsafe { MaybeUninit::uninit().assume_init() };
            for i in &mut arr[..] {
                if let Some(e) = iterator.next() {
                    let _ = i.write(e);
                } else {
                    let _ = i.write(T::default());
                }
            }
            unsafe { mem::transmute_copy::<_, [T; LEN]>(&arr) }
        };

        #[cfg(feature = "safe")]
        let data = core::array::from_fn(|_| {
            if let Some(e) = iterator.next() {
                e
            } else {
                T::default()
            }
        });

        CoreArray {
            array: Direct::new(data),
            _phantom: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T: Default, I, const LEN: usize> From<I> for CoreArray<T, Boxed, LEN>
where
    I: IntoIterator<Item = T>,
{
    /// Returns a array filled with an iterator, in the heap.
    ///
    /// # Examples
    /// ```
    /// use ladata::mem::BoxedArray;
    ///
    /// let s: BoxedArray<_, 4> = [1, 2, 3].into();
    ///
    /// assert_eq![s.as_slice(), &[1, 2, 3, 0]];
    /// ```
    fn from(iterator: I) -> CoreArray<T, Boxed, LEN> {
        let mut iterator = iterator.into_iter();

        #[cfg(feature = "safe")]
        let data = {
            let mut v = Vec::<T>::with_capacity(LEN);

            for _ in 0..LEN {
                if let Some(e) = iterator.next() {
                    v.push(e);
                } else {
                    v.push(T::default());
                }
            }
            let Ok(array) = v.into_boxed_slice().try_into() else {
                panic!("Can't turn the boxed slice into a boxed array");
            };
            array
        };

        #[cfg(not(feature = "safe"))]
        let data = {
            let mut v = Vec::<T>::with_capacity(LEN);

            for _ in 0..LEN {
                if let Some(e) = iterator.next() {
                    v.push(e);
                } else {
                    v.push(T::default());
                }
            }
            let slice = v.into_boxed_slice();
            let raw_slice = Box::into_raw(slice);
            // SAFETY: pointer comes from using `into_raw`, and capacity is right.
            unsafe { Box::from_raw(raw_slice as *mut [T; LEN]) }
        };

        CoreArray {
            array: data,
            _phantom: PhantomData,
        }
    }
}
