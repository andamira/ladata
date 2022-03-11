// ladata::traits
//
//!
//

use core::fmt::Debug;

// TRAITS
// -------------------------------------------------------------------------
// - DataTypes
// - DataTypesCopy
// - DataCells
// - DataCellsCopy
// - DataUnsafeCells

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

    // /// Returns the `TypeId` of the data represented by this type.
    // fn type_id(&self) -> TypeId;

    /// Returns true if the data represented by this type is [`Copy`].
    fn is_copy(&self) -> bool;
}

/// Common (marker) trait for *data types* representing `Copy` data.
///
/// Allows extending `DataType*Copy`**`With`** versions with custom *types*.
///
/// # Coherence
///
/// The `DataTypes::`[`is_copy`][DataTypes#method.is_copy]
/// super-trait method should probably return `true` too.
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
/// super-trait method should probably return `true` too.
///
/// # See also
/// - [`DataCells`]
/// - [`DataTypes`]
/// - [`DataTypesCopy`]
pub trait DataCellsCopy: DataCells + Copy {}

/// Common trait for unsafe data cells implemented with `union`.
///
/// # Safety
/// TODO
///
pub unsafe trait DataUnsafeCells {
}
