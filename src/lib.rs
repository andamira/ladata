//! A simple & modular mixed data model.
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
//! - [`DataType16bitCopy`][all::DataType16bitCopy] represents a 2-byte sized
//!   `Copy` type (== 1 byte).
//! - [`DataCell16bitCopy`][all::DataCell16bitWith] represents the corresponding
//!   2-Byte sized type + data (== 4 bytes).
//! - [`DataBare16bitCopy`][all::DataBare16bitCopy] represents the
//!   corresponding 2-Byte sized data without the type (== 2 bytes).
//! - [`DataType16bitCopyWith`][all::DataCell16bitCopyWith] represents the
//!   corresponding 2-Byte sized type, with a custom type embedded (>= 2 bytes).
//! - [`DataCell16bitCopyWith`][all::DataCell16bitCopyWith] represents the
//!   corresponding 2-Byte sized type + data, with a custom type embedded
//!   (>= 4 bytes).
//!
//! Note that `DataBare…`s are more space-efficient than `DataCell…`s but
//! not as convenient to use and also unsafe to read, because they're unions.
//! They are mostly intended to be used from collections that can store their
//! corresponding `DataType…`s separately. At the moment they only support
//! `Copy` types, and they can't host any custom types (lacking a `With` field).
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
//! [`DataCells`]: units::DataCells
//! [`DataTypes`]: units::DataTypes
//! [`DataBares`]: units::DataBares

#![allow(non_snake_case, non_camel_case_types)]
//
#![cfg_attr(not(feature = "std"), no_std)]

pub mod storage;
pub mod structs;
pub mod units;

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use super::storage::*;
    #[doc(inline)]
    pub use super::structs::{bits::*, list::*};
    #[doc(inline)]
    pub use crate::units::{bares::*, cells::*, traits::*, types::*};
}
