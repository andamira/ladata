// ladata::misc::count
//
//! A type that allows counting a limited number of elements.
//

use core::fmt;

use crate::error::{LadataError as Error, LadataResult as Result};

use nonmax::{NonMaxU16, NonMaxU32, NonMaxU8, NonMaxUsize};

#[rustfmt::skip]
macro_rules! count {
    // $name : the type name
    // $B : byte size
    // $b : bit size
    // $t : inner count type
    // $nmt: nonmax inner count type
    // $idx: index size to convert to
    ($name:ident, $B:literal, $b:literal, $t:ty, $nmt:ty, $idx:ident) => { paste::paste! {
        #[doc = "An " $b "-bit count that is known not to equal [`" $t "::MAX`]."]
        ///
        #[doc = "It can count from `0` to [`" $t "::MAX`]`-1` elements."]
        ///
        /// This enables some memory layout optimization.
        #[doc = "For example, `Option<Count" $b ">` is the same size as `u" $b "`."]
        ///
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name($nmt);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "Count {{ {} }}", self.0)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "{}", self.0)
            }
        }

        /// # constructors
        impl $name {
            /// Returns a new `Count` at 0.
            #[inline]
            pub fn new() -> Self {
                Self($nmt::new(0).unwrap())
            }
        }

        /// # methods
        impl $name {
            // /// Retuns the inner primitive type,
            // /// or `None` if it's invalid.
            // #[inline]
            // pub const fn get(&self) -> $t {
            //     if let Some(i) = self.0 {
            //         Some(i.get())
            //     } else {
            //         None
            //     }
            // }

            /// Returns the inner primitive type as a `usize`.
            ///
            /// If the count is invalid, the maximum primitive value
            /// casted to usize will be returned.
            #[inline]
            pub const fn as_usize(&self) -> usize {
                self.0.get() as usize
                // core::cmp::min(self.as_primitive(), usize::MAX as $t) as usize
            }

            /// Retuns the inner primitive type.
            ///
            /// If the count pointed to nothing, the maximum primitive value
            /// will be returned.
            #[inline]
            pub const fn as_primitive(&self) -> $t {
                self.0.get()
            }

            /// Increments the counter by 1.
            ///
            /// # Errors
            #[doc = "Overflows if the counter reaches [`" $t "::MAX`]."]
            #[inline]
            pub fn increment(&mut self) -> Result<()> {
                self.increment_by(1)
            }
            /// Increments the counter by `increment`.
            ///
            /// # Errors
            #[doc = "Overflows if the counter reaches [`" $t "::MAX`]."]
            #[inline]
            pub fn increment_by(&mut self, increment: $t) -> Result<()> {
                let c = self.0.get();
                let c = c.checked_add(increment).ok_or(Error::Overflow)?;
                self.0 = $nmt::new(c).ok_or(Error::Overflow)?;
                Ok(())
            }

            /// Decrements the counter by 1.
            ///
            /// # Errors
            #[doc = "Underflows if the counter reaches below 0."]
            #[inline]
            pub fn decrement(&mut self) -> Result<()> {
                self.decrement_by(1)
            }

            /// Decrements the counter by `decrement`.
            ///
            /// # Errors
            #[doc = "Underflows if the counter reaches below 0."]
            #[inline]
            pub fn decrement_by(&mut self, decrement: $t) -> Result<()> {
                let c = self.0.get();
                let c = c.checked_sub(decrement).ok_or(Error::Underflow)?;
                self.0 = $nmt::new(c).ok_or(Error::Underflow)?;
                Ok(())
            }
        }

        impl From<[<$name>]> for super::$idx {
            fn from(count: [<$name>]) -> super::$idx {
                count.0.into()
            }
        }

    }};
}

count![Count8, 1, 8, u8, NonMaxU8, Index8];
count![Count16, 2, 16, u16, NonMaxU16, Index16];
count![Count32, 4, 32, u32, NonMaxU32, Index32];

#[cfg(target_pointer_width = "8")]
count![CountUsize, 1, 8, u8, NonMaxUsize, IndexUsize];
#[cfg(target_pointer_width = "16")]
count![CountUsize, 2, 16, u16, NonMaxUsize, IndexUsize];
#[cfg(target_pointer_width = "32")]
count![CountUsize, 4, 32, u32, NonMaxUsize, IndexUsize];
#[cfg(target_pointer_width = "64")]
count![CountUsize, 8, 64, usize, NonMaxUsize, IndexUsize];

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn count_size() {
        assert_eq!(1, size_of::<Count8>());
        assert_eq!(2, size_of::<Count16>());
        assert_eq!(4, size_of::<Count32>());
    }
}
