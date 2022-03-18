// ladata::traits
//
//!
//

use core::fmt::Debug;

// TRAITS
// -------------------------------------------------------------------------
// - DataTypes
// - DataTypesCopy
// - DataUnits
// - DataUnitsCopy
// - DataLones

/// Common trait for *data types*.
///
/// Allows extending `DataType*`**`With`** versions with custom *types*.
///
/// # See also
/// - [`DataTypesCopy`]
/// - [`DataUnitsCopy`]
/// - [`DataUnits`]
pub trait DataTypes: Copy + Debug {
    /// Returns the alignment of the data represented by the current type.
    fn data_align(&self) -> usize;

    /// Returns the size of the data represented by this type.
    fn data_size(&self) -> usize;

    // /// Returns the `TypeId` of the data represented by this type.
    // fn type_id(&self) -> TypeId;

    /// Returns true if the data represented by this type is [`Copy`].
    fn is_copy(&self) -> bool;
}

/// Common (marker) trait for `Copy` *data types*.
///
/// Allows extending `DataType*Copy`**`With`** versions with custom *types*.
///
/// # Coherence
///
/// The `DataTypes::`[`is_copy`][DataTypes#method.is_copy]
/// super-trait method should probably return `true` as well.
///
/// # See also
/// - [`DataTypes`]
/// - [`DataUnits`]
/// - [`DataUnitsCopy`]
pub trait DataTypesCopy: DataTypes {}

/// Common trait for *data cells*.
///
/// Allows extending `DataUnit*`**`With`** versions.
///
/// See also:
/// - [`DataUnitsCopy`]
/// - [`DataTypesCopy`]
/// - [`DataTypes`]
pub trait DataUnits: Debug {
    /// Whether the data type in the current variant is [`Copy`].
    fn is_copy(&self) -> bool;
}

/// Common (marker) trait for `Copy` *data cells*.
///
/// Allows extending `DataUnit*Copy`**`With`** versions.
///
/// # Coherence
///
/// The `DataUnits::`[`is_copy`][DataUnits#method.is_copy]
/// super-trait method should probably return `true` as well.
///
/// # See also
/// - [`DataUnits`]
/// - [`DataTypes`]
/// - [`DataTypesCopy`]
pub trait DataUnitsCopy: DataUnits + Copy {}

/// Common trait for unsafe data cells implemented with `union`.
///
/// # Safety
/// TODO
///
pub unsafe trait DataLones {}
