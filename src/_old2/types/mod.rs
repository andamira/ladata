// ladata::types
// pub
//
//! Semantic categorization of individual data types.
//!
//
// TODO: symmetric DataType & DataCell paralell implementations,
// - DECIDE whether to nest sizes… or make it flat?
//   - matching should be convenient…
//
// 
//
// sub-implementations for sized versions
//
// MAYBE:
// - create a macro to avoid DRY for impl over:
//   - DataType
//   - NestedDataType
//   - CopyDataType
//   - NonCopyDataType
//   - …
//
// METHODS:
//   - into_data_type(self) 
//   - to_data_type(&self) (Copy)
//
//  NonCopyDataType (to require only this)
//  Convert into this from with a method that returns the new struct :)
//
//
// RETHINK:
// - multiple generic data types implementation…
//   - FIXED! do example with nested custom types
//   - create implementations for external data types :)
//
// - NAMES:
//   - A: trait DataTypes, enum DataType<T>,
//   - B: trait DataType, enum DataTypeCell
//
// - NoData vs NoDataType


mod api;
pub use api::DataTypes;

// MAYBE?
// mod generator_macro;
// pub use generator_macro::DataType; // TEMP

// pub use macro_generator::{DataType, DataType8, DataType16, DataType32, DataType64, DataType128, DataTypeNested};
// pub(crate) use macro_generator::{DataCell, DataCell8, DataCell16, DataCell32, DataCell64, DataCell128};

// mod variants;
// pub use variants::{DataType};

/*
 _SACAR

/// A flat enumeration of supported data types.
///
/// Data types are classified first semantically, and then by size.
///
/// From a *compsci* perspective they can be separated between Copy and non Copy:
///
/// The Copy types are:
/// - **Bool**[`n`](#legend) : Boolean value. (WIP)
/// - **BitField**[`N`](#legend) : bit fields. (WIP)
/// - **U**[`N`](#legend) : unsigned integer.
/// - **I**[`N`](#legend) : signed integer.
/// - **F**[`N`](#legend) : float.
/// - **Rational**[`N`](#legend) : rational number.
/// - **Date** : date.
/// - **Time** : time.
/// - **DateTime** : date and time.
/// - **Duration** : duration of time.
///
/// The non-Copy types are:
/// - **String** : A unicode utf-8 string.
/// - **Bytes** : A buffer of bytes.
/// - **SparseBitfield** : A buffer of bits. RETHINK: Bits (AKA SparseBitField)
/// - **BigInt** : Bit integer number.
/// - **BigDec** : Big decimal number.
///
/// Special data types are:
/// - **None** : Means there's no data type.
/// - **Other** : A custom data type, or the root of a tree of custom data types.
///   They can be Copy or not, depending on implementation.
///
/// ### Legend
/// * **[`n`](#types)** a [1, 2, 3, 4, 5, 6 or 7] suffix for the size in bits.
/// * **[`N`](#Types)** an [8, 16, 32, 64 or 128] suffix for the size in bits.
///
///
/// From a databases perspective they can be separated into *Categorical*
/// and *Numerical*. The [`NestedDataType`] structure follows this scheme.
///
/// # Custom data types
///
/// * If you don't need to integrate any custom data types you can use directly
/// the [`StandardDataType`] alias.
///
/// * By implementing the [`DataTypes`] trait on your custom data type
/// it can be used through the [`DataType::Other`] variant.
///
/// * To have multiple custom data types they have to be part of a single
/// enumeration implementing `DataTypes`.
///
/// ## Examples
///
// ## DESIGN
//
// - copy vs clone OK
// - MAYBE?:
//   - static vs dynamic
//   - immutable vs mutable
//   - fixed-size vs growable
*/

///
pub type DataType = DataTypePlus<NoDataType>;

/// A zero-sized struct that represents the absence of a data type.
///
/// Semantically equivalent to [`DataType::None`].
///
// Has several type aliases for specifying intentionality:
// - [`WithoutCustomDataTypes`]
///
/// facilitates using [`DataType`]
/// without any custom data types. See [`StandardDataType`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoDataType;

impl DataTypes for NoDataType {
    #[inline]
    fn is_copy(&self) ->  bool { true }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum DataTypePlus<T: DataTypes> { // IDEA: DataTypeGeneric
    // Copy Types (see CopyDataType)
    // ----------

    ///
    Bool,

    /// Two booleans.
    Bool2,

    /// Three booleans.
    Bool3,

    /// Four booleans.
    Bool4,

    ///
    Bool5,

    ///
    Bool6,

    ///
    Bool7,

    // BitFields

    ///
    BitArray8,
    ///
    BitArray6,
    ///
    BitArray32,
    ///
    BitArray64,
    ///
    BitArray128,

    // Discrete numbers

    ///
    I8,
    ///
    U8,
    ///
    I16,
    ///
    U16,
    ///
    I32,
    ///
    U32,
    ///
    I64,
    ///
    U64,
    ///
    I128,
    ///
    U128,

    // Continuous numbers

    /// Floating point 8bit.
    F8,
    ///
    F16,
    ///
    F32,
    ///
    F64,
    ///
    F128,

    /// Fixed decimal point 8 bit.
    Decimal8,
    Decimal16,
    Decimal32,
    Decimal33,
    Decimal64,
    Decimal128,

    // rational
    // TODO: CHECK precision when converting to percentage:
    //  - 100 / 256 = 0.390625 % minimal resolution with 8bit... but
    //    how that would change if considering 4b / 4b ? 0-127 / 1-127 ?
    //      - 0 for denominator means a negative decimal representation
    //        https://mathmaine.com/2014/07/31/negative-fractions/ :)
    //
    // Rational 8, // (4/4)
    ///
    Rational16,  // (8/8)
    ///
    Rational32,  // (16/16)
    ///
    Rational64,  // (32/32)
    ///
    Rational128, // (64/64)

    ///
    Date,
    ///
    Time,
    ///
    DateTime, // or TimeStamp?
    ///
    Duration,

    // RETHINK: Money type?
    // Money,

    /// A unicode codepoint.
    Char,

    // NonCopy Types (see NonCopyDataType)
    // -------------

    /// A unicode string.
    String, // structure

    /// A binary string.
    Binary,

    ///
    SparseBitfield,

    // `big_num` crate
    ///
    BigInt,
    ///
    BigDec,

    // Media, // Text, Image, Audio, Video… // TODO: Custom? or

    /// Intended for custom data types that implements [`DataTypes`].
    Other(T),

    /// A type that represents the absence of data type.
    ///
    /// `Copy:true, Size:0, Align:0`
    None,
}

