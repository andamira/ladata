// ladata::unit
//
//! Unitary types.
//!
//! The fundamental unitary abstractions are:
//! - `DataUnit…` enums that contain both the data, and the type of the data.
//! - `DataType…` enums that only contain the type of data, not the data itself.
//! - `RawData…` unions that only contain the raw data, not the type of the data.
//!
//! They implement the [`DataUnit`], [`DataType`] and [`RawData`] traits.
//!
//! Each one has many concrete implementations differentiated by:
//! - the maximum `size` of the represented data.
//! - whether all the included types of data are `Copy`.
//! - the possibility of embedding a custom type in the `With` variant./!
//!
//! They observe the following naming scheme:
//! ```txt
//!                                         Legend      |     Sizes
//! *) Unit <Size> [Copy] [With]        --------------- | ------------
//! *) Type <Size> [Copy] [With]        <> : required   |    8b =   1B
//! *) Raw  <Size> <Copy>               [] : optional   |   16b =   2B
//!                                      | : either or  |   32b =   4B
//!                                      = : alias      |   64b =   8B
//!                                                     |  128b =  16B
//!                                                     |  256b =  32B
//!                                                     |  512b =  64B
//!                                                     | 1024b = 128B
//! ```
//! 1. `<Unit|Type|Raw>`: encapsulates 1) only the data type,
//!    2) both the data type and the data, or 3) only the data, respectively.
//! 2. `<Size>`: confines the maximum size of the represented data,
//!    limiting the number of types and sizes of data available.
//! 3. `[Copy]`: indicates that all the included data types are `Copy`.
//! 4. `[With]`: allows to embed a custom implementation of a data unit or type
//!     in its `With` variant.
//!
//! - `Type` indicates the categorization of the data type information.
//! All *`DataType*`s* must implement the [`DataType`] trait, and
//! *`DataType*Copy*`* *types* must additionally implement the
//! [`DataTypeCopy`] trait.
//!
//! - `Unit` indicates the unitary combination of the raw data and the data type.
//! All *`DataUnit*`s* must implement the [`DataUnit`] trait, and
//! *`DataUnit*Copy*`* *units* must additionally implement the
//! [`DataUnitCopy`] trait.
//!
//! - `Raw` indicates the encapsulation of data without the type information.
//! All *`RawData`s* implements the (marker) [`RawData`] trait.
//!
//! ### `[Copy]`
//!
//! *`Copy`* indicates that the data represented by the *type*,
//! (and|or encapsulated by the *unit*) is [`Copy`].
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
//! are also available in bigger-sized units. For example the `U16(u16)` variant
//! is present in `DataUnit16bit` and `DataUnit32bit` but not in `DataUnit8bit`.
//!
//! Types can be found classified by size in the [`size`] module.
//!
//! ### `[With]`
//!
//! **`DataType*With`** enums can be extended generically by storing a type
//! implementing [`DataType`] in its `With` variant (or [`DataTypeCopy`]
//! in the case of `DataType*CopyWith`.
//!
//! In the same way, **`DataUnit*With`** enums can be extended generically by
//! storing a type implementing [`DataUnit`] in its `With` variant
//! (or [`DataUnitCopy`] in the case of `DataUnit*CopyWith`.
//!
//! Internally, all non-`With` versions are convenient type aliases to the
//! corresponding `With` version (having the same size and `Copy` semantics),
//! using the zero-sized [`()`] unit type. E.g.:
//! [`DataType256bit`][crate::all::DataType256bit]
//!
//! Note that `RawData…`s are more space-efficient than `DataUnit…`s but
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
//!     DataUnit32bit::F32(3.14),
//!     DataUnit32bit::Char('π'),
//! ];
//! for c in arr {
//!    match c {
//!         DataUnit32bit::F32(f) => println!("a float {f}"),
//!         DataUnit32bit::Char(c) => println!("a char {c:?}"),
//!         _ => (),
//!     }
//! }
//! ```
//!
//! See the [`customize.rs`](https://github.com/andamira/ladata/blob/main/examples/customize.rs)
//! example on how to use custom data types.
//!
//!
//! [`DataType`]: DataType
//! [`DataTypeCopy`]: DataTypeCopy
//! [`DataUnit`]: DataUnit
//! [`DataUnitCopy`]: DataUnitCopy
//! [`RawData`]: RawData
//! [`RawDataCopy`]: RawDataCopy

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{raw::*, traits::*, types::*, units::*};
}

/// *Raw* Data (only the unsafe *raw* data).
#[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
pub mod raw {
    super::macros::reexport![mod_raw, crate::unit::build; all_sizes];
}

/// Data *Type* (only the *type* of the data).
pub mod types {
    super::macros::reexport![mod_types, crate::unit::build; all_sizes];
}

/// Data *Units* (the unification of *data* and *type*).
pub mod units {
    super::macros::reexport![mod_units, crate::unit::build; all_sizes];
}

/// Every unitary type, organized by size.
pub mod sizes {
    super::macros::reexport![mod_size, crate::unit::build; all_sizes];
}

pub mod traits;

mod build;
mod macros;
mod nodata;

#[cfg(test)]
mod tests;
