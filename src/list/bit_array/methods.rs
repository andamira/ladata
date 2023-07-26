// ladata::list::bit_array::methods
//
//!
//

use crate::{
    error::{LadataError as Error, LadataResult as Result},
    list::{Array, BitArray},
    mem::Storage,
};

// ``
/// # Constructors
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> BitArray<S, BITLEN, BYTECAP> {
    /// Returns a `BitArray` having `BITLEN` bits set to the given `value`,
    /// with a capacity of `BYTECAP` bytes.
    ///
    /// More precisely every bit of `BYTECAP` will be set to `value`.
    ///
    /// # Errors
    /// Returns an error if `BITLEN > BYTECAP * 8`.
    #[inline]
    pub fn new(value: bool) -> Result<Self> {
        if BITLEN <= BYTECAP * 8 {
            Ok(Self {
                array: Array::new([value as u8 * u8::MAX; BYTECAP]),
            })
        } else {
            Err(Error::DimensionMismatch)
        }
    }
    /// Returns a `BitArray` having `BITLEN` bits set to the given `value`,
    /// with a capacity of `BYTECAP` bytes, unchecked version.
    ///
    /// More precisely every bit of `BYTECAP` will be set to `value`.
    ///
    /// # Panics
    /// Panics in debug if `BITLEN > BYTECAP * 8`.
    #[inline]
    pub fn new_unchecked(value: bool) -> Result<Self> {
        #[cfg(debug_assertions)]
        devela::iif![BITLEN > BYTECAP * 8; panic!("BITLEN > BYTECAP * 8")];

        Ok(Self {
            array: Array::new([value as u8; BYTECAP]),
        })
    }

    /// Returns a `BitArray` having `BITLEN` bits set to `0`, and a capacity of
    /// `BYTECAP` bytes.
    ///
    /// More precisely every bit of `BYTECAP` will be set to `0`.
    ///
    /// This is the same as calling [`new(false)`][Self#method.new].
    ///
    /// # Errors
    /// Returns an error if `BITLEN > BYTECAP * 8`.
    ///
    /// # Examples
    /// ```
    /// use {core::mem, ladata::all::{BitArray, Boxed}};
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let a = BitArray::<(), 8, 1>::new_zeroed()?;
    /// assert_eq![mem::size_of_val(&a), 1];
    /// assert![a.is_zeroed()];
    ///
    /// assert![BitArray::<(), 9, 1>::new_zeroed().is_err()];
    /// assert![BitArray::<(), 9, 2>::new_zeroed().is_ok()];
    ///
    /// let b = BitArray::<Boxed, 159, 20>::new_zeroed()?;
    /// assert_eq![mem::size_of_val(&b), mem::size_of::<usize>()];
    /// assert![b.is_zeroed()];
    ///
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn new_zeroed() -> Result<Self> {
        if BITLEN <= BYTECAP * 8 {
            Ok(Self {
                array: Array::new([0_u8; BYTECAP]),
            })
        } else {
            Err(Error::DimensionMismatch)
        }
    }

    /// Returns a `BitArray` having `BITLEN` bits set to `0`, and a capacity of
    /// `BYTECAP` bytes, unchecked version.
    ///
    /// More precisely every bit of `BYTECAP` will be set to `0`.
    ///
    /// # Panics
    /// Panics in debug if `BITLEN > BYTECAP * 8`.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BitArray;
    ///
    /// let a = BitArray::<(), 8, 1>::new_zeroed_unchecked();
    /// assert![a.is_zeroed()];
    /// ```
    #[inline]
    pub fn new_zeroed_unchecked() -> Self {
        #[cfg(debug_assertions)]
        devela::iif![BITLEN > BYTECAP * 8; panic!("BITLEN > BYTECAP * 8")];

        Self {
            array: Array::new([0_u8; BYTECAP]),
        }
    }

    /// Returns a `BitArray` having `BITLEN` bits set to `1`, and a capacity of
    /// `BYTECAP` bytes.
    ///
    /// More precisely every bit of `BYTECAP` will be set to `1`.
    ///
    /// # Errors
    /// Returns an error if `BITLEN > BYTECAP * 8`.
    ///
    /// # Examples
    /// ```
    /// use {core::mem, ladata::all::{BitArray, Boxed}};
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let a = BitArray::<(), 6, 1>::new_oned()?;
    /// assert![a.is_oned()];
    ///
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn new_oned() -> Result<Self> {
        if BITLEN <= BYTECAP * 8 {
            Ok(Self {
                array: Array::new([u8::MAX; BYTECAP]),
            })
        } else {
            Err(Error::DimensionMismatch)
        }
    }

    /// Returns a `BitArray` having `BITLEN` bits set to `1`, and a capacity of
    /// `BYTECAP` bytes, unchecked version.
    ///
    /// More precisely every bit of `BYTECAP` will be set to `1`.
    ///
    /// # Panics
    /// Panics in debug if `BITLEN > BYTECAP * 8`.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BitArray;
    ///
    /// let a = BitArray::<(), 8, 1>::new_oned_unchecked();
    /// assert![a.is_oned()];
    /// ```
    #[inline]
    pub fn new_oned_unchecked() -> Self {
        #[cfg(debug_assertions)]
        devela::iif![BITLEN > BYTECAP * 8; panic!("BITLEN > BYTECAP * 8")];

        Self {
            array: Array::new([u8::MAX; BYTECAP]),
        }
    }

    /* reconstructors */

    /// Returns itself with a new length, which must fit the same byte capacity.
    ///
    /// The inner byte array is moved, not copied.
    ///
    /// # Errors
    /// Returns an error if `NEW_BITLEN > BYTECAP * 8`.
    pub fn into_resized<const NEW_BITLEN: usize>(self) -> Result<BitArray<S, NEW_BITLEN, BYTECAP>> {
        if NEW_BITLEN >= BYTECAP * 8 {
            Ok(BitArray::<S, NEW_BITLEN, BYTECAP> { array: self.array })
        } else {
            Err(Error::DimensionMismatch)
        }
    }

    /* deconstructors */

    /// Returns the inner byte `Array` that contains the entire bit array,
    /// including its unused capacity.
    pub fn into_byte_array(self) -> Array<u8, S, BYTECAP> {
        self.array
    }

    /// Returns a byte slice containing the entire bit array,
    /// including its unused capacity.
    pub fn as_byte_slice(&self) -> &[u8] {
        self.array.as_slice()
    }

    /// Returns an exclusive byte slice containing the entire bit array,
    /// including its unused capacity.
    pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
        self.array.as_mut_slice()
    }
}

impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> BitArray<S, BITLEN, BYTECAP>
where
    S::Stored<[u8; BYTECAP]>: Clone,
{
    /// Returns a clone of itself with a new length, which must fit the same byte capacity.
    ///
    /// # Errors
    /// Returns an error if `NEW_BITLEN > BYTECAP * 8`.
    pub fn resized<const NEW_BITLEN: usize>(&self) -> Result<BitArray<S, NEW_BITLEN, BYTECAP>> {
        if NEW_BITLEN >= BYTECAP * 8 {
            Ok(BitArray::<S, NEW_BITLEN, BYTECAP> {
                array: self.array.clone(),
            })
        } else {
            Err(Error::DimensionMismatch)
        }
    }
}

impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> BitArray<S, BITLEN, BYTECAP> {
    /// Returns the total number of bits in the bit array.
    #[inline]
    pub const fn len(&self) -> usize {
        BITLEN
    }

    /// Returns `true` if the bit array has a length of 0.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        BITLEN == 0
    }

    /// Returns `true` if the bit array has no unused capacity.
    ///
    /// # Panics
    /// Will panic if `BYTECAP > usize::MAX / 8`.
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.unused_capacity() == 0
    }

    /// Returns the total capacity in bits.
    ///
    /// # Panics
    /// Will panic if `BYTECAP > usize::MAX / 8`.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BitArray;
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let a = BitArray::<(), 7, 2>::new_zeroed()?;
    /// assert_eq![16, a.capacity()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        BYTECAP * 8
    }

    /// Returns the unused capacity in bits.
    ///
    /// # Panics
    /// Will panic if `BYTECAP > usize::MAX / 8`.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BitArray;
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// # Ok(()) }
    /// ```
    #[inline]
    pub const fn unused_capacity(&self) -> usize {
        self.capacity() - self.len()
    }
}

