//! A simple & modular mixed data model for stable Rust.
//!
//! # Overview
//!
//! ## Units
//!
//! See the [`units`] module for more information.
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
//! - the possibility of embedding a custom type in the `With` variant.
//!
//! For example, these are concrete parallel implementations:
//! - [`DataType2ByteCopy`][all::DataType2ByteCopy] represents a 2-byte sized
//!   `Copy` type (== 1 byte).
//! - [`DataCell2ByteCopy`][all::DataCell2ByteWith] represents the corresponding
//!   2-Byte sized type + data (== 4 bytes).
//! - [`DataBare2ByteCopy`][all::DataBare2ByteCopy] represents the
//!   corresponding 2-Byte sized data without the type (== 2 bytes).
//! - [`DataType2ByteCopyWith`][all::DataCell2ByteCopyWith] represents the
//!   corresponding 2-Byte sized type, with a custom type embedded (>= 2 bytes).
//! - [`DataCell2ByteCopyWith`][all::DataCell2ByteCopyWith] represents the
//!   corresponding 2-Byte sized type + data, with a custom type embedded
//!   (>= 4 bytes).
//!
//! Note that `DataBare…`s are more space-efficient than `DataCell…`s but
//! not as convenient to use and also unsafe to read, because they're unions.
//! They are mostly intended to be used from collections that can store their
//! corresponding `DataType…`s separately. At the moment they only support
//! `Copy` types, and they can't host any custom types (lacking a `With` field).
//!
//! ## Lines
//!
//! See the [`lines`] module for more information.
//!
//! The fundamental linear abstractions are:
//! - `DataLine…` structs, based on an array `[DataCell*; const usize]`.
//! - `DataLineGrow…` structs, growable, based on a vector `Vec<DataCell*>`.
//! - `DataLineCompact*…` structs, uses `DataBare*` plus `DataType*`.
//!
//! ## External dependencies
//!
//! The standard library is enabled by default. You can remove it independently
//! of other features, by not including the "std" feature.
//!
//! The default features provide a curated selection of external types than can
//! complement the standard Rust types.
//!
//! ### Customizing features
//!
//! * Only `no_std`, without any dependencies:
//!   ```shell
//!   $ cargo build --no-default-features
//!   ```
//!
//! * only the standard library:
//!   ```shell
//!   $ cargo build --no-default-features --features=std
//!   ```
//!
//! * all the dependencies (See the full list in
//! [`Cargo.toml`](https://github.com/andamira/ladata/blob/main/Cargo.toml)):
//!   ```shell
//!   $ cargo build --features=deps_all
//!   ```
//!
//! # Basic usage
//! ```
//! # use ladata::all::*;
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
//! # Status
//! - This is still experimental work in progress. The API can and will keep changing.
//! - Currently waiting for [GAT stabilization] to keep improving the traits.
//!
//! [Gat stabilization]: https://github.com/rust-lang/rust/issues/44265
//!
#![allow(non_snake_case, non_camel_case_types)]
//
#![cfg_attr(not(feature = "std"), no_std)]

mod builder;
use builder::built;

#[cfg(test)]
#[allow(unused_imports)]
mod tests;

pub(crate) mod traits;
#[doc(inline)]
pub use traits::*;

pub mod other;

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use super::other::*;
    #[doc(inline)]
    pub use super::traits::*;
    #[doc(inline)]
    pub use crate::built::*;
}
/// Everything is available in here, organized by size.
pub mod sizes {
    crate::reexport![mod_sizes, crate::built; all_sizes];
}

/// Units of data, whether just the data, the type, or both.
/// # Unitary types: Data `Type*`|`Cell*`|`Bare*` and `NoData`
///
/// The concrete implementations of **unitary types** observes the following naming schemes:
/// ```txt
/// *) No Data                                                           Legend      |     Sizes
/// *)    Data <Type|Cell> <Size> [Copy] [With]                      --------------- | ------------
/// *)    Data <Bare>    <Size> <Copy>                               <> : required   |    8b =   1B
///                                                                  [] : optional   |   16b =   2B
///                                                                   | : either or  |   32b =   4B
///                                                                   = : alias      |   64b =   8B
///                                                                                  |  128b =  16B
///                                                                                  |  256b =  32B
///                                                                                  |  512b =  64B
///                                                                                  | 1024b = 128B
/// ```
/// 0. `No`: a special prefix for the [`NoData`][all::NoData] type.
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
/// Indicates the specific size of the data representation in memory. Can be
/// written using either bytes (**`N`**`Bytes`) or bits (**`N`**`bits`).
///
/// Specifically tells the maximum size of the data. Smaller-sized variants
/// are also available in bigger-sized cells. For example the `U16(u16)` variant
/// is present in `DataCell2Byte` and `DataCell4Byte` but not in `DataCell1Byte`.
///
/// There are also convenience aliases in bit sizes to byte sizes. E.g.:
/// [`DataType32bit`][all::DataType32bit] == [`DataType4Byte`][all::DataType4Byte]
///
/// Types can be found classified by size in the [`sizes`] module.
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
/// using the the zero-sized [`NoData`][all::NoData] type. E.g.:
/// [`DataType32Byte`][all::DataType32Byte]
pub mod units {
    /// Data *Bares* (just the unsafe bare data).
    pub mod bares {
        crate::reexport![mod_bares, crate::built; all_sizes];
    }

    /// Data *Cells* (the encapsulation of *data* plus *type*).
    pub mod cells {
        crate::reexport![mod_cells, crate::built; all_sizes];
    }

    /// Data *Types* (just the type of the data).
    pub mod types {
        crate::reexport![mod_types, crate::built; all_sizes];
    }

    #[doc(inline)]
    pub use {bares::*, cells::*, types::*};
}

/// Data *Lines* (sequences of *cells*).
///
/// # Linear types: Data `Line*`
///
/// Linear (AKA sequential) data structures, leverages Rust arrays and vecs to
/// contain sequences of unitary types.
///
/// The concrete implementations of **linear types** observes the following naming schema:
/// ```txt
/// -) Data Line [Grow] [Dense|Buffer] <Size> [Copy] [With]            Legend       |     Sizes
///                                                                 --------------- | -------------
///                                                                 <> : required   |   1B  =    8b
///                                                                 [] : optional   |   2B  =   16b
///                                                                  | : either or  |   4B  =   32b
///                                                                  = : alias      |   8B  =   64b
///                                                                                 |  16B  =  128b
///                                                                                 |  32B  =  256b
///                                                                                 |  64B  =  512b
///                                                                                 | 128B  = 1024b
/// ```
/// 1. `Data`: the *pivotal core*.
/// 2. `Line`: a sequence of `DataCell`s.
/// 3. `[Dense|Buffer]`: internally stores the cells as a sequence of `DataType`s
///     plus a sequence of 1) `DataBare`s or 2) Byte slices.
/// 4. `<Size>`: confines the maximum size of each collected cell.
/// 5. `[Copy]`: indicates that all the included data types are `Copy`.
/// 6. `[With]`: allows to embed a custom implementation of a data type or cell
///     in its `With` variant.
///
pub mod lines {
    crate::reexport![mod_lines, crate::built; all_sizes];
}
