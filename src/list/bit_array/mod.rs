// ladata::list::bit_array
//
//! Bit arrays.
//!
//! <https://en.wikipedia.org/wiki/Bit_array>
//

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

use crate::{list::Array, mem::Storage, misc::DataCollection};

mod impls;
mod methods;

/// An array of bits, backed by an [`Array`] of bytes.
pub struct BitArray<S: Storage, const BITLEN: usize, const BYTECAP: usize> {
    array: Array<u8, S, BYTECAP>,
}

/// An array of 8 bits.
pub type BitArray8<S> = BitArray<S, 8, 1>;
/// An array of 16 bits.
pub type BitArray16<S> = BitArray<S, 16, 2>;
/// An array of 32 bits.
pub type BitArray32<S> = BitArray<S, 32, 4>;
/// An array of 64 bits.
pub type BitArray64<S> = BitArray<S, 64, 8>;
/// An array of 128 bits.
pub type BitArray128<S> = BitArray<S, 128, 16>;

/// A [`BitArray`] stored in the stack.
pub type DirectBitArray<const BITLEN: usize, const BYTECAP: usize> = BitArray<(), BITLEN, BYTECAP>;

/// An array of 8 bits stored in the stack.
pub type DirectBitArray8 = BitArray<(), 8, 1>;
/// An array of 16 bits stored in the stack.
pub type DirectBitArray16 = BitArray<(), 16, 2>;
/// An array of 32 bits stored in the stack.
pub type DirectBitArray32 = BitArray<(), 32, 4>;
/// An array of 64 bits stored in the stack.
pub type DirectBitArray64 = BitArray<(), 64, 8>;
/// An array of 128 bits stored in the stack.
pub type DirectBitArray128 = BitArray<(), 128, 16>;

/// A [`BitArray`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub type BoxedBitArray<const BITLEN: usize, const BYTECAP: usize> =
    BitArray<Boxed, BITLEN, BYTECAP>;

/// An array of 8 bits stored in the heap.
pub type BoxedBitArray8 = BitArray<Boxed, 8, 1>;
/// An array of 16 bits stored in the heap.
pub type BoxedBitArray16 = BitArray<Boxed, 16, 2>;
/// An array of 32 bits stored in the heap.
pub type BoxedBitArray32 = BitArray<Boxed, 32, 4>;
/// An array of 64 bits stored in the heap.
pub type BoxedBitArray64 = BitArray<Boxed, 64, 8>;
/// An array of 128 bits stored in the heap.
pub type BoxedBitArray128 = BitArray<Boxed, 128, 16>;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::{
        BoxedBitArray, BoxedBitArray128, BoxedBitArray16, BoxedBitArray32, BoxedBitArray64,
        BoxedBitArray8,
    };

    #[doc(inline)]
    pub use super::{
        BitArray, DirectBitArray, DirectBitArray128, DirectBitArray16, DirectBitArray32,
        DirectBitArray64, DirectBitArray8,
    };
}

/* traits */

// TODO: RETHINK
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> DataCollection
    for BitArray<S, BITLEN, BYTECAP>
{
    type Element = bool;
    fn collection_is_empty(&self) -> Option<bool> {
        None
    }
    fn collection_is_full(&self) -> Option<bool> {
        None
    }
    fn collection_capacity(&self) -> usize {
        BITLEN
    }
    fn collection_len(&self) -> usize {
        BITLEN
    }
}

// FIX: return ref
// impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> DataArray for BitArray<S, BITLEN, BYTECAP> {
//     fn array_get(&self, index: usize) -> Result<&<Self as DataCollection>::Element> {
//         if index >= BITLEN {
//             return Err(Error::IndexOutOfBounds(index));
//         }
//         let byte_index = index / 8;
//         let bit_within_byte_index = index % 8;
//         let bit_mask = 1 << bit_within_byte_index;
//         Ok(&(self.array[byte_index] & bit_mask != 0))
//     }
//     fn array_set(&mut self, index: usize, element: <Self as DataCollection>::Element) -> Result<()> {
//         self.set_bit(index, element)
//     }
// }
