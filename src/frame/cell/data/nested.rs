// src/frame/cell/data/nested
//
//!
//!
//

use crate::frame::handle::{Handle128, Handle16, Handle32, Handle64, Handle8};

/// A nested representation of cell data (40 bytes).
#[derive(Clone, Debug, PartialEq)]
pub enum CellDataNested {
    Categorical(CategoricalData),
    Numerical(NumericalData),
}

/// Categorical data (32 bytes).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum CategoricalData {
    /// Boolean value.
    Bool(bool),

    // /// String value.
    // String(String),

    // /// A collection of bytes.
    // Bytes(Vec<u8>),
    /// A handle ID for relational operations.
    Id(IdData),
}

/// Identifier data (24 bytes).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)] // MAYBE Hash, Copy
pub enum IdData {
    // /// A UUID.
    // Uuid(Uuid),
    Handle8(Handle8),

    Handle16(Handle16),

    Handle32(Handle32),

    Handle64(Handle64),

    Handle128(Handle128),
}

/// Numerical data (32 bytes).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum NumericalData {
    /// An integer number.
    Discrete(DiscreteData),

    /// A floating-point number.
    Continuous(ContinuousData),
}

/// Discrete data (24 bytes)
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum DiscreteData {
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

/// Continuous data (16 bytes)
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum ContinuousData {
    // F16, // Half
    F32(f32),
    F64(f64),
    // F128, // TwoFloat
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn size() {
        assert_eq![40, size_of::<CellDataNested>()];
        assert_eq![32, size_of::<CategoricalData>()];
        assert_eq![24, size_of::<IdData>()];
        assert_eq![32, size_of::<NumericalData>()];
        assert_eq![24, size_of::<DiscreteData>()];
        assert_eq![16, size_of::<ContinuousData>()];
    }
}
