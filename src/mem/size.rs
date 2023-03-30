// ladata::mem::size
//
//! Size trait.
//

use core::mem::{size_of, size_of_val};

/// Convenience trait for size related information.
///
/// It's automatically implemented for all types.
pub trait Size {
    /// Returns the type size in bytes.
    fn byte_size(&self) -> usize {
        size_of_val(self)
    }

    /// Returns the type size in bits.
    fn bit_size(&self) -> usize {
        self.byte_size() * 8
    }

    /// Returns the byte size of a pointer in the current platform (`usize`, `isize`).
    fn pointer() -> usize {
        size_of::<usize>()
    }

    /// Returns the size ratio between a pointer size and the current type.
    ///
    /// For example: the ratio will be (1, 1) if both sizes are the same,
    /// (2, 1) if a pointer is double the size, and (1, 2) if a pointer is
    /// half the size.
    fn pointer_ratio(&self) -> (usize, usize) {
        fn gcd(m: usize, n: usize) -> usize {
            if n == 0 {
                m
            } else {
                gcd(n, m % n)
            }
        }
        let s = self.byte_size();
        let g = gcd(size_of::<usize>(), s);
        (size_of::<usize>() / g, s / g)
    }
}

impl<T> Size for T {}
