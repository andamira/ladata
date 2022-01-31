// src/dataframe/cell/data
//!

use uuid::Uuid;

mod nested;
pub use nested::{
    CategoricalData, CellDataNested, ContinuousData, DiscreteData, IdData, NumericalData,
};

use crate::dataframe::handle::{Handle128, Handle16, Handle32, Handle64, Handle8};

/// A flat representation of cell data (32 bytes).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum CellData {
    // Categorical
    // -----------
    /// Boolean value.
    Bool(bool),
    /// String value.
    String(String),
    /// A collection of bytes.
    Bytes(Vec<u8>),

    // Id
    /// UUID
    Uuid(Uuid),
    // A handle ID for relational operations.
    Handle8(Handle8),
    Handle16(Handle16),
    Handle32(Handle32),
    Handle64(Handle64),
    Handle128(Handle128),

    // Numerical
    // ---------

    // Continuous
    F32(f32),
    F64(f64),

    // Discrete
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn size() {
        assert_eq![32, size_of::<CellData>()];
    }
}
