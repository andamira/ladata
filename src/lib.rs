//
//! **`ladata`**
//!
//! A simple model for working with heterogeneous data in Rust.
//!
//! # Summary
//!
//! It's based on 2 basic abstractions: `DataType` for dealing with data types,
//! and `DataCell` for also dealing with concrete data.
//!
//! Each one have multiple concrete implementation that allows choosing the right
//! size and constraints.
//!
//! # Diagram of type name morphology
//!
//! The concrete type builder follows a constructive naming scheme that
//! facilitates identifying the type with the right combination of constraints.
//!
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
//! 5. `[With]` allows to extend the type with a custom implementation of
//!     [`DataTypes`]|[`DataCells`] via the `With` variant.
//!
//!
//! ## Longer explanation
//!
//! ### `<Type|Cell>`
//!
//! `Type` is a categorization of data types.
//!
//! All *types* must implement the [`DataTypes`] trait, and `Copy` *types* must
//! additionally implement the [`DataTypesCopy`] trait.
//!
//! `Cell` is an encapsulation of data together with static type information.
//!
//! All *cells* must implement the [`DataCells`] trait, and `Copy` *cells* must
//! additionally implement the [`DataCellsCopy`] trait.
//!
//! ### `[Copy]`
//!
//! **`Data*Copy`**`*` means the data represented by the *type*,
//! (and|or encapsulated by the *cell*) is [`Copy`].
//! And therefore it automatically derives `Copy`.
//!
//! Only types that can be copied with simple shallow bit-for-bit copy,
//! leaving the source initialized, can be `Copy`.
//! That leaves out types referencing the heap and other resources.
//!
//! Fundamental Rust's `Copy` types:
//! [`i8`], [`i16`], [`i32`], [`i64`], [`i128`]
//! [`u8`], [`u16`], [`u32`], [`u64`], [`u128`]
//! [`usize`], [`isize`], [`f32`], [`f64`], [`char`], [`bool`],
//! [`[T: Copy; const N: usize]`][array]…
//!
//! Fundamental Rust's non-Copy types:
//! [`String`], [`Vec`],
//! [`Rc`][std::rc::Rc], [`Arc`][std::sync::Arc]… TODO
//!
//! ### *`Size`*
//!
//! Reprents a size constrain for the data representation,
//!
//! Specifically indicates the maximum size, therefore smaller-sized variants
//! are available also in bigger-sized cells.
//!
//! Specific sizes have implemented size-marker traits:
//! - DataSize8b, DataSize16b, DataSize32b, DataSize64b, DataSize128b,
//!   DataSize256b, DataSize512b, DataSize1024b.
//!
//! The concrete type have convenience bytes type aliases defined
//! so you can choose a size in bits or bytes, but they refer to the same object,
//! and the associated size-trait is expressed in bits.
//!
//!
//! Specific From<> conversions are automatically implemented between
//! same-sized variants in differently sized `DataCell*`s.
//!
//!
//! ### `[With]`
//!
//! The fourth and last part of the name.
//!
//! **`Data*With`** enums can be extended generically by a type implementing
//! Data[[`Types`]|[`Cells`]] stored in its `With` variant.
//!
//! The non-With version is a type alias of the With one, that uses [`NoData`]
//! for the `With` variant.
//!
//! [`Types`]: DataTypes
//! [`Cells`]: DataCells
//!
//! # External types
//!
//! external crates are used to fill several gaps :
//!
//! - Floats
//! - Decimals
//! - 8, 16 & 128 bit sized floats.
//! - Strings
//!   - sixbit: small static strings (`sixbit`)
//!   - vector and strings using a 32bit pointer size in 64bit machine.
//!   - vector and strings using a single usize (by inlining capacity & len)
//! - BitFields
//!   - `…`
//! - BigNumbers
//!   -
#![allow(non_snake_case)]

#[cfg(test)]
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
}

/// Re-export of data *cells* & *types* of specific sizes.
pub mod size {
    crate::reexport![mod_size super::builder; all_sizes];
}
