// ladata::unit::traits
//
//! The traits for unitary data.
//
// - DataType
// - DataTypeCopy
// - DataUnit
// - DataUnitCopy
// - RawData

use core::fmt::Debug;

/// Common trait for *data types*.
///
/// Allows extending `DataType*`**`With`** versions with custom *types*.
///
/// # See also
/// - [`DataTypeCopy`]
/// - [`DataUnitCopy`]
/// - [`DataUnit`]
pub trait DataType: Copy + Debug {
    /// Returns the alignment of the data represented by the current type.
    fn data_align(&self) -> usize;

    /// Returns the size of the data represented by this type.
    fn data_size(&self) -> usize;

    /// Returns true if the data represented by this type is [`Copy`].
    fn is_copy(&self) -> bool;
}

/// Common (marker) trait for `Copy` *data types*.
///
/// Allows extending `DataType*Copy`**`With`** versions with custom *types*.
///
/// # Coherence
///
/// The `DataType::`[`is_copy`][DataType#method.is_copy]
/// super-trait method should probably return `true` as well.
///
/// # See also
/// - [`DataType`]
/// - [`DataUnit`]
/// - [`DataUnitCopy`]
pub trait DataTypeCopy: DataType {}

/// Common trait for *data units*.
///
/// Allows extending `DataUnit*`**`With`** versions.
///
/// See also:
/// - [`DataUnitCopy`]
/// - [`DataTypeCopy`]
/// - [`DataType`]
pub trait DataUnit: Debug {
    /// Whether the data type in the current variant is [`Copy`].
    fn is_copy(&self) -> bool;
}

/// Common (marker) trait for `Copy` *data units*.
///
/// Allows extending `DataUnit*Copy`**`With`** versions.
///
/// # Coherence
///
/// The `DataUnit::`[`is_copy`][DataUnit#method.is_copy]
/// super-trait method should probably return `true` as well.
///
/// # See also
/// - [`DataUnit`]
/// - [`DataType`]
/// - [`DataTypeCopy`]
pub trait DataUnitCopy: DataUnit + Copy {}

/// Common trait for *unsafe data units*.
///
/// # Safety
/// TODO
///
#[cfg(not(feature = "safe"))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
pub unsafe trait RawData {}

/// Comon (marker) trait for *unsafe* `Copy` *data units*.
///
/// # Safety
/// TODO
///
#[cfg(not(feature = "safe"))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
pub unsafe trait RawDataCopy: RawData + Copy {}
