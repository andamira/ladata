// ladata::list::bit_array::methods
//
//!
//

use crate::{
    list::{Array, BitArray},
    mem::Storage,
};
use core::fmt;

// Default
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> Default
    for BitArray<S, BITLEN, BYTECAP>
{
    /// Returns an empty bit array, of `BITLEN` bits, stored in `BYTECAP` bytes.
    ///
    /// # Panics
    /// Panics in debug if `BITLEN > BYTECAP * 8`.
    fn default() -> Self {
        #[cfg(debug_assertions)]
        devela::iif![BITLEN > BYTECAP * 8; panic!("BITLEN > BYTECAP * 8")];

        Self {
            array: Array::new([u8::default(); BYTECAP]),
        }
    }
}

// Clone
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> Clone for BitArray<S, BITLEN, BYTECAP>
where
    S::Stored<[u8; BYTECAP]>: Clone,
{
    fn clone(&self) -> Self {
        BitArray {
            array: self.array.clone(),
        }
    }
}

// Copy
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> Copy for BitArray<S, BITLEN, BYTECAP> where
    S::Stored<[u8; BYTECAP]>: Copy
{
}

// Debug
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> fmt::Debug
    for BitArray<S, BITLEN, BYTECAP>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BitArray<{}, BITLEN: {}, BYTECAP: {}> {{ ",
            S::name(),
            BITLEN,
            BYTECAP
        )?;
        write!(f, "{}", self)?;
        write!(f, " }}")?;
        Ok(())
    }
}

// Display
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> fmt::Display
    for BitArray<S, BITLEN, BYTECAP>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_bytes = BITLEN / 8;
        let remaining_bits = BITLEN % 8;

        // Print remaining bits in the last byte, if any
        if remaining_bits > 0 {
            let last_byte = self.array[full_bytes];
            for i in (0..remaining_bits).rev() {
                write!(f, "{}", (last_byte >> i) & 1)?;
            }
            if full_bytes > 0 {
                write!(f, "_")?;
            }
        }

        // Print full bytes
        for (i, byte) in self.array[0..full_bytes].iter().rev().enumerate() {
            for i in (0..8).rev() {
                write!(f, "{}", (byte >> i) & 1)?;
            }
            if i < full_bytes - 1 {
                write!(f, "_")?;
            }
        }

        Ok(())
    }
}

// LowerHex
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> fmt::LowerHex
    for BitArray<S, BITLEN, BYTECAP>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_bytes = BITLEN / 8;
        let remaining_bits = BITLEN % 8;

        if remaining_bits > 0 {
            let mask = (1 << remaining_bits) - 1;
            let digit = self.array[full_bytes] & mask;
            write!(f, "{:01x}", digit)?;
        }

        for byte in self.array[..full_bytes].iter().rev() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

// UpperHex
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> fmt::UpperHex
    for BitArray<S, BITLEN, BYTECAP>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_bytes = BITLEN / 8;
        let remaining_bits = BITLEN % 8;

        if remaining_bits > 0 {
            let mask = (1 << remaining_bits) - 1;
            let digit = self.array[full_bytes] & mask;
            write!(f, "{:01X}", digit)?;
        }

        for byte in self.array[..full_bytes].iter().rev() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

// Octal
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> fmt::Octal
    for BitArray<S, BITLEN, BYTECAP>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut decimal_val: usize = 0;

        for i in 0..BITLEN {
            let byte_index = i / 8;
            let bit_offset = i % 8;
            let bit = (self.array[byte_index] >> bit_offset) & 1;
            decimal_val |= (bit as usize) << i;
        }

        write!(f, "{:o}", decimal_val)
    }
}

// Binary
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> fmt::Binary
    for BitArray<S, BITLEN, BYTECAP>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_bytes = BITLEN / 8;
        let remaining_bits = BITLEN % 8;

        if remaining_bits > 0 {
            for i in (0..remaining_bits).rev() {
                write!(f, "{}", (self.array[full_bytes] >> i) & 1)?;
            }
        }

        for byte in self.array[..full_bytes].iter().rev() {
            for i in (0..8).rev() {
                write!(f, "{}", (byte >> i) & 1)?;
            }
        }
        Ok(())
    }
}
