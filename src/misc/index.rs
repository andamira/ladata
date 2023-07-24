// ladata::misc::index
//
//! A type that allows indexing a limited number of elements.
//

use core::fmt;

use devela::{NonMaxU16, NonMaxU32, NonMaxU8, NonMaxUsize};

#[rustfmt::skip]
macro_rules! index {
    // $name : the type name
    // $B : byte size
    // $b : bit size
    // $t : inner index type
    // $nmt: nonmax inner index type
    ($name:ident, $B:literal, $b:literal, $t:ty, $nmt:ty) => { devela::paste! {
        #[doc = "An " $b "-bit index that is known not to equal [`" $t "::MAX`]."]
        ///
        #[doc = "It can index from `0` to [`" $t "::MAX`]`-1` elements."]
        ///
        /// This enables some memory layout optimization.
        #[doc = "For example, `Option<Index" $b ">` is the same size as `u" $b "`."]
        ///
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(Option<$nmt>);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(index) = self.0 {
                    write!(f,  "{}", index)
                } else {
                    write!(f,  "_")
                }
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "{:?}", self)
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
            ///
            /// If the index pointed to nothing, the maximum primitive value
            /// casted to usize will be returned.
            #[inline]
            pub fn as_usize(&self) -> usize {
                core::cmp::min(self.as_primitive(), usize::MAX as $t) as usize
            }

            /// Retuns the inner primitive type.
            ///
            /// If the index pointed to nothing, the maximum primitive value
            /// will be returned.
            #[inline]
            pub const fn as_primitive(&self) -> $t {
                if let Some(i) = self.0 {
                    i.get()
                } else {
                    $t::MAX
                }
            }
        }

        impl From<$t> for $name {
            // converts $t::MAX to None
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

index![Index8, 1, 8, u8, NonMaxU8];
index![Index16, 2, 16, u16, NonMaxU16];
index![Index32, 4, 32, u32, NonMaxU32];

#[cfg(target_pointer_width = "8")]
index![IndexUsize, 1, 8, usize, NonMaxUsize];
#[cfg(target_pointer_width = "16")]
index![IndexUsize, 2, 16, usize, NonMaxUsize];
#[cfg(target_pointer_width = "32")]
index![IndexUsize, 4, 32, usize, NonMaxUsize];
#[cfg(target_pointer_width = "64")]
index![IndexUsize, 8, 64, usize, NonMaxUsize];

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