/// # Single bit operations
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> BitArray<S, BITLEN, BYTECAP> {
    /* get */

    /// Gets the bit's value at `index`.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BitArray;
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let mut ba = BitArray::<(), 3, 1>::new_zeroed()?;
    /// assert_eq![ba.get_bit(2), Ok(false)];
    ///
    /// ba.set_bit(2, true)?;
    /// assert_eq![ba.get_bit(2), Ok(true)];
    ///
    /// assert![ba.get_bit(3).is_err()];
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn get_bit(&self, index: usize) -> Result<bool> {
        if index >= BITLEN {
            return Err(Error::IndexOutOfBounds(index));
        }
        let byte_index = index / 8;
        let bit_index_in_byte = index % 8;
        let bit_mask = 1 << bit_index_in_byte;
        Ok(self.array[byte_index] & bit_mask != 0)
    }
    /// Gets the bit's value at `index`, unchecked.
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    ///
    /// # Examples
    /// ```
    /// use ladata::all::BitArray;
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let mut ba = BitArray::<(), 3, 1>::new_zeroed()?;
    /// assert_eq![ba.get_bit_unchecked(2), false];
    ///
    /// ba.set_bit(2, true)?;
    /// assert_eq![ba.get_bit_unchecked(2), true];
    ///
    /// # Ok(()) }
    /// ```
    ///
    /// ```should_panic
    /// use ladata::all::BitArray;
    /// # fn main() -> ladata::all::LadataResult<()> {
    ///
    /// let mut ba = BitArray::<(), 3, 1>::new_zeroed()?;
    /// let _ = ba.get_bit_unchecked(3); // panics in debug
    /// let _ = ba.get_bit_unchecked(8); // panics in release
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn get_bit_unchecked(&self, index: usize) -> bool {
        #[cfg(debug_assertions)]
        devela::iif![index >= BITLEN; panic!("index >= BITLEN")];

        let byte_index = index / 8;
        let bit_index_in_byte = index % 8;
        let bit_mask = 1 << bit_index_in_byte;
        self.array[byte_index] & bit_mask != 0
    }

    /* set */

    /// Sets the bit at `index` to `value`.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    #[inline]
    pub fn set_bit(&mut self, index: usize, value: bool) -> Result<()> {
        if index >= BITLEN {
            return Err(Error::IndexOutOfBounds(index));
        }
        let byte_index = index / 8;
        let bit_index_in_byte = index % 8;
        let bit_mask = 1 << bit_index_in_byte;
        let value = (value as u8) << bit_index_in_byte;
        // clear the bit at the position and then set it with the new value
        self.array[byte_index] = (self.array[byte_index] & !bit_mask) | value;
        Ok(())
    }
    /// Sets the bit at `index` to `value`.
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    #[inline]
    pub fn set_bit_unchecked(&mut self, index: usize, value: bool) {
        #[cfg(debug_assertions)]
        devela::iif![index >= BITLEN; panic!("index >= BITLEN")];

        let byte_index = index / 8;
        let bit_index_in_byte = index % 8;
        let bit_mask = 1 << bit_index_in_byte;
        let value = (value as u8) << bit_index_in_byte;
        // clear the bit at the position and then set it with the new value
        self.array[byte_index] = (self.array[byte_index] & !bit_mask) | value;
    }

    /// Sets the bit at `index` to `value`, returning its previous value.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    #[inline]
    pub fn get_set_bit(&mut self, index: usize, value: bool) -> Result<bool> {
        if index >= BITLEN {
            return Err(Error::IndexOutOfBounds(index));
        }
        let byte_index = index / 8;
        let bit_index_in_byte = index % 8;
        let bit_mask = 1 << bit_index_in_byte;
        let previous = self.array[byte_index] & bit_mask != 0;
        let value = (value as u8) << bit_index_in_byte;
        // clear the bit at the position and then set it with the new value
        self.array[byte_index] = (self.array[byte_index] & !bit_mask) | value;
        Ok(previous)
    }
    /// Sets the bit at `index` to `value`, returning its previous value.
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    #[inline]
    pub fn get_set_bit_unchecked(&mut self, index: usize, value: bool) -> bool {
        #[cfg(debug_assertions)]
        devela::iif![index >= BITLEN; panic!("index >= BITLEN")];

        let byte_index = index / 8;
        let bit_index_in_byte = index % 8;
        let bit_mask = 1 << bit_index_in_byte;
        let previous = self.array[byte_index] & bit_mask != 0;
        let value = (value as u8) << bit_index_in_byte;
        // clear the bit at the position and then set it with the new value
        self.array[byte_index] = (self.array[byte_index] & !bit_mask) | value;
        previous
    }

    /* set, convenience */

    /// Sets the bit at the given `index` to `1`.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    #[inline]
    pub fn set_one(&mut self, index: usize) -> Result<()> {
        self.set_bit(index, true)
    }
    /// Sets the bit at the given `index` to `1`, unchecked.
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    #[inline]
    pub fn set_one_unchecked(&mut self, index: usize) {
        self.set_bit_unchecked(index, true)
    }
    /// Sets the bit at the given `index` to `1`, returning its previous value.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    #[inline]
    pub fn get_set_one(&mut self, index: usize) -> Result<bool> {
        self.get_set_bit(index, true)
    }
    /// Sets the bit at the given `index` to `1`, unchecked.
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    #[inline]
    pub fn get_set_one_unchecked(&mut self, index: usize) -> bool {
        self.get_set_bit_unchecked(index, true)
    }

    /// Sets the bit at the given `index` to `0`.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    #[inline]
    pub fn set_zero(&mut self, index: usize) -> Result<()> {
        self.set_bit(index, false)
    }
    /// Sets the bit at the given `index` to `0`, unchecked.
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    #[inline]
    pub fn set_zero_unchecked(&mut self, index: usize) {
        self.set_bit_unchecked(index, false)
    }
    /// Sets the bit at the given `index` to `0`, returning its previous value.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    #[inline]
    pub fn get_set_zero(&mut self, index: usize) -> Result<bool> {
        self.get_set_bit(index, false)
    }
    /// Sets the bit at the given `index` to `0`, unchecked.
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    #[inline]
    pub fn get_set_zero_unchecked(&mut self, index: usize) -> bool {
        self.get_set_bit_unchecked(index, false)
    }

    /* toggle */

    /// Toggles the bit at the given `index`.
    ///
    /// Returns the previous bit value.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    #[inline]
    pub fn toggle(&mut self, index: usize) -> Result<()> {
        let bit = self.get_bit(index)?;
        self.set_bit(index, !bit)
    }
    /// Toggles the bit at the given `index`.
    ///
    /// Returns the previous bit value
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    #[inline]
    pub fn toggle_unchecked(&mut self, index: usize) {
        let bit = self.get_bit_unchecked(index);
        self.set_bit_unchecked(index, !bit)
    }

    /// Toggles the bit at the given `index` returning the previous value.
    ///
    /// # Errors
    /// Returns an error if `index >= BITLEN`.
    #[inline]
    pub fn get_toggle(&mut self, index: usize) -> Result<bool> {
        let bit = self.get_bit(index)?;
        self.get_set_bit(index, !bit)
    }
    /// Toggles the bit at the given `index` returning the previous value.
    ///
    /// # Panics
    /// Panics in debug if `index >= BITLEN` and in release if `index >= BYTECAP * 8`.
    #[inline]
    pub fn get_toggle_unchecked(&mut self, index: usize) -> bool {
        let bit = self.get_bit_unchecked(index);
        self.get_set_bit_unchecked(index, !bit)
    }
}

