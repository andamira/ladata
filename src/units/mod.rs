// ladata::builder
//
//! Types are built here and reexported from `lib.rs`.
/// Units of data, whether just the data, the type, or both.
/// # Unitary types: Data `Type*`|`Cell*`|`Bare*` and [`()`]
///
/// The concrete implementations of **unitary types** observes the following naming schemes:
/// ```txt
///                                                   Legend      |     Sizes
/// *)    Data <Type> <Size> [Copy] [With]        --------------- | ------------
/// *)    Data <Cell> <Size> [Copy] [With]        <> : required   |    8b =   1B
/// *)    Data <Bare> <Size> <Copy>               [] : optional   |   16b =   2B
///                                                | : either or  |   32b =   4B
///                                                = : alias      |   64b =   8B
///                                                               |  128b =  16B
///                                                               |  256b =  32B
///                                                               |  512b =  64B
///                                                               | 1024b = 128B
/// ```
/// 1. `Data`: the *pivotal core*.
/// 2. `<Type|Cell|Bare>`: encapsulates 1) only the data type,
///    2) both the data type and the data, or 3) only the data.
/// 3. `<Size>`: confines the maximum size of the represented data,
///    limiting the number of types and sizes of data available.
/// 4. `[Copy]`: indicates that all the included data types are `Copy`.
/// 5. `[With]`: allows to embed a custom implementation of a data type or cell
///     in its `With` variant.
///
/// ### `<Type|Cell|Bare>`
///
/// - `Type` indicates the categorization of the data type information.
/// All *`DataType*`s* must implement the [`DataTypes`] trait, and
/// *`DataType*Copy*`* *types* must additionally implement the
/// [`DataTypesCopy`] trait.
///
/// - `Cell` indicates the encapsulation of the data and the type information.
/// All *`DataCell*`s* must implement the [`DataCells`] trait, and
/// *`DataCell*Copy*`* *cells* must additionally implement the
/// [`DataCellsCopy`] trait.
///
/// - `Bare` indicates the encapsulation of data without the type information.
/// All *`DataBare`s* implements the (marker) [`DataBares`] trait.
///
/// ### `[Copy]`
///
/// *`Copy`* indicates that the data represented by the *type*,
/// (and|or encapsulated by the *cell*) is [`Copy`].
///
/// Only types that can be copied with simple shallow bit-for-bit copy,
/// leaving the source initialized, can be `Copy`.
/// This leaves out types referencing the heap and other resources.
///
/// ### *`<Size>`*
///
/// Indicates the specific size of the data representation in memory, in bits.
///
/// Specifically tells the maximum size of the data. Smaller-sized variants
/// are also available in bigger-sized cells. For example the `U16(u16)` variant
/// is present in `DataCell16bit` and `DataCell32bit` but not in `DataCell8bit`.
///
/// Types can be found classified by size in the [`size`][crate::size] module.
///
/// ### `[With]`
///
/// **`DataType*With`** enums can be extended generically by storing a type
/// implementing [`DataTypes`] in its `With` variant (or [`DataTypesCopy`]
/// in the case of `DataType*CopyWith`.
///
/// In the same way, **`DataCell*With`** enums can be extended generically by
/// storing a type implementing [`DataCells`] in its `With` variant
/// (or [`DataCellsCopy`] in the case of `DataCell*CopyWith`.
///
/// Internally, all non-`With` versions are convenient type aliases to the
/// corresponding `With` version (having the same size and `Copy` semantics),
/// using the zero-sized [`()`] unit type. E.g.:
/// [`DataType256bit`][crate::all::DataType256bit]
///
/// [`DataTypes`]: crate::DataTypes
/// [`DataTypesCopy`]: crate::DataTypesCopy
/// [`DataCells`]: crate::DataCells
/// [`DataCellsCopy`]: crate::DataCellsCopy
/// [`DataBares`]: crate::DataBares
/// [`DataBaresCopy`]: crate::DataBaresCopy

/// Data *Bares* (just the unsafe bare data).
pub mod bares {
    crate::reexport![mod_bares, crate::built; all_sizes];
}

/// Data *Cells* (the encapsulation of *data* plus *type*).
pub mod cells {
    crate::reexport![mod_cells, crate::built; all_sizes];
}

/// Data *Types* (just the *type* of the data).
pub mod types {
    crate::reexport![mod_types, crate::built; all_sizes];
}
#[doc(inline)]
pub use {bares::*, cells::*, types::*};

pub(crate) mod built;
pub(crate) mod macros;
#[cfg(test)]
mod tests;
