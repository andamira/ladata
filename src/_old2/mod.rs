// ladata::lib

/*
//
// `ladata` is a model for data manipulation that aims to provide simple and
// powerful abstractions with a low cognitive burden.
//
//! `ladata` aims to be a sensible model for data manipulation that combines
//! simplicity, versatilty and composability.
//!
//! It models *data* as the interplay of three fundamental complementary aspects:
//! **types**, **structures** and **encodings**, that are woven together into
//! **concrete implementations**.
//!
//! ## Data Types
//!
//! This aspect is mostly concerned with the semantic categorization of
//! individual units of data, independently of its structural organization
//! or memory encoding.
//!
//! Types can be broadly categorized according to different perspectives:
//!
//! - *compsci*: `Copy` (`F64` `Bool`…) vs `non-Copy` types (`String`, `BigInt`…).
//! - *databases*: `Categorical` (`Bool`, `String`…) vs `Numerical` types
//!   (`I8`, `BigDec`…).
//!
//! See also:
//! - [`DataType`]: all the supported data types.
//! - [`StandardDataType`]: non-generic, non-customizable version of `DataType`.
//! - [`NestedDataType`]: nested enumeration from a *databases* perspective.
//! - [`CopyDataType`]: flat enumeration of only *Copy* types.
//! - [`NonCopyDataType`]: flat enumeration only *non-Copy* types.
//!
//! - [`DataTypeClass`]: essential classes of data types.
//! - [`DataTypeApi`]: 
//!
//! ## Data Structures
//!
//! This aspect is mostly concerned with providing interface abstractions over
//! collections of data types, independenly of the concrete types and encodings.
//!
//! Structures can be broadly divided into abstract interfaces (Data)
//! and concrete implementations.
//!
//!
//! - [`DataDimensional`], [`DataD1,`] [`DataD2,`] [`DataD3`]…
//! - [`DataTree`]
//! - [`DataGraph`]
//!
//! See also: [`DataStructureType`], [`DataStructureApi`].
//!
//! ## Data Encodings
//!
//! This aspect is mostly concerned with the specific format the data takes
//! inside the computer memory, independently of its semantic type or its
//! associated structural interface.
//!
//! ## Concrete implementations
//! 
//! Several higher level data structure implementations are provided:
//!
//! - [`DataFrame`] :
//! - [`DataTable`] :
//! - [`DataLog`]
//!
//! Additionally, there are concrete trait implementations for the Rust's
//! standard library collections.
//
*/

// pub mod concrete;
// #[doc(inline)]
// pub use concrete::{DataFrame, DataTable};

// pub mod encodings;
// #[doc(inline)]
// pub use encodings::{DataEncoding, DataEncodingApi, StandardDataEncoding};

pub mod structures;
// #[doc(inline)]
// pub use structures::{DataStructureClass, DataStructure, DataStructureApi, StandardDataStructure};

// data types
pub mod types;
// #[doc(inline)]
// pub use types::{DataType, DataTypeApi, StandardDataType};