/// # set methods
impl<S: Storage, const BITLEN: usize, const BYTECAP: usize> BitArray<S, BITLEN, BYTECAP> {
    /// Returns `true` if all bits with `index < BITLEN` are set to `0`.
    pub fn is_zeroed(&self) -> bool {
        let full_bytes = BITLEN / 8;
        let remaining_bits = BITLEN % 8;

        for i in 0..full_bytes {
            if self.array[i] != 0 {
                return false;
            }
        }

        if remaining_bits != 0 {
            let mask = (1 << remaining_bits) - 1;
            if self.array[full_bytes] & mask != 0 {
                return false;
            }
        }

        true
    }

    /// Returns `true` if all bits with `index < BITLEN` are set to `1`.
    pub fn is_oned(&self) -> bool {
        let full_bytes = BITLEN / 8;
        let remaining_bits = BITLEN % 8;

        for i in 0..full_bytes {
            if self.array[i] != u8::MAX {
                return false;
            }
        }

        if remaining_bits != 0 {
            let mask = (1 << remaining_bits) - 1;
            if self.array[full_bytes] & mask != mask {
                return false;
            }
        }

        true
    }

    // /// Computes the intersection of this bit array with another.
    // pub fn intersect<S2: Storage>(&mut self, other: &BitArray<S2, BITLEN, BYTECAP>) {
    //     for i in 0..BYTECAP {
    //         self.array[i] &= other.array[i];
    //     }
    // }
}
