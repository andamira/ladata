// ladata::traits
//
//!
//

use core::fmt::Debug;

// TRAITS
// -------------------------------------------------------------------------
// - DataType
// - DataTypeCopy
// - DataCell
// - DataCellCopy
// - DataBare

/// Common trait for *data types*.
///
/// Allows extending `DataType*`**`With`** versions with custom *types*.
///
/// # See also
/// - [`DataTypeCopy`]
/// - [`DataCellCopy`]
/// - [`DataCell`]
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
/// super-trait method should return `true` as well.
///
/// # See also
/// - [`DataType`]
/// - [`DataCell`]
/// - [`DataCellCopy`]
pub trait DataTypeCopy: DataType {}

/// Common trait for *data cells*.
///
/// Allows extending `DataCell*`**`With`** versions.
///
/// See also:
/// - [`DataCellCopy`]
/// - [`DataTypeCopy`]
/// - [`DataType`]
pub trait DataCell: Debug {
    /// Whether the data type in the current variant is [`Copy`].
    fn is_copy(&self) -> bool;
}

/// Common (marker) trait for `Copy` *data cells*.
///
/// Allows extending `DataCell*Copy`**`With`** versions.
///
/// # Coherence
///
/// The `DataCell::`[`is_copy`][DataCell#method.is_copy]
/// super-trait method should return `true` as well.
///
/// # See also
/// - [`DataCell`]
/// - [`DataType`]
/// - [`DataTypeCopy`]
pub trait DataCellCopy: DataCell + Copy {}

/// Common trait for *unsafe data cells*.
///
/// # Safety
/// TODO
///
pub unsafe trait DataBare {}

/// Comon (marker) trait for *unsafe* `Copy` *data cells*.
///
/// # Safety
/// TODO
///
pub unsafe trait DataBareCopy: DataBare + Copy {}
