// ladata::misc::index
//
//! An type that allows indexing a limited number of elements, or
//

use core::fmt::{self, Debug};

use nonmax::{NonMaxU16, NonMaxU32, NonMaxU8, NonMaxUsize};

#[rustfmt::skip]
macro_rules! inner_index {
    // $name : the type name
    // $B : byte size
    // $b : bit size
    // $t : inner index type
    // $nmt: nonmax inner index type
    ($name:ident, $B:literal, $b:literal, $t:ty, $nmt:ty) => { paste::paste! {
        #[doc = "An " $b "-bit index that is known not to equal [`" $t "::MAX`]."]
        ///
        #[doc = "It can index from `0` to [`" $t "::MAX`]`-1` elements."]
        ///
        /// This enables some memory layout optimization.
        #[doc = "For example, `Option<Index" $b ">` is the same size as `u" $b "`."]
        ///
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(Option<$nmt>);

        impl Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "{:?}", self.0)
            }
        }

        /// # constructors
        impl $name {
            /// Returns a new `Index` pointing to `index`.
            #[doc = "or `None` if the provided `index` equals [`" $t "::MAX`]."]
            #[inline]
            pub const fn new(index: $t) -> Option<Self> {
                if let Some(i) = $nmt::new(index) {
                    Some(Self(Some(i)))
                } else {
                    None
                }
            }

            /// Returns a new index pointing to nothing.
            #[inline]
            pub const fn none() -> Self {
                Self(None)
            }
        }

        /// # methods
        impl $name {
            /// Returns `true` if the index is pointing to something.
            #[inline]
            pub const fn is_some(&self) -> bool {
                self.0.is_some()
            }

            /// Returns `true` if the index is pointing to nothing.
            #[inline]
            pub const fn is_none(&self) -> bool {
                self.0.is_none()
            }

            /// Retuns the inner primitive type,
            /// or `None` if it's pointing to nothing.
            #[inline]
            pub const fn get(&self) -> Option<$t> {
                if let Some(i) = self.0 {
                    Some(i.get())
                } else {
                    None
                }
            }

            /// Returns the inner primitive type as a `usize`.
            #[inline]
            pub const fn as_usize(&self) -> Option<usize> {
                if let Some(v) = self.get() {
                    Some(v as usize)
                } else {
                    None
                }
            }

            /// Increments by 1 the inner value,
            /// as long as the index is pointing to something,
            /// and the index is not out of bounds.
            // IMPROVE: return Result
            #[must_use]
            #[inline]
            pub fn increment(&mut self) -> Option<()> {
                if let Some(i) = self.0 {
                    self.0 = $nmt::new(i.get().checked_add(1)?);
                    Some(())
                } else {
                    None
                }
            }
            /// Decrements by 1 the inner value,
            /// as long as the index is pointing to something,
            /// and the index is not out of bounds.
            // IMPROVE return Result
            #[must_use]
            #[inline]
            pub fn decrement(&mut self) -> Option<()> {
                if let Some(i) = self.0 {
                    self.0 = $nmt::new(i.get().checked_sub(1)?);
                    Some(())
                } else {
                    None
                }
            }
        }

        impl From<$t> for $name {
            /// Converts $t::MAX to None
            #[inline]
            fn from(index: $t) -> Self {
                if let Some(i) = $nmt::new(index) {
                    Self(Some(i))
                } else {
                    Self(None)
                }
            }
        }
        impl From<Option<$t>> for $name {
            #[inline]
            fn from(index: Option<$t>) -> Self {
                if let Some(i) = index {
                    Self($nmt::new(i))
                } else {
                    Self(None)
                }
            }
        }
        impl From<$nmt> for $name {
            #[inline]
            fn from(index: $nmt) -> Self {
                Self(Some(index))
            }
        }
    }};
}

inner_index![Index8, 1, 8, u8, NonMaxU8];
inner_index![Index16, 2, 16, u16, NonMaxU16];
inner_index![Index32, 4, 32, u32, NonMaxU32];

#[cfg(target_pointer_width = "8")]
inner_index![IndexUsize, 1, 8, u8, NonMaxUsize];
#[cfg(target_pointer_width = "16")]
inner_index![IndexUsize, 2, 16, u16, NonMaxUsize];
#[cfg(target_pointer_width = "32")]
inner_index![IndexUsize, 4, 32, u32, NonMaxUsize];
#[cfg(target_pointer_width = "64")]
inner_index![IndexUsize, 8, 64, usize, NonMaxUsize];

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn index_size() {
        assert_eq!(1, size_of::<Index8>());
        assert_eq!(2, size_of::<Index16>());
        assert_eq!(4, size_of::<Index32>());
    }
}
