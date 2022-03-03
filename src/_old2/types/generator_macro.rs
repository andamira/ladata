// ladata::types::generator_macro
//
//!
//
// DESIGN:
// 
// Call a single function, with a list of variants.
//
// receive a list of tokens, separated by sections,
//
// - the common ones
//   - by size? generated?
// 

use super::DataTypes;

///
macro_rules! create_data_types_and_cells {
    () => {
        // pub enum DataType {
        // }

        // pub enum DataTypePlus<T: DataTypes> {
        pub enum DataType<T: DataTypes> {
            None,
            Other(T),
        }

    #[derive(Clone, Copy, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum DataTypePlus<T: DataTypes> {
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


        // CELL

        // pub enum DataCell<C: DataCellAble> {
        // }

    };
    (type_var;) => {
    };

    // (cell_var;) => {
    // };
}

// macro_rules! create_type_variant {
//     ($name:ty) => {
//     };
// }
//
// create_data_types_and_cells!{
// };


// ($inner:ident, $tname:tt, $type:ty, $rhs:ty, $($bound:tt)* ) => {
// crate::impl_ops![N, Natural, Natural<N>, Self, NumInt + Unsigned + Clone];
