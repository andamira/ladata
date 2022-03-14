//! **`ladata`**
//!
//! A versatile data model for working with mixed data in a modular way.
//!
//! # Status
//!
//! Experimental.
//!
//! # Overview
//!
//! - Works in stable Rust and `no_std`.
//! - Supports mixed/heterogeneous data.
//! - Supports custom data types and features customization.
//! - Is designed to be modular and composable like building blocks.
//! - Is lightweight and simple enough.
//!
//! The fundamental interconnected abstractions of this library are:
//! - `DataType…` enums, which only contains type information.
//! - `DataCell…` enums, which contains both data and type information.
//! - `DataUnsafeCell…` unions, which only contains data.
//!
//! Each one has multiple concrete implementations that differ by:
//! - the maximum **`size`** of the referred data.
//! - the presence or lack of `Copy` semantics.
//! - the possibility of embedding a custom type in the `With` variant.
//!
//! For example, these are three inter-related implementations:
//! - [`DataType2ByteCopy`][all::DataType2ByteCopy] represents a 2-byte sized
//!   `Copy` type (occupies 1 byte).
//! - [`DataCell2ByteCopy`][all::DataCell2Byte] represents the corresponding
//!   2-Byte sized data, and type (occupies 4 bytes).
//! - [`DataUnsafeCell2ByteCopy`][all::DataUnsafeCell2ByteCopy] represents the
//!   corresponding 2-Byte sized data without the type (occupies 2 bytes).
//!
//! Note that `DataUnsafeCell…`s are more space-efficient than `DataCell…`s but
//! not as convenient to use and, because they're unions, also unsafe to read.
//! They are mostly intended to be used from collections that can store their
//! corresponding `DataType…`s separately. At the moment they only support
//! `Copy` types, and they can't include custom types (lacking a `With` field).
//!
//! ## External dependencies
//!
//! This library enables by default the standard library. Removing the "std"
//! feature makes the library completely `no_std`, independently of any other
//! feature enabled.
//!
//! The default features are selected with the intention of providing a well
//! balanced default experience. Bringing all the new types of data other than
//! the fundamental Rust types.
//!
//! ### Examples of features customizations
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
//! use ladata::all::*;
//!
//! let arr = [
//!     DataCell32bit::F32(3.14),
//!     DataCell32bit::Char('π'),
//!     DataCell32bit::I16(-314),
//!     DataCell32bit::ByteArray4([3, 141, 59, 26]),
//! ];
//!
//! for c in arr {
//!    match c {
//!         DataCell32bit::F32(f) => println!("{f}"),
//!         DataCell32bit::Char(c) => println!("{c:?}"),
//!         DataCell32bit::I16(i) => println!("{i:?}"),
//!         DataCell32bit::ByteArray4(ba) => println!("{ba:?}"),
//!         _ => (),
//!     }
//! }
//!
//! ```
//!
//! See the [`customize.rs`](https://github.com/andamira/ladata/blob/main/examples/customize.rs)
//! example on how to use custom data types.
//!
//! # How to choose *Types* and *Cells*
//!
//! The concrete implementations observes the following naming schemes:
//! ```txt
//! *)    Data <Type|Cell>    <Size>     [Copy] [With]
//!                              ↑                             ·---------------·
//!                       |   8b =   1B|                       |<> : required  |
//!                       |  16b =   2B|                       |[] : optional  |
//!                       |  32b =   4B|                       | | : either or |
//!                       |  64b =   8B|                       | = : alias     |
//!                       | 128b =  16B|                       ·---------------·
//!                       | 256b =  32B|
//!                       | 512b =  64B|
//!                       |1024b = 128B|
//!                              ↓
//! *)    Data <UnsafeCell>   <Size>     <Copy>
//! *) No Data
//! ```
//! 0. `No`: a special prefix for the [`NoData`][all::NoData] type.
//! 1. `Data`: everything revolves around this concept.
//! 2. `<Type|Cell|UnsafeCell>`: encapsulates either just the data type,
//!    both the data type and the data, or just the data.
//! 3. `<Size>`: constrains the maximum size of the data represented by the type,
//!    either in bits or Bytes (in powers of 2).
//! 4. `[Copy]`: indicates that all the data types included are `Copy`.
//! 5. `[With]`: allows to extend the data type or cell with a custom
//!     implementation in the `With` variant.
//!
//! ### `<Type|Cell|UnsafeCell>`
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
//! All *`DataUnsafeCell`s* implements the (marker) [`DataUnsafeCells`] trait.
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
//! using the the zero-sized [`NoData`][all::NoData] type. E.g.:
//! [`DataType32Byte`][all::DataType32Byte]
//!
//! # Design
//!
//! Some of the concerns when designing the library were:
//!
//! - To figure out a simple enough and viable solution for working with
//!   heterogeneous data in stable Rust.
//!
//! - To fill the size gaps in the standard type system, providing more choice
//!   of data types, specially in smaller memory sizes.
//!
//! - To decouple the data type from the data representation and from the data
//!   structures, enabling them to be easily used as modular building blocks
//!   for more complex data structures.
//!
//! - To remain as lightweight and customizable as possible.
//!
//!
//! # Planned features
//!
//! - [ ] More automatic trait implementations.
//! - [ ] More abstract data types.
//! - [ ] More examples.

#![allow(non_snake_case, non_camel_case_types)]
//
#![cfg_attr(not(feature = "std"), no_std)]

mod builder;

#[cfg(test)]
#[allow(unused_imports)]
mod tests;

pub(crate) mod traits;
#[doc(inline)]
pub use traits::*;

pub mod special;
#[doc(inline)]
pub use special::NoData;

/// Everything is available here.
pub mod all {
    #[doc(inline)]
    pub use super::builder::*;
    #[doc(inline)]
    pub use super::special::*;
    #[doc(inline)]
    pub use super::traits::*;
}

/// Data *cells* & *types* classified by size.
pub mod size {
    crate::reexport![mod_size super::builder; all_sizes];
}
