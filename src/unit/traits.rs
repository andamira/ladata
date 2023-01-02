// ladata::unit::traits
//
//! The traits for unitary data.
//
// - DataTypes
// - DataTypesCopy
// - DataCells
// - DataCellsCopy
// - DataBares

use core::fmt::Debug;

/// Common trait for *data types*.
///
/// Allows extending `DataType*`**`With`** versions with custom *types*.
///
/// # See also
/// - [`DataTypesCopy`]
/// - [`DataCellsCopy`]
/// - [`DataCells`]
pub trait DataTypes: Copy + Debug {
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
/// The `DataTypes::`[`is_copy`][DataTypes#method.is_copy]
/// super-trait method should probably return `true` as well.
///
/// # See also
/// - [`DataTypes`]
/// - [`DataCells`]
/// - [`DataCellsCopy`]
pub trait DataTypesCopy: DataTypes {}

/// Common trait for *data cells*.
///
/// Allows extending `DataCell*`**`With`** versions.
///
/// See also:
/// - [`DataCellsCopy`]
/// - [`DataTypesCopy`]
/// - [`DataTypes`]
pub trait DataCells: Debug {
    /// Whether the data type in the current variant is [`Copy`].
    fn is_copy(&self) -> bool;
}

/// Common (marker) trait for `Copy` *data cells*.
///
/// Allows extending `DataCell*Copy`**`With`** versions.
///
/// # Coherence
///
/// The `DataCells::`[`is_copy`][DataCells#method.is_copy]
/// super-trait method should probably return `true` as well.
///
/// # See also
/// - [`DataCells`]
/// - [`DataTypes`]
/// - [`DataTypesCopy`]
pub trait DataCellsCopy: DataCells + Copy {}

/// Common trait for *unsafe data cells*.
///
/// # Safety
/// TODO
///
pub unsafe trait DataBares {}

/// Comon (marker) trait for *unsafe* `Copy` *data cells*.
///
/// # Safety
/// TODO
///
pub unsafe trait DataBaresCopy: DataBares + Copy {}
