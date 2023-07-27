// ladata::mem::size
//
//! Traits related to memory size.
//

use core::mem::{size_of, size_of_val};

mod bit_size;
pub use bit_size::*;

impl<T> DataSize for T {}

/// Convenience trait for Size related information.
///
/// It is automatically implemented for every sized type.
pub trait DataSize: Sized {
    /// The size of this type in bytes, rounded up if it's not a multiple of 8.
    const BYTES: usize = size_of::<Self>();

    /// The pointer size in bits for the current platform.
    const UBITS: usize = Self::UBYTES * 8;

    /// The pointer size in bytes for the current platform.
    const UBYTES: usize = size_of::<usize>();

    /// Returns the size ratio between [`UBYTES`][Self#constant.UBYTES] and
    /// [`BYTES`][Self#constant.BYTES].
    ///
    /// For example: the ratio will be (1, 1) if both sizes are the same,
    /// (2, 1) if a pointer is double the byte size, and (1, 2) if a pointer is
    /// half the byte size.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::DataSize;
    ///
    /// assert_eq![1_usize.pointer_ratio(), (1, 1)];
    /// assert_eq!["slice".pointer_ratio(), (1, 2)];
    /// assert_eq![String::new().pointer_ratio(), (1, 3)];
    ///
    /// #[cfg(target_pointer_width = "64")]
    /// assert_eq!['a'.pointer_ratio(), (2,1)]
    /// ```
    fn pointer_ratio(&self) -> (usize, usize) {
        #[inline]
        fn gcd(m: usize, n: usize) -> usize {
            if n == 0 {
                m
            } else {
                gcd(n, m % n)
            }
        }
        let g = gcd(Self::UBYTES, Self::BYTES);
        (Self::UBYTES / g, Self::BYTES / g)
    }

    /// Returns the size in bytes of this type, in the stack.
    ///
    /// Ignores any allocated resources in the heap.
    fn stack_byte_size(&self) -> usize {
        size_of_val(self)
    }

    // /// Returns the size in bytes of this type, in the heap.
    // fn heap_byte_size(&self) -> usize {
    //     todo![] // TODO
    // }
}
