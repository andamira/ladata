//! **`ladata`**
//!
//! A simple model for working with heterogeneous data.
//!
//! # Overview
//!
//! The main abstractions are:
//! - `DataType…` enums, which only contains type information.
//! - `DataCell…` enums, which contains both data and type information.
//! - `DataUnsafeCell…` unions, which only contains the data.
//!
//! Each one has multiple concrete implementations differing both in the
//! data size and its `Copy` semantics.
//!
//! For example, these are three inter-related implementations:
//! - [`DataType2ByteCopy`][all::DataType2ByteCopy] represents a 2-byte sized
//!   `Copy` type (occupies 1 byte).
//! - [`DataCell2ByteCopy`][all::DataCell2Byte] represents the corresponding
//!   2-Byte sized data, and type (occupies 4 bytes).
//! - [`DataUnsafeCell2ByteCopy`][all::DataUnsafeCell2ByteCopy] represents the
//!   corresponding 2-Byte sized data without the type (occupies 2 bytes).
//!
//! `DataUnsafeCell…`s are more space-efficient than `DataCell…`s but not as
//! convenient to use, and they are unsafe to read. They are mostly intended
//! to be used from collections that store their corresponding `DataType…`s
//! separately. At the moment they only support `Copy` types, and they can't
//! include custom types either (they lack a `With` field).
//!
//! # Naming scheme
//!
//! The concrete type implementations employs the following naming scheme:
//! ```txt
//! Data <Cell|Type>    <Size>    [Copy] [With]
//!                 |   8b =   1B|
//!                 |  16b =   2B|
//!                 |  32b =   4B|
//!                 |  64b =   8B|                 ·---------------·
//!                 | 128b =  16B|                 |<> : required  |
//!                 | 256b =  32B|                 |[] : optional  |
//!                 | 512b =  64B|                 | | : either or |
//!                 |1024b = 128B|                 | = : alias     |
//! ```
//!
//! Special case: `DataUnsafeCell <Size:bits|Bytes> <Copy>`
//!
//! 1. `Data`
//! 2. `<Cell|Type>` memory data plus type, or only the type information.
//!    of the default flat representation.
//! 3. `<Size>` constrains the maximum size of the data in memory (in bits or Bytes)
//! 4. `[Copy]` indicates that all the types included are `Copy`.
//! 5. `[With]` allows to extend the type with a custom implementation
//!     in the `With` variant.
//!
//! ## `<Cell|Type|UnsafeCell>`
//!
//! - `Type` indicates just the categorization of data types.
//! All *`DataType*`s* must implement the [`DataTypes`] trait, and
//! *`DataType*Copy*`* *types* must additionally implement the
//! [`DataTypesCopy`] trait.
//!
//! - `Cell` indicates the encapsulation of data together with type information.
//! All *`DataCell*`s* must implement the [`DataCells`] trait, and
//! *`DataCell*Copy*`* *cells* must additionally implement the
//! [`DataCellsCopy`] trait.
//!
//! - `UnsafeCell` indicates the encapsulation of data without type information.
//! All *`DataUnsafeCell`s* implement the [`DataUnsafeCells`] trait.
//!
//! ## `[Copy]`
//!
//! *`Copy`* indicates that the data represented by the *type*,
//! (and|or encapsulated by the *cell*) is [`Copy`].
//!
//! Only types that can be copied with simple shallow bit-for-bit copy,
//! leaving the source initialized, can be `Copy`.
//! This leaves out types referencing the heap and other resources.
//!
//! ## *`<Size>`*
//!
//! Indicates the specific size of the data representation in memory. Can be
//! written using either bytes (**`N`**`Bytes`) or bits (**`N`**`bits`).
//!
//! Specifically tells the maximum size of the data. Smaller-sized variants
//! are also available in bigger-sized cells. For example the `U16(u16)` variant
//! is present in `DataCell2Byte` and `DataCell4Byte` but not in `DataCell1Byte`.
//!
//! There are also convenience aliases in bit sizes to byte sizes. E.g.:
//! [`DataType32bit`][all::DataType32bit] == [`DataType4Byte`][all::DataType4Byte]
//!
//! Types can be found classified by size in the [`size`] module.
//!
//! ## `[With]`
//!
//! **`DataType*With`** enums can be extended generically by storing a type
//! implementing [`DataTypes`] in its `With` variant (or [`DataTypesCopy`]
//! in the case of `DataType*CopyWith`.
//!
//! In the same way, **`DataCell*With`** enums can be extended generically by
//! storing a type implementing [`DataCells`] in its `With` variant
//! (or [`DataCellsCopy`] in the case of `DataCell*CopyWith`.
//!
//! Internally, non-`With` versions are aliases to the corresponding `With`
//! version with the zero-sized [`NoData`] type.

#![allow(non_snake_case, non_camel_case_types)]

#[cfg(test)]
#[allow(unused_imports)]
mod tests;

pub(crate) mod nodata;
pub(crate) mod traits;

mod builder;

#[doc(inline)]
pub use nodata::NoData;
#[doc(inline)]
pub use traits::{DataCells, DataCellsCopy, DataTypes, DataTypesCopy, DataUnsafeCells};

/// Every type and trait is re-exported here.
pub mod all {
    #[doc(inline)]
    pub use super::builder::*;
    pub use super::traits::*;
    pub use super::nodata::*;
}

/// Re-export of data *cells* & *types* of specific sizes.
pub mod size {
    crate::reexport![mod_size super::builder; all_sizes];
}
