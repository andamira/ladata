// ladata::mem::stack::methods
//
//! Arrays.
//

#[cfg(not(feature = "safe"))]
use core::mem::{self, MaybeUninit};

use core::marker::PhantomData;

use crate::mem::{Array, Raw, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

// ``
impl<T, S: Storage, const LEN: usize> Array<T, S, LEN> {
    pub fn new(array: [T; LEN]) -> Self {
        Self {
            array: array.into(),
            _phantom: PhantomData,
        }
    }
}

// `S:() + T:Clone`
impl<T: Clone, const LEN: usize> Array<T, (), LEN> {
    /// Returns an array, allocated in the stack,
    /// filled with `element`, cloned.
    ///
    /// # Examples
    /// ```
    /// use ladata::mem::Array;
    ///
    /// let s = Array::<_, (), 16>::with('\0');
    /// ```
    pub fn with(element: T) -> Self {
        #[cfg(not(feature = "safe"))]
        let data = Raw::new({
            let mut arr: [MaybeUninit<T>; LEN] = unsafe { MaybeUninit::uninit().assume_init() };

            for i in &mut arr[..] {
                let _ = i.write(element.clone());
            }

            // TEMP:FIX: can't use transmute for now:
            // - https://github.com/rust-lang/rust/issues/62875
            // - https://github.com/rust-lang/rust/issues/61956
            // mem::transmute::<_, [T; LEN]>(&arr)
            unsafe { mem::transmute_copy::<_, [T; LEN]>(&arr) }
        });

        #[cfg(feature = "safe")]
        let data = Raw::new(core::array::from_fn(|_| element.clone()));

        Self {
            array: data,
            _phantom: PhantomData,
        }
    }
}

// `S:Boxed + T:Clone`
#[cfg(feature = "std")]
impl<T: Clone, const LEN: usize> Array<T, Boxed, LEN> {
    /// Returns an empty stack, allocated in the heap,
    /// using `element` to fill the remaining free data.
    ///
    /// # Examples
    /// ```
    /// use ladata::mem::BoxedArray;
    ///
    /// let mut s = BoxedArray::<_, 1_000>::with(0);
    /// ```
    pub fn with(element: T) -> Self {
        #[cfg(feature = "safe")]
        let data = {
            let mut v = Vec::<T>::with_capacity(LEN);

            for _ in 0..LEN {
                v.push(element.clone());
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
                v.push(element.clone());
            }

            let slice = v.into_boxed_slice();
            let raw_slice = Box::into_raw(slice);
            // SAFETY: pointer comes from using `into_raw`, and capacity is right.
            unsafe { Box::from_raw(raw_slice as *mut [T; LEN]) }
        };

        Self {
            array: data,
            _phantom: PhantomData,
        }
    }
}

// `T: PartialEq`
impl<T: PartialEq, S: Storage, const CAP: usize> Array<T, S, CAP> {
    /// Returns true if the array contains `element`.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::Array;
    ///
    /// let a = Array::<_, (), 5>::new([5, 78, 42, 33, 9]);
    ///
    /// assert![a.contains(&9)];
    /// assert![!a.contains(&8)];
    /// ```
    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|n| n == element)
    }
}

// ``
impl<T, S: Storage, const LEN: usize> Array<T, S, LEN> {
    /// Returns the number of elements in the array.
    #[inline]
    pub const fn len(&self) -> usize {
        LEN
    }

    /// Returns `true` if the array has a length of 0.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        LEN == 0
    }
}
