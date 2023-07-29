// ladata::misc::count
//
//! A type that allows counting a limited number of elements.
//

use crate::error::{LadataError as Error, LadataResult as Result};
use core::fmt;
use devela::{NonMaxU16, NonMaxU32, NonMaxU8, NonMaxUsize};

#[rustfmt::skip]
macro_rules! count {
    // $name : the type name
    // $B : byte size
    // $b : bit size
    // $t : inner count type (e.g. u8, usize)
    // $T : Camel case inner count type (e.g. U8, Usize)
    // $idx: corresponding index size to convert to
    ($name:ident, $B:literal, $b:literal, $t:ty, $T:ty, $idx:ident) => { devela::paste! {
        #[doc = "An " $b "-bit counter, from `0` to [`" $t "::MAX`] elements."]
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name($t);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "Counter{} {{ {} }}", $b, self.0)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "{}", self.0)
            }
        }

        /// # constructors
        impl $name {
            /// Returns a new `Counter` at `0`.
            #[inline]
            pub const fn new() -> Self {
                Self(0)
            }
        }

        /// # methods
        impl $name {
            /// Retuns the inner primitive type.
            ///
            /// If the count pointed to nothing, the maximum primitive value
            /// will be returned.
            #[inline]
            pub const fn as_primitive(&self) -> $t {
                self.0
            }

            /// Returns the inner primitive type as a `usize`.
            ///
            /// If the count is invalid, the maximum primitive value
            /// casted to usize will be returned.
            #[inline]
            pub const fn as_usize(&self) -> usize {
                // IMPROVE
                self.0 as usize
            }

            /// Returns a non-max index that points to `1` less than the current count.
            ///
            /// If the current count is `0`, the index will point to `None`.
            pub const fn as_current_index(&self) -> super::$idx {
                #[cfg(feature = "safe")]
                match self.0 {
                    0 => super::$idx::none(),
                    _ => {
                        super::$idx(Some(
                            if let Some(nmt) = [<NonMax$T>]::new(self.0 - 1) {
                                nmt
                            } else {
                                unreachable![]
                            }
                        ))
                    }
                }

                // SAFETY: the value can't ever be the maximum
                #[cfg(not(feature = "safe"))]
                match self.0 {
                    0 => super::$idx::none(),
                    _ => super::$idx(Some(unsafe { [<NonMax$T>]::new_unchecked(self.0 - 1) }) )
                }
            }

            /// Returns a non-max index that points to the current count value.
            ///
            #[doc = "If the current count is [`" $t "::MAX`], the index will point to `None`."]
            pub const fn as_next_index(&self) -> super::$idx {
                #[cfg(feature = "safe")]
                match self.0 {
                    $t::MAX => super::$idx::none(),
                    _ => {
                        super::$idx(Some(
                            if let Some(nmt) = [<NonMax$T>]::new(self.0) {
                                nmt
                            } else {
                                unreachable![]
                            }
                        ))
                    }
                }

                // SAFETY: the value can't ever be the maximum
                #[cfg(not(feature = "safe"))]
                match self.0 {
                    $t::MAX => super::$idx::none(),
                    _ => super::$idx(Some(unsafe { [<NonMax$T>]::new_unchecked(self.0) }) )
                }
            }

            //

            /// Increments the counter by `1`.
            ///
            /// # Errors
            #[doc = "Overflows if the counter would exceed [`" $t "::MAX`]."]
            #[inline]
            pub fn increment(&mut self) -> Result<()> {
                self.increment_by(1)
            }
            /// Increments the counter by `increment`.
            ///
            /// # Errors
            #[doc = "Overflows if the counter would exceed [`" $t "::MAX`]."]
            #[inline]
            pub fn increment_by(&mut self, increment: $t) -> Result<()> {
                self.0 = self.0.checked_add(increment).ok_or(Error::Overflow)?;
                Ok(())
            }
            /// Returns a copy of the counter incremented by `1`.
            ///
            /// # Errors
            #[doc = "Overflows if the counter would exceed [`" $t "::MAX`]."]
            #[inline]
            pub const fn incremented(&self) -> Result<Self> {
                self.incremented_by(1)
            }
            /// Returns a copy of the counter incremented by `increment`.
            ///
            /// # Errors
            #[doc = "Overflows if the counter would exceed [`" $t "::MAX`]."]
            #[inline]
            pub const fn incremented_by(&self, increment: $t) -> Result<Self> {
                if let Some(result) = self.0.checked_add(increment) {
                    Ok(Self(result))
                } else {
                    Err(Error::Overflow)
                }
            }

            /// Decrements the counter by `1`.
            ///
            /// # Errors
            /// Underflows if the counter would subceed `0`.
            #[inline]
            pub fn decrement(&mut self) -> Result<()> {
                self.decrement_by(1)
            }
            /// Decrements the counter by `decrement`.
            ///
            /// # Errors
            /// Underflows if the counter would subceed `0`.
            #[inline]
            pub fn decrement_by(&mut self, decrement: $t) -> Result<()> {
                self.0 = self.0.checked_sub(decrement).ok_or(Error::Underflow)?;
                Ok(())
            }
            /// Returns a copy of the counter decremented by `1`.
            ///
            /// # Errors
            /// Underflows if the counter would subceed `0`.
            #[doc = "Underflow if the counter would subceed [`" $t "::MAX`]."]
            #[inline]
            pub const fn decremented(&self) -> Result<Self> {
                self.decremented_by(1)
            }
            /// Returns a copy of the counter decremented by `increment`.
            ///
            /// # Errors
            /// Underflows if the counter would subceed `0`.
            #[inline]
            pub const fn decremented_by(&self, decrement: $t) -> Result<Self> {
                if let Some(result) = self.0.checked_sub(decrement) {
                    Ok(Self(result))
                } else {
                    Err(Error::Underflow)
                }
            }
        }
    }};
}

count![Counter8, 1, 8, u8, U8, NonMaxIndex8];
count![Counter16, 2, 16, u16, U16, NonMaxIndex16];
count![Counter32, 4, 32, u32, U32, NonMaxIndex32];

#[cfg(target_pointer_width = "8")]
count![CounterUsize, 1, 8, usize, Usize, NonMaxIndexUsize];
#[cfg(target_pointer_width = "16")]
count![CounterUsize, 2, 16, usize, Usize, NonMaxIndexUsize];
#[cfg(target_pointer_width = "32")]
count![CounterUsize, 4, 32, usize, Usize, NonMaxIndexUsize];
#[cfg(target_pointer_width = "64")]
count![CounterUsize, 8, 64, usize, Usize, NonMaxIndexUsize];

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn counter_size() {
        assert_eq!(1, size_of::<Counter8>());
        assert_eq!(2, size_of::<Counter16>());
        assert_eq!(4, size_of::<Counter32>());
    }
}
