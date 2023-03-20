// ladata::struct::bit
//
//! Bit arrays.
//!
//! <https://en.wikipedia.org/wiki/Bit_array>
//

#[cfg(feature = "std")]
use crate::mem::Boxed;

use core::fmt;

use crate::all::{
    Array, /* ArrayAdt, */ CollectionAdt, LadataError as Error, LadataResult as Result,
    Storage,
};

/// A bit array, backed by an [`Array`] of bytes.
pub struct BitArray<S: Storage, const LEN: usize, const U8LEN: usize> {
    array: Array<u8, S, U8LEN>,
}

/// A [`BitArray`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedBitArray<const LEN: usize, const U8LEN: usize> = BitArray<Boxed, LEN, U8LEN>;

/// A [`BitArray`] stored in the stack.
pub type DirectBitArray<const LEN: usize, const U8LEN: usize> = BitArray<(), LEN, U8LEN>;

impl<const LEN: usize, const U8LEN: usize> Clone for BitArray<(), LEN, U8LEN> {
    fn clone(&self) -> Self {
        BitArray { array: self.array }
    }
}
impl<const LEN: usize, const U8LEN: usize> Copy for BitArray<(), LEN, U8LEN> {}

impl<const LEN: usize, const U8LEN: usize> fmt::Debug for BitArray<(), LEN, U8LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "std")]
        let mut bits = String::new();
        #[cfg(feature = "std")]
        for byte in self.array.iter() {
            bits += &format!["{byte:b}"];
        }

        let mut debug = f.debug_struct(stringify![DirectBitArray]);
        debug.field("LEN", &LEN);
        debug.field("U8LEN", &U8LEN);

        #[cfg(feature = "std")]
        debug.field("", &bits);

        debug.finish()
    }
}

impl<S: Storage, const LEN: usize, const U8LEN: usize> BitArray<S, LEN, U8LEN> {
    /// Returns an empty bit array, of `LEN` bits, stored in U8LEN bytes.
    ///
    /// # Errors
    /// if `LEN` > `U8LEN` * 8.
    pub fn new() -> Result<Self> {
        if U8LEN * 8 >= LEN {
            Ok(Self {
                array: Array::new([0_u8; U8LEN]),
            })
        } else {
            Err(Error::DimensionMismatch)
        }
    }
    /// Gets the bit's value at `index`.
    pub fn get_bit(&self, index: usize) -> Result<bool> {
        if index >= LEN {
            return Err(Error::IndexOutOfBounds(index));
        }
        let byte_index = index / 8;
        let bit_within_byte_index = index % 8;
        let bit_mask = 1 << bit_within_byte_index;
        Ok(self.array[byte_index] & bit_mask != 0)
    }
    /// Sets the bit at `index` to `value`.
    pub fn set_bit(&mut self, index: usize, value: bool) -> Result<()> {
        if index >= LEN {
            return Err(Error::IndexOutOfBounds(index));
        }
        let byte_index = index / 8;
        let bit_within_byte_index = index % 8;
        let bit_mask = 1 << bit_within_byte_index;
        if value {
            self.array[byte_index] |= bit_mask;
        } else {
            self.array[byte_index] &= !bit_mask;
        }
        Ok(())
    }
}
impl<S: Storage, const LEN: usize, const U8LEN: usize> Default for BitArray<S, LEN, U8LEN> {
    /// Returns an empty bit array, of `LEN` bits, stored in U8LEN bytes.
    ///
    /// # Panics
    /// if `LEN` > `U8LEN` * 8.
    fn default() -> Self {
        assert![U8LEN * 8 >= LEN];
        Self {
            array: Array::new([u8::default(); U8LEN]),
        }
    }
}

impl<S: Storage, const LEN: usize, const U8LEN: usize> CollectionAdt for BitArray<S, LEN, U8LEN> {
    type Element = bool;
    fn collection_is_empty(&self) -> bool {
        LEN == 0
    }
    fn collection_len(&self) -> usize {
        LEN
    }
    fn collection_byte_len(&self) -> usize {
        U8LEN
    }
}

// FIX: return ref
// impl<S: Storage, const LEN: usize, const U8LEN: usize> ArrayAdt for BitArray<S, LEN, U8LEN> {
//     fn array_get(&self, index: usize) -> Result<&<Self as CollectionAdt>::Element> {
//         if index >= LEN {
//             return Err(Error::IndexOutOfBounds(index));
//         }
//         let byte_index = index / 8;
//         let bit_within_byte_index = index % 8;
//         let bit_mask = 1 << bit_within_byte_index;
//         Ok(&(self.array[byte_index] & bit_mask != 0))
//     }
//     fn array_set(&mut self, index: usize, element: <Self as CollectionAdt>::Element) -> Result<()> {
//         self.set_bit(index, element)
//     }
// }
