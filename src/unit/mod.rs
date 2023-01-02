// ladata::unit
//
//! Unitary data.
//!
//! The fundamental unitary abstractions are:
//! - `DataCell…` enums that contains both the data and the type information.
//! - `DataType…` enums that only contains type information, not the data itself.
//! - `DataBare…` unions that only contains the data, not the type of the data.
//!
//! They implement the [`DataCells`], [`DataTypes`] and [`DataBares`] traits.
//!
//! Each one has many concrete implementations differentiated by:
//! - the maximum `size` of the represented data.
//! - whether all the included types of data are `Copy`.
//! - the possibility of embedding a custom type in the `With` variant./!
//!
//! They observe the following naming scheme:
//! ```txt
//!                                         Legend      |     Sizes
//! *) Type <Size> [Copy] [With]        --------------- | ------------
//! *) Cell <Size> [Copy] [With]        <> : required   |    8b =   1B
//! *) Bare <Size> <Copy>               [] : optional   |   16b =   2B
//!                                      | : either or  |   32b =   4B
//!                                      = : alias      |   64b =   8B
//!                                                     |  128b =  16B
//!                                                     |  256b =  32B
//!                                                     |  512b =  64B
//!                                                     | 1024b = 128B
//! ```
//! 1. `<Type|Cell|Bare>`: encapsulates 1) only the data type,
//!    2) both the data type and the data, or 3) only the data, respectively.
//! 2. `<Size>`: confines the maximum size of the represented data,
//!    limiting the number of types and sizes of data available.
//! 3. `[Copy]`: indicates that all the included data types are `Copy`.
//! 4. `[With]`: allows to embed a custom implementation of a data type or cell
//!     in its `With` variant.
//!
//! - `Type` indicates the categorization of the data type information.
//! All *`DataType*`s* must implement the [`DataTypes`] trait, and
//! *`DataType*Copy*`* *types* must additionally implement the
//! [`DataTypesCopy`] trait.
//!
//! - `Cell` indicates the encapsulation of the data and the type information.
//! All *`DataCell*`s* must implement the [`DataCells`] trait, and
//! *`DataCell*Copy*`* *cells* must additionally implement the
//! [`DataCellsCopy`] trait.
//!
//! - `Bare` indicates the encapsulation of data without the type information.
//! All *`DataBare`s* implements the (marker) [`DataBares`] trait.
//!
//! ### `[Copy]`
//!
//! *`Copy`* indicates that the data represented by the *type*,
//! (and|or encapsulated by the *cell*) is [`Copy`].
//!
//! Only types that can be copied with simple shallow bit-for-bit copy,
//! leaving the source initialized, can be `Copy`.
//! This leaves out types referencing the heap and other resources.
//!
//! ### *`<Size>`*
//!
//! Indicates the specific size of the data representation in memory, in bits.
//!
//! Specifically tells the maximum size of the data. Smaller-sized variants
//! are also available in bigger-sized cells. For example the `U16(u16)` variant
//! is present in `DataCell16bit` and `DataCell32bit` but not in `DataCell8bit`.
//!
//! Types can be found classified by size in the [`size`] module.
//!
//! ### `[With]`
//!
//! **`DataType*With`** enums can be extended generically by storing a type
//! implementing [`DataTypes`] in its `With` variant (or [`DataTypesCopy`]
//! in the case of `DataType*CopyWith`.
//!
//! In the same way, **`DataCell*With`** enums can be extended generically by
//! storing a type implementing [`DataCells`] in its `With` variant
//! (or [`DataCellsCopy`] in the case of `DataCell*CopyWith`.
//!
//! Internally, all non-`With` versions are convenient type aliases to the
//! corresponding `With` version (having the same size and `Copy` semantics),
//! using the zero-sized [`()`] unit type. E.g.:
//! [`DataType256bit`][crate::all::DataType256bit]
//!
//! Note that `DataBare…`s are more space-efficient than `DataCell…`s but
//! not as convenient to use and also unsafe to read, because they're unions.
//! They are mostly intended to be used from collections that can store their
//! corresponding `DataType…`s separately. At the moment they only support
//! `Copy` types, and they can't host any custom types (lacking a `With` field).
//!
//! ## Custom unit data
//!
//! ```
//! use ladata::unit::*;
//!
//! let arr = [
//!     DataCell32bit::F32(3.14),
//!     DataCell32bit::Char('π'),
//! ];
//! for c in arr {
//!    match c {
//!         DataCell32bit::F32(f) => println!("a float {f}"),
//!         DataCell32bit::Char(c) => println!("a char {c:?}"),
//!         _ => (),
//!     }
//! }
//! ```
//!
//! See the [`customize.rs`](https://github.com/andamira/ladata/blob/main/examples/customize.rs)
//! example on how to use custom data types.
//!
//!
//! [`DataTypes`]: DataTypes
//! [`DataTypesCopy`]: DataTypesCopy
//! [`DataCells`]: DataCells
//! [`DataCellsCopy`]: DataCellsCopy
//! [`DataBares`]: DataBares
//! [`DataBaresCopy`]: DataBaresCopy

/// Data *Bares* (just the unsafe bare data).
pub mod bares {
    super::macros::reexport![mod_bares, crate::unit::build; all_sizes];
}

/// Data *Cells* (the encapsulation of *data* plus *type*).
pub mod cells {
    super::macros::reexport![mod_cells, crate::unit::build; all_sizes];
}

/// Data *Types* (just the *type* of the data).
pub mod types {
    super::macros::reexport![mod_types, crate::unit::build; all_sizes];
}

/// Every unitary type, organized by size.
pub mod size {
    super::macros::reexport![mod_size, crate::unit::build; all_sizes];
}

pub mod traits;

#[doc(inline)]
pub use {bares::*, cells::*, traits::*, types::*};

mod build;
mod macros;
mod nodata;

#[cfg(test)]
mod tests;
